use serde::{Deserialize, Serialize};
use crate::constants::{PLAYER1, PLAYER2, NUM_PITS, NUM_PEBBLES, FULL_CYCLE, MANCALA_PIT, TOTAL_PEBBLES};
use crate::types::{MancalaBoard, MancalaTurn, Pit, Pebbles};
use crate::error::MoveError;
use std::fmt;
use std::io;

enum Outcome {
    Normal,
    FreeTurn,
    Capture
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Copy)]
#[serde(rename_all = "camelCase")]
pub struct MancalaGame {
    board: MancalaBoard,
    current_turn: MancalaTurn
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
    pub fn new_from_game(board: MancalaBoard, current_turn: MancalaTurn) -> Self {
        match (board, current_turn) {
            (board, Some(turn)) => Self {
                board: board.clone(),
                current_turn: Some(turn),
            },
            _ => Self::default(), // se não veio nada, usa inicial padrão
        }
    }

    pub fn make_move(mut self, mut pit: Pit) -> Result<Self, MoveError> {
        if let Some(player) = self.current_turn {
            let mut board_side = player;
            let mut pebbles = self.board[player][pit];

            if pebbles == 0 { return Err(MoveError::InvalidMove) }
            
            let landing = Self::calc_landing_pos(pit, pebbles);
            let outcome: Outcome = if landing == pit {
                if self.board[player][pit] == FULL_CYCLE { Outcome::Capture } else { Outcome::Normal }
            } else {
                match landing {
                    0 => Outcome::FreeTurn,
                    1..=6 => if self.board[player][landing] == 0 && (self.board[switch_turn_number(player)][7 - landing] != 0 || landing > pit) && pebbles < FULL_CYCLE { Outcome::Capture } else { Outcome::Normal },
                    _ => Outcome::Normal,
                }
            };
            self.board[player][pit] = 0;

            while pebbles > 0 {
                if pit > MANCALA_PIT {
                    if pit == 1 && board_side != player {
                        pit = NUM_PITS;
                        board_side = switch_turn_number(board_side);
                    } else {
                        pit -= 1;
                    }
                } else {
                    pit = NUM_PITS;
                    board_side = switch_turn_number(board_side);
                }
                self.board[board_side][pit] += 1;
                pebbles -= 1;
            }

            match outcome {
                Outcome::FreeTurn => (),
                Outcome::Capture => {
                    self.board[player][MANCALA_PIT] += 1 + self.board[switch_turn_number(player)][7 - landing];
                    self.board[player][landing] = 0;
                    self.board[switch_turn_number(player)][7 - landing] = 0;
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

            return Ok(self);
        } else {
            return Err(MoveError::InvalidTurn);
        }
    }

    pub fn get_current_turn(self) -> MancalaTurn {
        return self.current_turn;
    }

    pub fn get_pit_pebbles(self, player: usize, pit: usize) -> u8 {
        return self.board[player][pit];
    }

    fn reset_current_turn(&mut self) {
        self.current_turn = None;
    }

    fn calc_landing_pos(pit: Pit, pebbles: Pebbles) -> Pit {
        (FULL_CYCLE as i32 + pit as i32 - pebbles as i32).rem_euclid(FULL_CYCLE as i32) as Pit
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