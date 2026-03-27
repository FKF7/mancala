pub type Pit = usize;
pub type Pebbles = u8;
pub type MancalaTurn = Option<usize>;   // 0 ou 1
pub type MancalaBoard = [[Pebbles; 7]; 2];
pub type EncodedGameState = u64;
pub type EncodedHint = u8;