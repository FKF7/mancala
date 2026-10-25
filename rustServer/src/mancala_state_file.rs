use std::io::{self, Read, Write, Seek, SeekFrom};
use std::fs::OpenOptions;
use std::path::PathBuf;
use crate::types::*;
use crate::mancala_codec::MancalaCodec;
use crate::hint::{Hint, HintData, HintType};
use crate::constants::{NUM_PITS};
use crate::file_path_handler::FilePathHandler;

// TO DO LIST:
// IMPROVE ERROR HANDLING AND LOGGING
// FINISH IMPLEMENTING THE PRUNE BOOL IN FORWARD DATA RECURSION

const HINT_BYTES_IN_FILE: u64 = 3; // 1 byte for encoded hint, 1 byte for min, 1 byte for max
const TOTAL_HINT_BYTES_IN_FILE: u64 = HINT_BYTES_IN_FILE * NUM_PITS as u64;
const FILE_HEADER_SIZE: u64 = TOTAL_HINT_BYTES_IN_FILE + 2; // 2 bytes for min and max
const COMPLETED_FILE_SIZE: u64 = 6;
const PARENT_SIZE: u64 = 8;


#[derive(PartialEq, PartialOrd, Eq, Clone, Copy, Debug)]
pub enum StateCompletion {
    Incomplete,
    PrunedCompleted,
    Completed
}

#[derive(Clone, Debug)]
pub struct MancalaStateFile {
    code: EncodedGameState,
    path: PathBuf,
    hints: HintData,
    min: Pebbles,
    max: Pebbles,
    completion: StateCompletion,
    parents: Vec<EncodedGameState>
}

impl MancalaStateFile {
    pub fn new_empty(state_code: EncodedGameState, parent: Option<EncodedGameState>) -> Self {
        let pebbles_left = MancalaCodec::get_pebbles_on_game_code(state_code);

        let hints: HintData = std::array::from_fn(|i| {
            if MancalaCodec::get_pebbles_on_game_code_pit(state_code, i + 1) == 0 {
                Hint::new_invalid()
            } else {
                Hint::new_empty(pebbles_left)
            }
        });

        Self {
            code: state_code,
            path: FilePathHandler::generate_path(state_code),
            hints,
            min: 0,
            max: pebbles_left,
            completion: StateCompletion::Incomplete,
            parents: parent.into_iter().collect()
        }
    }

    pub fn new_from_read_file(state_code: EncodedGameState) -> io::Result<Self> {
        let path = FilePathHandler::generate_path(state_code);
        if path.exists() {
            return Self::read_file(state_code, Some(path));
        } else {
            return Err(io::Error::new(io::ErrorKind::NotFound, format!("File not found: {}", path.display())));
        }
    }

    pub fn save_as_new(self) -> io::Result<()> {
        if self.path.exists(){
            return Ok(());
        }

        let temp_file = FilePathHandler::get_temp_path(&self.path); // for atomic writing
        {
            FilePathHandler::create_directory_if_needed(&temp_file)?;
            let mut file = match OpenOptions::new()
                .write(true)
                .create(true)
                .open(&temp_file) {
                    Ok(file) => file,
                    Err(e) => {
                        eprintln!("Error opening file: {}", e);
                        return Err(e);
                    }
                };

            // hints
            for hint in self.hints {
                file.write_all(&[hint.get_encoded(), hint.get_min(), hint.get_max()])?;
            }

            // min max
            file.write_all(&self.min.to_be_bytes())?;
            file.write_all(&self.max.to_be_bytes())?;

            // parents
            for parent in self.parents {
                file.write_all(&parent.to_be_bytes())?;
            }
            file.flush()?;
        }

        std::fs::rename(&temp_file, &self.path)?;
        Ok(())
    }

    fn save_as_completed(&mut self) -> io::Result<()> {
        let temp_file = FilePathHandler::get_temp_path(&self.path); // for atomic writing
        {
            FilePathHandler::create_directory_if_needed(&temp_file)?;
            let mut file = match OpenOptions::new()
                .write(true)
                .create(true)
                .open(&temp_file) {
                    Ok(file) => file,
                    Err(e) => {
                        eprintln!("Error opening file: {}", e);
                        return Err(e);
                    }
                };

            // write the 6 bytes for completed state
            for hint in self.hints {
                file.write_all(&[hint.get_encoded()])?;
            }
            file.flush()?;
        }

        std::fs::rename(&temp_file, &self.path)?;
        Ok(())
    }

    /// Checks if the parent code is in the list of parents, and if not, appends it to the list and writes it to the file.
    /// Returns true if the parent was added, false if it was already present.
    pub fn append_parent_if_needed(&mut self, parent_code: EncodedGameState) -> io::Result<bool> {
        if !self.parents.contains(&parent_code) {
            self.parents.push(parent_code);
            
            if self.get_completion() != StateCompletion::Completed {
                let mut file = OpenOptions::new()
                    .append(true)
                    .open(&self.path)?;

                file.write_all(&parent_code.to_be_bytes())?;
            }
            
            return Ok(true);
        }
        Ok(false)
    }

    pub fn save_updated_hint(&mut self, pit: Pit, new_hint: Hint) -> io::Result<bool> {
        let mut file = OpenOptions::new()
            .write(true)
            .open(self.path.clone())?;

        if pit < 1 || pit > NUM_PITS {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Pit number out of range on {}", self.path.display())));
        }

        self.hints[pit - 1] = new_hint;
        let mut state_metadata_updated = self.update_completion();
        
        if self.get_completion() == StateCompletion::Completed {
            self.update_max();
            self.update_min(new_hint.get_min());
            match self.save_as_completed() {
                Ok(()) => return Ok(true),
                Err(e) => return Err(e),
            }
        } else {
            let offset: u64 = (pit as u64 - 1) * HINT_BYTES_IN_FILE;
            file.seek(SeekFrom::Start(offset))?;

            file.write_all(&[MancalaCodec::encode_hint(&new_hint), new_hint.get_min(), new_hint.get_max()])?;

            if self.update_min(new_hint.get_min()) {
                file.seek(SeekFrom::Start(NUM_PITS as u64 * HINT_BYTES_IN_FILE))?;
                file.write_all(&self.min.to_be_bytes())?;
                state_metadata_updated = true;
            }

            if self.update_max() {
                file.seek(SeekFrom::Start(NUM_PITS as u64 * HINT_BYTES_IN_FILE + 1))?;
                file.write_all(&self.max.to_be_bytes())?;
                state_metadata_updated = true;
            }
            return Ok(state_metadata_updated);
        }
    }

    pub fn get_code(&self) -> EncodedGameState {
        self.code
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }

    pub fn get_hints(&self) -> &HintData {
        &self.hints
    }

    pub fn get_hint(&self, pit: Pit) -> Hint {
        self.hints[pit - 1]
    }

    pub fn get_hint_value(&self, pit: Pit) -> Pebbles {
        self.hints[pit - 1].get_value()
    }

    pub fn get_hint_type(&self, pit: Pit) -> HintType {
        self.hints[pit - 1].get_type()
    }

    pub fn get_min(&self) -> Pebbles {
        self.min
    }

    pub fn get_max(&self) -> Pebbles {
        self.max
    }

    /// Updates the completion status of the state based on the current hints.
    /// Returns true if the completion status changed, false otherwise.
    pub fn update_completion(&mut self) -> bool {
        let completion_before = self.completion;
        self.completion = Self::_update_completion(&self.hints);
        
        completion_before != self.completion
    }

    pub fn get_completion(&self) -> StateCompletion {
        self.completion
    }

    pub fn get_parents(&self) -> &Vec<EncodedGameState> {
        &self.parents
    }

    pub fn get_best_hint_value(&self) -> Pebbles {
        let mut best_hint_value: Pebbles = 0;
        for i in 1..=6 {
            let hint_value = self.get_hint_value(i);
            // unknown hints will have value = 0, so this it's not a problem to compare them too
            // I guess before Alpha-Beta update, comparing type wasn't needed either cause invalid would also return zero
            if self.hints[i - 1].get_type() != HintType::Invalid && hint_value > best_hint_value {
                best_hint_value = hint_value;
            }
        }
        best_hint_value
    }

    /// Updates the maximum hint value of the state.
    /// Returns true if the maximum hint value changed, false otherwise.
    fn update_max(&mut self) -> bool {
        let max_before = self.max;
        self.max = self.hints.iter().filter(|hint| hint.get_type() != HintType::Invalid)
            .map(|hint| hint.get_max()).max().unwrap();
        return max_before != self.max;
    }

    fn update_min(&mut self, new_min_candidate: Pebbles) -> bool {
        if new_min_candidate > self.min {
            self.min = new_min_candidate;
            return true;
        }
        return false;
    }

    fn read_file(state_code: EncodedGameState, path: Option<PathBuf>) -> io::Result<Self> {
        let path = path.unwrap_or_else(|| FilePathHandler::generate_path(state_code));
        let mut file = match OpenOptions::new()
            .read(true)
            .open(&path) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Error opening file: {}", e);
                    return Err(e);
                }
            };

        let size = file.metadata()?.len();

        if size == COMPLETED_FILE_SIZE {
            return Self::read_completed_file(state_code, path);
        } else {
            if size < FILE_HEADER_SIZE || (size - FILE_HEADER_SIZE) % PARENT_SIZE != 0 {
                return Err(io::Error::new(io::ErrorKind::InvalidData, format!("File size is not valid: {}", path.display())));
            }
        }

        let mut hints_vec = Vec::<Hint>::with_capacity(NUM_PITS);

        for _ in 0..NUM_PITS {
            let mut hint_buffer = [0u8; 3];
            match file.read_exact(&mut hint_buffer) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Error reading hint from file: {}", path.display());
                    return Err(e);
                }
            }

            let [encoded_hint, min, max] = hint_buffer;
            hints_vec.push(Hint::new_from_encoded(encoded_hint, min, max));
        }

        let hints: HintData = hints_vec.try_into().expect("Failed to convert hints_vec to HintData array");

        let mut min_max_buffer = [0u8; 2];
        file.read_exact(&mut min_max_buffer)?;
        let [min, max] = min_max_buffer;

        let mut parents: Vec<EncodedGameState> = Vec::new();
        let mut parent_buffer=  [0u8; 8];
        while let Ok(_) = file.read_exact(&mut parent_buffer) {
            let parent = u64::from_be_bytes(parent_buffer);
            parents.push(parent);
        }

        let state_completion = Self::_update_completion(&hints);

        Ok(Self { code: state_code, path, hints, min, max, completion: state_completion, parents })
    }

    fn read_completed_file(state_code: EncodedGameState, path: PathBuf) -> io::Result<Self> {
        let mut file = match OpenOptions::new()
            .read(true)
            .open(&path) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Error opening file: {}", e);
                    return Err(e);
                }
            };

        let mut hints_buffer = [0u8; NUM_PITS];
        file.read_exact(&mut hints_buffer)?;

        let mut best_hint: Pebbles = 0;

        let hints: HintData = std::array::from_fn(|i| {
            let (hint_value, hint_type) = MancalaCodec::decode_hint(hints_buffer[i]);
            if hint_value > best_hint {
                best_hint = hint_value;
            }
            if hint_type == HintType::Invalid {
                return Hint::new_invalid();
            }
            Hint::new_completed(hint_value)
        });

        Ok(Self { code: state_code, path, hints, min: best_hint, max: best_hint, completion: StateCompletion::Completed, parents: Vec::new() })
    }

    fn _update_completion(hints: &HintData) -> StateCompletion {
        let mut pruned_found = false;
        for hint in hints.iter() {
            match hint.get_type() {
                HintType::Normal | HintType::Unknown => {
                    return StateCompletion::Incomplete;
                },
                HintType::PrunedCompleted => pruned_found = true,
                _ => ()
            };
        }

        if pruned_found {
            StateCompletion::PrunedCompleted
        } else {
            StateCompletion::Completed
        }
    }
}