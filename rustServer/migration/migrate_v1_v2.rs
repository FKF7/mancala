use std::path::{Path, PathBuf};
use std::io::{Read, Write};
use std::thread::sleep;
use std::{fs, io, println};
use std::fs::OpenOptions;
use std::time::{Duration, Instant};

use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

const HINT_TYPE_MASK: u8 = 0xC0; // 1100 0000
const ORIGINAL_INVALID_HINT: u8 = 0xC0; // 1100 0000
const COMPLETE_HINT: u8 = 0x80; // 1000 0000
const NEW_INVALID_HINT: u8 = 0xFF; // 1100 0000

pub const BASE_DATA_DIR: &str = "data";
pub const MIGRATED_DATA_DIR: &str = "migrated_data";
pub const NEW_PATH_DATA_DIR: &str = "new_path_data";

pub struct DataMigrator;

impl DataMigrator {
    pub fn migrate_data() -> io::Result<()> {
        let stop_requested = Arc::new(AtomicBool::new(false));
        {
            let stop_requested = Arc::clone(&stop_requested);

            ctrlc::set_handler(move || {
                stop_requested.store(true, Ordering::Relaxed);
            }).expect("Couldn't set Ctrl-C handler");
        }



        let start_time = Instant::now();
        let (copies, migrations) = Self::visit_dir(Path::new(BASE_DATA_DIR), 0, 0, start_time, &stop_requested)?;
        println!("Processed {} / {} files in {:?}", migrations, copies, start_time.elapsed());
        Ok(())
    }

    fn visit_dir(dir: &Path, files_copied: u64, files_migrated: u64, start_time: Instant, stop_requested: &Arc<AtomicBool>) -> io::Result<(u64, u64)> { //copies, migrations
        let mut copies = files_copied;
        let mut migrations = files_migrated;
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                if stop_requested.load(Ordering::Relaxed) {
                    println!("Ctrl+C pressed");
                    sleep(Duration::from_millis(5000));
                    return Err(io::Error::new(io::ErrorKind::Interrupted, "Migration stopped by user"));
                }
                (copies, migrations) = Self::visit_dir(&path, copies, migrations, start_time, &stop_requested)?;
            } else {
                if Self::process_file(&path)? {
                    migrations += 1;
                }
                copies += 1;

                if copies % 10000 == 0 {
                    println!("Processed {} / {} files in {:?}", migrations, copies, start_time.elapsed());
                }
                if stop_requested.load(Ordering::Relaxed) {
                    println!("Ctrl+C pressed");
                    sleep(Duration::from_millis(5000));
                    return Err(io::Error::new(io::ErrorKind::Interrupted, format!("Migration interrupted by user at {} / {} files in {:?}", migrations, copies, start_time.elapsed())));
                }
            }
        }

        fs::remove_dir(dir)?;
        Ok((copies, migrations))
    }

    fn process_file(path: &Path) -> io::Result<bool> { //completed
        let state_file = MancalaStateFile::new_from_read_file(path)?;
        state_file.copy_to_new_location()?;
        let completed = state_file.migrate_completed_file()?;
        std::fs::remove_file(&state_file.original_path)?;
        Ok(completed)
    }
}

pub struct MancalaStateFile {
    original_path: PathBuf,
    migrate_path: PathBuf,
    copy_path: PathBuf,
    hints: [u8; 6],
    completed: bool,
}

impl MancalaStateFile {
    pub fn new_from_read_file(path: &Path) -> io::Result<Self> {
        let hints = Self::read_hints_from_old(path)?;
        let (new_hints, completed) = Self::translate_hints(hints);
        let (migrate_path, copy_path) = FilePathHandler::convert_path_to_new_format(path);
        Ok(Self {
            original_path: path.to_path_buf(),
            migrate_path,
            copy_path,
            hints: new_hints,
            completed,
        })
    }

    pub fn copy_to_new_location(&self) -> io::Result<()> {
        FilePathHandler::create_directory_if_needed(&self.copy_path)?;
        fs::copy(&self.original_path, &self.copy_path)?;
        Ok(())
    }

    pub fn migrate_completed_file(&self) -> io::Result<bool> {
        if self.completed {
            FilePathHandler::create_directory_if_needed(&self.migrate_path)?;
            {
                let mut file = match OpenOptions::new()
                .write(true)
                .create(true)
                .open(&self.migrate_path) {
                    Ok(file) => file,
                    Err(e) => {
                        eprintln!("Error opening file: {}", e);
                        return Err(e);
                    }
                };

                file.write_all(&self.hints)?;
            }
            return Ok(true);
        }
        Ok(false)
    }

    fn read_hints_from_old(path: &Path) -> io::Result<[u8; 6]> {
        let mut file = match OpenOptions::new()
            .read(true)
            .open(&path) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Error opening file: {}", e);
                    return Err(e);
                }
            };

        let mut hints = [0u8; 6];
        file.read_exact(&mut hints)?;
        Ok(hints)
    }

    fn translate_hints(hints: [u8; 6]) -> ([u8; 6], bool) {
        let mut new_hints = [0u8; 6];
        let mut completed = true;
        for (i, &hint) in hints.iter().enumerate() {
            new_hints[i] = match hint & HINT_TYPE_MASK {
                ORIGINAL_INVALID_HINT => NEW_INVALID_HINT,
                COMPLETE_HINT => hint,
                _ => {
                    completed = false;
                    hint
                },
            };
        }
        (new_hints, completed)
    }
}

pub struct FilePathHandler;

impl FilePathHandler {

    pub fn convert_path_to_new_format(path: &Path) -> (PathBuf, PathBuf) {
        let mut components = path.iter();
        let _ = components.next();
        let mut hex = String::with_capacity(12);

        for _ in 0..6 {
            hex.push_str(components.next().unwrap().to_str().unwrap());
        }

        let filename = components.next().unwrap();

        (
            PathBuf::from(MIGRATED_DATA_DIR)
                .join(&hex[0..3])
                .join(&hex[3..6])
                .join(&hex[6..9])
                .join(&hex[9..12])
                .join(filename),
            PathBuf::from(NEW_PATH_DATA_DIR)
                .join(&hex[0..3])
                .join(&hex[3..6])
                .join(&hex[6..9])
                .join(&hex[9..12])
                .join(filename)
        )
    }

    pub fn create_directory_if_needed(path: &Path) -> io::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        Ok(())
    }
}