use crate::types::*;

pub const PLAYER1: usize = 0;
pub const PLAYER2: usize = 1;
pub const NUM_PITS: Pit = 6;
pub const NUM_PEBBLES: Pebbles = 4;
pub const FULL_CYCLE: Pebbles = 13;
pub const MANCALA_PIT: Pit = 0;
pub const TOTAL_PEBBLES: Pebbles = 48;
pub const DEFAULT_BOARD_CODE: EncodedGameState = 0x0210842108421084;
pub const EMPTY_BOARD_CODE: EncodedGameState = 0;
pub const BASE_DATA_DIR: &str = "data";
pub const DATA_FILE_EXT: &str = "bin";
pub const BITS_PER_PIT: u8 = 5;
pub const TOTAL_CODE_BITS: u8 = 60;
pub const MAX_SIMULATION_MOVES: i32 = 100;
