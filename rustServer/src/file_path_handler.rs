use crate::constants::{BASE_DATA_DIR, DATA_FILE_EXT, TEMP_FILE_EXT};
use crate::types::EncodedGameState;
use std::path::{PathBuf, Path};
use std::io;

pub struct FilePathHandler;

impl FilePathHandler {
    pub fn generate_path(code: EncodedGameState) -> PathBuf {
        let hex = format!("{:016X}", code);
        let trimmed = &hex[1..]; // remove first 0

        PathBuf::from(BASE_DATA_DIR)
            .join(&trimmed[0..2])
            .join(&trimmed[2..4])
            .join(&trimmed[4..6])
            .join(&trimmed[6..8])
            .join(&trimmed[8..10])
            .join(&trimmed[10..12])
            .join(format!("{}.{}", &trimmed[12..15], DATA_FILE_EXT))
    }

    pub fn get_temp_path(path: &PathBuf) -> PathBuf {
        let mut new_path = path.clone();
        new_path.set_extension(TEMP_FILE_EXT);
        new_path
    }

    pub fn create_directory_if_needed(path: &Path) -> io::Result<()> {
        if let Some(parent) = path.parent() {
            match std::fs::create_dir_all(parent) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}