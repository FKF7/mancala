use crate::constants::*;
use crate::mancala_codec::{MancalaCodec};
use crate::types::*;
use crate::mancala_game_model::MancalaGame;
use crate::error::{CodecError};
use crate::hint::{Hint, HintData, HintType};
use crate::file_path_handler::FilePathHandler;
use crate::mancala_state_file::{MancalaStateFile, StateCompletion};
use std::path::{PathBuf};
use std::collections::VecDeque;
use std::{io, println};

#[derive(Debug)]
pub enum CodecErrorOrError {
    CodecError(CodecError),
    Error(io::Error)
}

pub struct DataHandler {
    state_stack: Vec<MancalaStateFile>,
}

impl DataHandler {
    pub fn new(sim: Option<()>) -> Self {
        DataHandler {
            state_stack: match sim {
                Some(_) => Vec::with_capacity(40),
                None => Vec::new(),
            },
        }
    }

    pub fn fetch_path(mancala_game: MancalaGame) -> Result<PathBuf, CodecErrorOrError> {
        let state_code = match MancalaCodec::encode_game(&mancala_game, None) {
            Ok(code) => code,
            Err(e) => return Err(CodecErrorOrError::CodecError(e))
        };

        let path = FilePathHandler::generate_path(state_code);

        Ok(path)
    }

    pub fn fetch_sequence(state_code: EncodedGameState) -> Result<String, CodecErrorOrError> {
        let mut sequence = VecDeque::<String>::with_capacity(200); // will have 1 char per move, plus a newline for each non-free-turn move
        let mut current_code = state_code;

        while current_code != EMPTY_BOARD_CODE {
            let move_pit = MancalaCodec::get_move_from_game_code(current_code);
            
            if !MancalaCodec::check_free_turn_on_game_code(current_code, move_pit) {
                sequence.push_front("\n".to_string());
            }
            sequence.push_front(move_pit.unwrap().to_string());

            let state_file = match MancalaStateFile::new_from_read_file(current_code) {
                Ok(state) => state,
                Err(e) => return Err(CodecErrorOrError::Error(e)),
            };

            if state_file.get_parents().is_empty() {
                break;
            }
            current_code = state_file.get_parents()[0];
        }

        Ok(sequence.into_iter().collect())
    }

    pub fn fetch_hint_data(mancala_game: MancalaGame, state_code: Option<EncodedGameState>) -> Result<(HintData, Pebbles, Pebbles), CodecErrorOrError> {
        let state_code = match state_code {
            Some(code) => code,
            None => match MancalaCodec::encode_game(&mancala_game, None) {
                Ok(code) => code,
                Err(e) => return Err(CodecErrorOrError::CodecError(e))
            }
        };

        let current_turn = match mancala_game.get_current_turn() {
            Some(turn) => turn,
            None => return Ok(([Hint::new_empty(0); 6], 0, 0))
        };

        let current_mancala = mancala_game.get_pit_pebbles(current_turn, MANCALA_PIT);

        let game_state = match MancalaStateFile::new_from_read_file(state_code) {
            Ok(state) => state,
            Err(e) => return Err(CodecErrorOrError::Error(e)),
        };

        let saved_hints = game_state.get_hints();
        let hint_data: HintData = std::array::from_fn(|i| {
            Hint::new(
                saved_hints[i].get_value() + current_mancala,
                saved_hints[i].get_type(),
                saved_hints[i].get_min() + current_mancala,
                saved_hints[i].get_max() + current_mancala
            )
        });

        Ok((hint_data, game_state.get_min() + current_mancala, game_state.get_max() + current_mancala))
    }

    pub fn simulate_until_moves(&mut self, game: MancalaGame, ends_reached: u32, max_ends: Option<u32>, prune: bool) -> io::Result<(u32, Option<usize>)> {
        let mut ends_reached = ends_reached;
        let mut prune_index: Option<usize>;
        if game.get_current_turn() == None {
            return Ok((ends_reached + 1, None));
        }

        let state_code = MancalaCodec::encode_game(&game, None).expect("Failed to encode game state for simulation");
        let state = MancalaStateFile::new_from_read_file(state_code)?;
        
        let mut skipped_all = true;
        self.state_stack.push(state);
        let simulation_depth = self.state_stack.len() - 1;
        for i in 1..=NUM_PITS {
            match max_ends {
                Some(max) => {
                    if (max == 0 && ends_reached >= DEFAULT_MAX_SIMULATION_ENDS) || ends_reached >= max {
                        self.state_stack.pop();
                        return Ok((ends_reached, None));
                    };
                },
                _ => ()
            }

            let hint = self.state_stack[simulation_depth].get_hint(i);
            match hint.get_type() {
                HintType::Completed | HintType::Invalid => continue,
                HintType::PrunedCompleted => if prune { continue } else { skipped_all = false },
                HintType::Normal => if prune && hint.get_max() <= self.state_stack[simulation_depth].get_min() { continue } else { skipped_all = false },
                HintType::Unknown => skipped_all = false,
            }

            let next_game = match game.clone().make_move(i) {
                Ok(game) => { game }
                Err(_) => { continue }
            };
            match self.save_data(next_game, game, i, self.state_stack.len() - 1)? {
                Some(index) => {
                    // println!("Pruned at depth {} for move {}, while at simulation depth {}", index, i, simulation_depth);
                    if index < simulation_depth {
                        // println!("would have returned here");

                        self.state_stack.pop();
                        // println!("Pruned at depth {} for move {}", index, i);
                        return Ok((ends_reached, Some(index)));
                    }
                },
                None => ()
            }
            (ends_reached, prune_index) = self.simulate_until_moves(next_game, ends_reached, max_ends, prune)?;
            match prune_index {
                Some(index) => {
                    // println!("Pruned at depth {} for move {}, while at simulation depth {}", index, i, simulation_depth);
                    if index < simulation_depth {
                        // println!("would have returned here");
                        self.state_stack.pop();
                        // println!("Pruned at depth {} for move {}", index, i);
                        return Ok((ends_reached, Some(index)));
                    }
                },
                None => ()
            }
        }
        self.state_stack.pop();

        if skipped_all {
            return Ok((ends_reached + 1, None));
        }

        Ok((ends_reached, None))
    }

    pub fn save_data(&self, game: MancalaGame, parent_game: MancalaGame, move_pit: Pit, simulation_depth: usize) -> io::Result<Option<usize>> { // returns a bool representing if the state gets pruned later on
        let state_code = MancalaCodec::encode_game(&game, None).expect("Failed to encode game");
        let parent_code = MancalaCodec::encode_game(&parent_game, Some(move_pit)).expect("Failed to encode game");
        let file_path = FilePathHandler::generate_path(state_code);

        if !FilePathHandler::generate_path(parent_code).exists() { // If parent doesn't exist, exit
            return Err(io::Error::new(io::ErrorKind::NotFound, "Parent file not found"));
        }

        if !file_path.exists() && state_code != EMPTY_BOARD_CODE {
            let game_state = MancalaStateFile::new_empty(state_code, Some(parent_code));
            return match game_state.save_as_new() {
                Ok(_) => Ok(None),
                Err(e) => Err(e)
            };
        } else { // file already exists
            let parent_turn = parent_game.get_current_turn().unwrap();
            let current_turn = match state_code {
                EMPTY_BOARD_CODE => parent_turn,
                _ => game.get_current_turn().unwrap()
            };
            return self.begin_forward_data_if_needed(
                state_code,
                game.get_pit_pebbles(current_turn, MANCALA_PIT),
                parent_code,
                parent_game.get_pit_pebbles(parent_turn, MANCALA_PIT),
                simulation_depth
            );
        }
    }

    fn begin_forward_data_if_needed(&self, current_code: EncodedGameState, current_mancala: Pebbles, parent_code: EncodedGameState, parent_mancala: Pebbles, simulation_depth: usize) -> io::Result<Option<usize>> {
        let mut current_state = match current_code {
            EMPTY_BOARD_CODE => MancalaStateFile::new_empty(current_code, Some(parent_code)),
            _ => MancalaStateFile::new_from_read_file(current_code)?
        };
        let parent_included = current_state.append_parent_if_needed(parent_code)?;

        if current_code == EMPTY_BOARD_CODE {
            return self.endgame_forward_data(
                current_state,
                parent_mancala,
                current_mancala,
                simulation_depth
            );
        } else {
            if (current_state.get_completion() == StateCompletion::Incomplete) && !parent_included {
                return Ok(None);
            }
            let (_, prune_index) = self.forward_data_recursion(current_state, simulation_depth)?;
            return Ok(prune_index);
        }
    }

    fn endgame_forward_data(&self, current_state: MancalaStateFile, parent_mancala: Pebbles, endgame_mancala: Pebbles, simulation_depth: usize) -> io::Result<Option<usize>> {
        let parent_code= current_state.get_parents()[0]; // only one parent should be expected if game end has been reached

        let mut parent_state = MancalaStateFile::new_from_read_file(parent_code)?;

        let move_pit = MancalaCodec::get_move_from_game_code(parent_code).unwrap();

        if parent_state.get_hint_type(move_pit) == HintType::Completed {
            return Ok(None);
        }

        let pebbles_move: Pebbles = endgame_mancala - parent_mancala;
        
        let new_hint = Hint::new(pebbles_move, HintType::Completed, pebbles_move, pebbles_move);
        parent_state.save_updated_hint(move_pit, new_hint)?;

        let (_, prune_index) = self.forward_data_recursion(parent_state, simulation_depth)?;
        Ok(prune_index)
    }

    fn forward_data_recursion(&self, current_state: MancalaStateFile, simulation_depth: usize) -> io::Result<(bool, Option<usize>)> {
        let base_current_best_hint_value = current_state.get_best_hint_value();
        let mut global_pruned = true;
        let mut all_completed = true;
        let mut prune_index = None;
        let stack_code = match self.state_stack.len() {
            0 => EMPTY_BOARD_CODE,
            _ => self.state_stack[simulation_depth].get_code()
        };

        for parent_code in current_state.get_parents() {
            let mut parent_state = if MancalaCodec::are_from_same_state(*parent_code, stack_code) {
                MancalaStateFile::new_from_read_file(*parent_code)?
            } else {
                MancalaStateFile::new_from_read_file(*parent_code)?
            };
            let move_pit = MancalaCodec::get_move_from_game_code(*parent_code).unwrap();

            if parent_state.get_hint_type(move_pit) == HintType::Completed {
                continue;
            }

            all_completed &= parent_state.get_hint_type(move_pit) == HintType::PrunedCompleted;

            let current_pebbles = MancalaCodec::get_pebbles_on_game_code(current_state.get_code());
            let parent_pebbles = MancalaCodec::get_pebbles_on_game_code(*parent_code);
            let pebbles_diff = parent_pebbles - current_pebbles;
            let free_turn_ind = Self::get_free_turn_ind(*parent_code);

            let current_best_hint_value = Self::swap_hint_value_if_needed(current_pebbles, free_turn_ind, base_current_best_hint_value);
            
            let pebbles_move = current_best_hint_value + pebbles_diff;

            let min: u8;
            let max: u8;

            if free_turn_ind {
                min = current_state.get_min() + pebbles_diff;
                max = current_state.get_max() + pebbles_diff;
            } else {
                min = parent_pebbles - current_state.get_max();
                max = parent_pebbles - current_state.get_min();
            }

            let hint_type = match current_state.get_completion() {
                StateCompletion::Completed => HintType::Completed,
                StateCompletion::PrunedCompleted => HintType::PrunedCompleted,
                StateCompletion::Incomplete => if min == max { // quase ctz q posso deixar só normal aqui, mas vai q
                    HintType::PrunedCompleted
                } else {
                    HintType::Normal
                }
            };

            let new_hint = Hint::new(pebbles_move, hint_type, min, max);
            if new_hint.equals(&parent_state.get_hint(move_pit)) {
                continue;
            }

            let best_hint_before = parent_state.get_best_hint_value();
            
            let state_metadata_updated = parent_state.save_updated_hint(move_pit, new_hint)?;

            let best_hint_after = parent_state.get_best_hint_value();

            let mut local_pruned = new_hint.get_max() <= parent_state.get_min() && new_hint.get_type() == HintType::Normal;
            prune_index = if local_pruned && MancalaCodec::are_from_same_state(*parent_code, stack_code) { Some(simulation_depth) } else { None };

            if state_metadata_updated || best_hint_after != best_hint_before {
                let (forward_prune, forward_prune_index) = self.forward_data_recursion(
                    parent_state,
                    if simulation_depth > 0 { simulation_depth - 1 } else { 0 }
                )?;
                local_pruned |= forward_prune;
                match forward_prune_index {
                    Some(index) => {
                        prune_index = Some(index);
                    },
                    None => ()
                }
            }

            global_pruned &= local_pruned;
        }

        Ok((global_pruned && !all_completed, prune_index))
    }

    fn swap_hint_value_if_needed(board_pebbles: Pebbles, free_turn_ind: bool, hint_value: Pebbles) -> Pebbles {
        match free_turn_ind {
            true => hint_value,
            false => board_pebbles - hint_value,
        }
    }

    fn get_free_turn_ind(state_code: EncodedGameState) -> bool {
        state_code == EMPTY_BOARD_CODE || MancalaCodec::check_free_turn_on_game_code(state_code, None)
    }
}