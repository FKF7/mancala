use serde::{Deserialize, Serialize};
use std::fmt;

enum Outcome {
    Normal,
    FreeTurn,
    Capture
}

pub type Pit = u8;
pub type MancalaTurn = Option<usize>;   // 0 ou 1
pub type MancalaBoard = [[u8; 7]; 2];

pub const PLAYER1: usize = 0;
pub const PLAYER2: usize = 1;
pub const MANCALA_PIT: usize = 0;
pub const NUM_PITS: u8 = 6;
pub const NUM_PEBBLES: u8 = 4;
pub const FULL_CYCLE: u8 = 13;
pub const FREE_TURN: u8 = 0;
pub const CAPTURE: u8 = 1;
pub const TOTAL_PEBBLES: u8 = 48;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MancalaGame {
    board: MancalaBoard,
    current_turn: MancalaTurn,
}

impl Default for MancalaGame {
    fn default() -> Self {
        let mut board: MancalaBoard = [[0u8; 7]; 2];
        for i in 1..=6 {
            board[PLAYER1][i] = NUM_PEBBLES;
            board[PLAYER2][i] = NUM_PEBBLES;
        }
        Self { board, current_turn: Some(PLAYER1) }
    }
}

impl fmt::Display for MancalaGame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "MancalaGame:")?;
        write!(f, "   ")?;
        for i in 1..=6 {
            write!(f, " {}", self.board[1][i])?;
        }
        write!(f, "\n")?;
        writeln!(f, "  {}             {}", self.board[1][0], self.board[0][0])?;
        write!(f, "   ")?;
        for i in (1..=6).rev() {
            write!(f, " {}", self.board[0][i])?;
        }
        write!(f, "\n")?;
        writeln!(f, "  Current turn: {}", match self.current_turn {
            Some(n) => n.to_string(),
            None => "None".to_string(),
        })?;
        Ok(())
    }
}

impl MancalaGame {
    pub fn new_from_game(board: Option<[[u8; 7]; 2]>, current_turn: MancalaTurn) -> Self {
        match (board, current_turn) {
            (Some(b), Some(turn)) => Self {
                board: b.clone(), // clone profundo (como pediu)
                current_turn: Some(turn),
            },
            _ => Self::default(), // se não veio nada, usa inicial padrão
        }
    }

    pub fn make_move(mut self, mut pit: Pit) -> Self {
        if let Some(player) = self.current_turn {
            let mut board_side = player;
            let mut pebbles = self.board[player][pit as usize];
            
            let landing = (FULL_CYCLE + pit - pebbles) % FULL_CYCLE;
            let outcome: Outcome = if landing == pit {
                if self.board[player][pit as usize] == FULL_CYCLE { Outcome::Capture } else { Outcome::Normal }
            } else {
                match landing {
                    0 => Outcome::FreeTurn,
                    1..=6 => if self.board[player][landing as usize] == 0 && self.board[switch_turn_number(player)][7 - landing as usize] != 0 || landing > pit { Outcome::Capture } else { Outcome::Normal },
                    _ => Outcome::Normal,
                }
            };
            self.board[player][pit as usize] = 0;

            while pebbles > 0 {
                if pit > MANCALA_PIT as u8 {
                    if pit == 1 && board_side != player {
                        pit = NUM_PITS as Pit;
                        board_side = switch_turn_number(board_side);
                    } else {
                        pit -= 1;
                    }
                } else {
                    pit = NUM_PITS as Pit;
                    board_side = switch_turn_number(board_side);
                }
                self.board[board_side][pit as usize] += 1;
                pebbles -= 1;
            }

            match outcome {
                Outcome::FreeTurn => (),
                Outcome::Capture => {
                    self.board[player][MANCALA_PIT] += 1 + self.board[switch_turn_number(player)][7 - landing as usize];
                    self.board[player][landing as usize] = 0;
                    self.board[switch_turn_number(player)][7 - landing as usize] = 0;
                    self.switch_current_turn();
                },
                Outcome::Normal => {
                    self.switch_current_turn();
                },
            };

            if let Some(empty_side) = check_for_end(self.board) {
                self.board[switch_turn_number(empty_side)][1..=6].fill(0);
                self.board[switch_turn_number(empty_side)][MANCALA_PIT] = TOTAL_PEBBLES - self.board[empty_side][MANCALA_PIT];
                self.reset_current_turn();
            }
        }

        // if (boardSide !== null) {

        //     if (game.getPit(boardSide, pit) == 1 && boardSide === game.getCurrentTurn() && pit !== 0 && game.getOppositePit(boardSide, pit) !== 0) {
        //         game.incrementCurrentBoardPit(0, game.getOppositePit(game.getCurrentTurn(), pit) + 1);
        //         game.resetCapturePits(boardSide, pit);
        //     }
            
        //     if(this.isGameEnded(game)) {
        //         this.endGame(game);
        //     } else if (!freeTurn) {
        //         game.switchPlayerTurn();
        //     }
        // }

        return self;
    }
    
    pub fn reset_board(mut self) {
        self.board = [
            [0, 4, 4, 4, 4, 4, 4],
            [0, 4, 4, 4, 4, 4, 4],
        ];
        self.current_turn = Some(PLAYER1);
    }

    pub fn get_board(self) -> MancalaBoard {
        return self.board;
    }

    pub fn get_current_turn(self) -> MancalaTurn {
        return self.current_turn;
    }

    pub fn reset_current_turn(&mut self) {
        self.current_turn = None;
    }

    fn switch_current_turn(&mut self) {
        self.current_turn = match self.current_turn {
            Some(n) => Some(n ^ 1),
            None => None,
        };
    }
}

fn switch_turn_number(current_turn: usize) -> usize {
    return current_turn ^ 1;
}

fn check_for_end(board: MancalaBoard) -> MancalaTurn {
    if (1..=6).all(|i| board[PLAYER1][i] == 0) {
        return Some(PLAYER1);
    } else if (1..=6).all(|i| board[PLAYER2][i] == 0) {
        return Some(PLAYER2);
    }

    None
}