use crate::constants::*;
use crate::mancala_game_codec::{MancalaGameCodec};
use crate::types::*;
use crate::mancala_game_model::MancalaGame;
use crate::error::{CodecError, MoveError};
use std::io::{self, Read, Write, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::fs::{OpenOptions};

pub struct MancalaStateFile {
    code: EncodedGameState,
    path: PathBuf,
    hints: [EncodedHint; 6],        // Hint possui 2 bits iniciais de controle, e 6 bits correspondentes ao valor
    parents: Vec<EncodedGameState>  // ISSO PRECISA SER O VALOR JUNTO COM O ÚLTIMO MOVIMENTO! PODE USAR O PRIMEIRO CARACTER HEX PARA ARMAZENAR O MOVIMENTO E MANTER U64
}

const HINT_MASK_TYPE: EncodedHint = 0xC0;   // 1100 0000
const HINT_MASK_VALUE: EncodedHint = 0x3F;  // 0011 1111

const HINT_COMPLETED: EncodedHint = 0x80;   // 1000 0000
const HINT_INVALID: EncodedHint = 0xC0;     // 1100 0000
const HINT_UNKNOWN: EncodedHint = 0x40;     // 0100 0000
const HINT_NORMAL: EncodedHint = 0x00;      // 0000 0000


#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hint {
    value: Pebbles,
    hint_type: EncodedHint
}

impl Hint {
    pub fn new_empty() -> Self {
        Self {
            value: 0,
            hint_type: HINT_UNKNOWN
        }
    }

    pub fn new(value: Pebbles, hint_type: EncodedHint) -> Self {
        Self {
            value,
            hint_type
        }
    }
}

pub type HintData = [Hint; 6];

enum ForwardReason {
    NoForward,
    NewBestHint,
    StateCompletion
}

impl MancalaStateFile {
    pub fn new_empty(state_code: EncodedGameState, parent: Option<EncodedGameState>) -> Self {

        let mut hints: [EncodedHint; 6] = [HINT_UNKNOWN; 6];

        for i in 0..NUM_PITS {
            if MancalaGameCodec::get_pebbles_on_code_pit(state_code, i + 1) == 0 {
                hints[i] = HINT_INVALID;
            }
        }

        Self {
            code: state_code,
            path: FilePathHandler::generate_path(state_code),
            hints,
            parents: parent.into_iter().collect()
        }
    }

    pub fn new_from_read_file(state_code: EncodedGameState) -> io::Result<Self> {
        let path = FilePathHandler::generate_path(state_code);
        if path.exists() {
            return Self::read_file(state_code, Some(path));
        } else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "File not found"));
        }
    }

    pub fn save_as_new(self) -> io::Result<()> {
        if self.path.exists(){
            return Ok(());
        }

        FilePathHandler::create_directory_if_needed(&self.path)?;
        let mut file = match OpenOptions::new()
            .write(true)
            .create(true)
            .open(&self.path) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Error opening file: {}", e);
                    return Err(e);
                }
            };

        // hints
        file.write_all(&self.hints)?;

        // parents
        for parent in self.parents {
            file.write_all(&parent.to_be_bytes())?;
        }

        Ok(())
    }

    pub fn append_parent_if_needed(&mut self, parent_code: EncodedGameState) -> io::Result<bool> {
        if !self.parents.contains(&parent_code) {
            self.parents.push(parent_code);
            let mut file = OpenOptions::new()
                .append(true)
                .open(self.path.clone())?;

            file.write_all(&parent_code.to_be_bytes())?;
            return Ok(true);
        }
        Ok(false)
    }

    pub fn save_updated_hint(&mut self, pit: Pit, new_hint: EncodedHint) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .open(self.path.clone())?;

        let offset: u64 = (pit - 1).try_into().unwrap();

        file.seek(SeekFrom::Start(offset))?;
        file.write_all(&[new_hint])?;

        self.hints[pit - 1] = new_hint;

        Ok(())
    }

    pub fn get_hint_value(&self, pit: Pit) -> Pebbles {
        self.hints[pit - 1] & HINT_MASK_VALUE
    }

    pub fn get_hint_type(&self, pit: Pit) -> u8 {
        self.hints[pit - 1] & HINT_MASK_TYPE
    }

    fn get_completed(&self) -> bool {
        self.hints.iter().all(|&hint| (hint & HINT_MASK_TYPE) >= HINT_COMPLETED) // State is completed if all moves are completed or invalid
    }

    pub fn get_best_hint_value(&self) -> Pebbles {
        let mut best_hint_value: Pebbles = 0;
        for i in 1..=6 {
            let hint_type = self.get_hint_type(i);
            let hint_value = self.get_hint_value(i);
            if (hint_type == HINT_COMPLETED || hint_type == HINT_NORMAL) && hint_value > best_hint_value {
                best_hint_value = hint_value;
            }
        }
        best_hint_value
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

        let mut hints = [0u8; 6];
        let mut parents: Vec<EncodedGameState> = Vec::new();

        file.read_exact(&mut hints)?;

        let mut parent_buffer=  [0u8; 8];
        while let Ok(_) = file.read_exact(&mut parent_buffer) {
            let parent = u64::from_be_bytes(parent_buffer);
            parents.push(parent);
        }

        Ok(Self { code: state_code, path, hints, parents })
    }
}

struct FilePathHandler;

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

    fn create_directory_if_needed(path: &Path) -> io::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum CodecErrorOrError {
    CodecError(CodecError),
    Error(io::Error)
}

pub struct DataHandler;

impl DataHandler {
    pub fn fetch_hint_data(mancala_game: MancalaGame) -> Result<HintData, CodecErrorOrError> {
        let mut hint_data: HintData = [Hint::new_empty(); 6];
        let current_turn = match mancala_game.get_current_turn() {
            Some(turn) => turn,
            None => return Ok(hint_data)
        };
        let current_mancala = mancala_game.get_pit_pebbles(current_turn, MANCALA_PIT);
        let state_code = match MancalaGameCodec::encode(&mancala_game, None) {
            Ok(code) => code,
            Err(e) => return Err(CodecErrorOrError::CodecError(e))
        };

        let game_state = match MancalaStateFile::new_from_read_file(state_code) {
            Ok(state) => state,
            Err(e) => return Err(CodecErrorOrError::Error(e)),
        };

        //println!("Fetching data from {:?}", game_state.path);

        for i in 0..NUM_PITS {
            let value = game_state.get_hint_value(i + 1) + current_mancala;
            let hint_type = game_state.get_hint_type(i + 1);
            hint_data[i] = Hint::new(value, hint_type);
        }

        Ok(hint_data)
    }

    pub fn simulate_until_moves(game: MancalaGame, ends_reached: i32) -> io::Result<i32> {
        let mut ends_reached = ends_reached;
        if game.get_current_turn() == None {
            return Ok(1);
        }
        let hints = Self::fetch_hint_data(game).expect("Failed to retrieve hints");
        
        for i in 1..=NUM_PITS {
            if ends_reached >= 50000000 {
                println!("Ends Reached: {}", ends_reached);
                // return Ok(ends_reached);
            }
            if hints[i - 1].hint_type == HINT_COMPLETED || hints[i - 1].hint_type == HINT_INVALID { continue }
            let next_game = match game.clone().make_move(i) {
                Ok(game) => { game }
                Err(_) => { continue }
            };
            Self::save_data(next_game, game, i)?;
            Self::simulate_until_moves(next_game, ends_reached)?;
        }

        Ok(ends_reached)
    }

    pub fn save_data(game: MancalaGame, parent_game: MancalaGame, move_pit: Pit) -> io::Result<()> { //returns a bool saying if there were any changes by forwarding or not
        let state_code = MancalaGameCodec::encode(&game, None).expect("Failed to encode game");
        let parent_code = MancalaGameCodec::encode(&parent_game, Some(move_pit)).expect("Failed to encode game");
        let file_path = FilePathHandler::generate_path(state_code);

        if !FilePathHandler::generate_path(parent_code).exists() { // If parent doesn't exist, exit
            return Err(io::Error::new(io::ErrorKind::NotFound, "Parent file not found"));
        }

        if !file_path.exists() && state_code != EMPTY_BOARD_CODE {
            return Self::create_file(state_code, parent_code, Some(file_path));
        } else { // file already exists
            let parent_turn = parent_game.get_current_turn().unwrap();
            let current_turn = match state_code {
                EMPTY_BOARD_CODE => parent_turn,
                _ => game.get_current_turn().unwrap()
            };
            return Self::begin_forward_data_if_needed(
                state_code,
                game.get_pit_pebbles(current_turn, MANCALA_PIT),
                parent_code,
                parent_game.get_pit_pebbles(parent_turn, MANCALA_PIT)
            );
        }
    }

    fn begin_forward_data_if_needed(current_code: EncodedGameState, current_mancala: Pebbles, parent_code: EncodedGameState, parent_mancala: Pebbles) -> io::Result<()> {
        let mut current_state = match current_code {
            EMPTY_BOARD_CODE => MancalaStateFile::new_empty(current_code, Some(parent_code)),
            _ => MancalaStateFile::new_from_read_file(current_code)?
        };
        let parent_included = current_state.append_parent_if_needed(parent_code)?;

        if !current_state.get_completed() && !parent_included {
            return Ok(())
        }

        if current_code == EMPTY_BOARD_CODE {
            Self::endgame_forward_data(
                current_state,
                parent_mancala,
                current_mancala
            )?;
        } else {
            Self::forward_data_recursion(
                current_state,
                current_mancala,
                ForwardReason::NewBestHint
            )?;
        }

        Ok(())
    }

    fn endgame_forward_data(current_state: MancalaStateFile, parent_mancala: Pebbles, endgame_mancala: Pebbles) -> io::Result<()> {
        let parent_code= current_state.parents[0]; // only one parent should be expected if game end has been reached

        let mut parent_state = match MancalaStateFile::new_from_read_file(parent_code) {
            Ok(state) => state,
            Err(e) => return Err(e)
        };

        let move_pit = MancalaGameCodec::get_move_from_code(parent_code).unwrap();

        if parent_state.get_hint_type(move_pit) == HINT_COMPLETED {
            return Ok(());
        }

        let pebbles_move: Pebbles = endgame_mancala - parent_mancala;

        let new_hint: EncodedHint = pebbles_move | HINT_COMPLETED;
        
        parent_state.save_updated_hint(move_pit, new_hint)?;

        Self::forward_data_recursion(parent_state, parent_mancala, ForwardReason::NewBestHint)
    }

    fn forward_data_recursion(current_state: MancalaStateFile, base_current_mancala: Pebbles, reason: ForwardReason) -> io::Result<()> {
        let base_endgame_mancala = base_current_mancala + current_state.get_best_hint_value();

        for parent_code in current_state.parents.clone() {

            let mut parent_state = match MancalaStateFile::new_from_read_file(parent_code) {
                Ok(state) => state,
                Err(e) => return Err(e)
            };

            let move_pit = MancalaGameCodec::get_move_from_code(parent_code).unwrap();

            if parent_state.get_hint_type(move_pit) == HINT_COMPLETED {
                continue
            }

            let current_pebbles = MancalaGameCodec::get_pebbles_on_code(current_state.code);
            let parent_pebbles = MancalaGameCodec::get_pebbles_on_code(parent_code);
            let free_turn_ind = Self::get_free_turn_ind(parent_code);

            let current_mancala = Self::swap_mancala_value_if_needed(base_current_mancala, free_turn_ind, current_pebbles);
            let endgame_mancala = Self::swap_mancala_value_if_needed(base_endgame_mancala, free_turn_ind, 0);
            let parent_mancala = current_mancala - (parent_pebbles - current_pebbles);

            let pebbles_move = endgame_mancala - parent_mancala;


            let new_hint: EncodedHint = match reason {
                ForwardReason::NoForward => panic!("Help"),
                ForwardReason::StateCompletion => {
                    match parent_state.get_hint_type(move_pit) {
                        HINT_UNKNOWN => pebbles_move | HINT_COMPLETED,
                        HINT_NORMAL => parent_state.get_hint_value(move_pit) | HINT_COMPLETED,
                        HINT_COMPLETED => panic!("If the hint is already completed, it shouldn't be forwarding stuff"),
                        HINT_INVALID => panic!("It should be impossible to be here since this move is impossible"),
                        _ => panic!("Math doesn't make sense and the universe will explode")
                    }
                },
                ForwardReason::NewBestHint => {
                    match current_state.get_completed() {
                        true => pebbles_move | HINT_COMPLETED,
                        false => pebbles_move
                    }
                }
            };

            let best_hint_before = parent_state.get_best_hint_value();
            let mod_hint_type_before = parent_state.get_hint_type(move_pit);
            
            parent_state.save_updated_hint(move_pit, new_hint)?;

            let best_hint_after = parent_state.get_best_hint_value();

            let should_forward = if best_hint_before != best_hint_after || mod_hint_type_before == HINT_UNKNOWN { ForwardReason::NewBestHint }
                else if parent_state.get_completed() { ForwardReason::StateCompletion }
                else { ForwardReason::NoForward };

            match should_forward {
                ForwardReason::NoForward => (),
                _ => Self::forward_data_recursion(parent_state, parent_mancala, should_forward)?
            }
        }

        Ok(())
    }

    fn swap_mancala_value_if_needed(mancala_value: Pebbles, free_turn_ind: bool, board_pebbles: Pebbles) -> Pebbles {
        match free_turn_ind {
            true => mancala_value,
            false => TOTAL_PEBBLES - (mancala_value + board_pebbles),
        }
    }

    fn get_free_turn_ind(state_code: EncodedGameState) -> bool {
        state_code == EMPTY_BOARD_CODE || MancalaGameCodec::check_free_turn_on_code(state_code, None)
    }

    fn create_file(state_code: EncodedGameState, parent_code: EncodedGameState, file_path: Option<PathBuf>) -> io::Result<()> {
        let file_path = match file_path {
            Some(path) => path,
            None => FilePathHandler::generate_path(state_code)
        };

        FilePathHandler::create_directory_if_needed(&file_path)?;

        let file_content = MancalaStateFile::new_empty(state_code,Some(parent_code));

        let mut file = match OpenOptions::new()
            .write(true)
            .create(true)
            .open(&file_path) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Error opening file: {}", e);
                    return Err(e);
                }
            };

        // hints
        file.write_all(&file_content.hints)?;

        // parents
        for parent in file_content.parents {
            file.write_all(&parent.to_be_bytes())?;
        }

        Ok(())
    }
}