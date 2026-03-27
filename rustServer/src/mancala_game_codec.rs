use crate::constants::*;
use crate::types::*;
use crate::mancala_game_model::MancalaGame;
use crate::error::CodecError;

const MAX_5_BIT_NUMBER: u8 = 31; // 2^5 - 1
const LAST_5_BITS_MASK: u64 = 31; // 0001 1111
const FIRST_4_BITS_MASK: u64 = 0xF000000000000000; // 1111 0000... (4 bits for move, 60 bits for board)

pub struct MancalaGameCodec;

impl MancalaGameCodec {
    pub fn encode(game: &MancalaGame, next_move: Option<Pit>) -> Result<EncodedGameState, CodecError> {
        // valida turno
        if game.get_current_turn() == None {
            return Ok(EMPTY_BOARD_CODE);
        }

        let mut code: EncodedGameState = 0;

        let players = match game.get_current_turn() { // start encoding from opposit player's perspective
            Some(PLAYER1) => [PLAYER2, PLAYER1],
            Some(PLAYER2) => [PLAYER1, PLAYER2],
            _ => return Err(CodecError::InvalidTurn),
        };

        for board_side in players {
            for pit in 1..=NUM_PITS { // ignore Mancala pits (0)
                let pebbles = game.get_pit_pebbles(board_side, pit);
                if pebbles > MAX_5_BIT_NUMBER {
                    return Err(CodecError::ValueOutOfRange { board_side, pit, value: pebbles });
                }

                code <<= BITS_PER_PIT; // shifting before so that the 1st time doesn't affect, so n-1 shifts
                code |= pebbles as EncodedGameState;
            }
        }

        if let Some(move_pit) = next_move {
            code &= !FIRST_4_BITS_MASK; // clear first 4 bits
            code |= (move_pit as EncodedGameState) << TOTAL_CODE_BITS; // set move in first 4 bits
        }

        Ok(code)
    }

    // pub fn decode(code: EncodedGameState, current_turn: MancalaTurn) -> Result<MancalaGame, CodecError> {
    //     let mut code: EncodedGameState = code;

    //     if current_turn == None {
    //         return Err(CodecError::GameEnded);
    //     }

    //     let mut board: MancalaBoard = [[0u8; 7]; 2];

    //     let players = match current_turn {
    //         Some(PLAYER1) => [PLAYER1, PLAYER2],
    //         Some(PLAYER2) => [PLAYER2, PLAYER1],
    //         None => return Err(CodecError::GameEnded),
    //         _ => return Err(CodecError::InvalidTurn),
    //     };

    //     for board_side in players {
    //         for pit in (1..=6).rev() {
                
    //             let pebbles = (code & LAST_5_BITS_MASK) as u8;
    //             board[board_side][pit] = pebbles;
    //             code >>= 5;
    //         }
    //     }
    //     Ok(MancalaGame::new_from_game(board, current_turn))
    // }

    pub fn get_move_from_code(code: EncodedGameState) -> Option<Pit> {
        let move_pit = ((code & FIRST_4_BITS_MASK) >> TOTAL_CODE_BITS) as Pit;
        if move_pit != 0 {
            Some(move_pit)
        } else {
            None
        }
    }

    pub fn get_pebbles_on_code_pit(state_code: EncodedGameState, pit: Pit) -> Pebbles {
        ((state_code >> (BITS_PER_PIT * (NUM_PITS - pit) as u8)) & LAST_5_BITS_MASK) as Pebbles
    }

    pub fn check_free_turn_on_code(game_code: EncodedGameState, move_pit: Option<Pit>) -> bool {
        let move_pit = match move_pit {
            Some(p) => p,
            None => match Self::get_move_from_code(game_code) { // If move_pit is None, it should be extracted from the code
                Some(p) => p,
                None => return false, // if no move, can't be free turn
            },
        };
        let pit_pebbles = Self::get_pebbles_on_code_pit(game_code, move_pit);
        (pit_pebbles % 13) as i8 - move_pit as i8 == 0
    }

    pub fn get_pebbles_on_code(game_code: EncodedGameState) -> Pebbles {
        let mut pebbles: Pebbles = 0;
        for i in 0..12 {
            pebbles += ((game_code >> (BITS_PER_PIT * (11 - i))) & LAST_5_BITS_MASK) as Pebbles;
        }
        pebbles
    }
}

#[cfg(test)]
mod encode_tests {
    use super::*;

    #[test]
    fn new_board() {
        let game = MancalaGame::new_from_game(
            [
                [0, 4, 4, 4, 4, 4, 4],
                [0, 4, 4, 4, 4, 4, 4],
            ],
            Some(PLAYER1)
        );
        let next_move = Some(4);
        let expected_code: EncodedGameState = 0x4210842108421084;

        let code = MancalaGameCodec::encode(&game, next_move).unwrap();
        assert_eq!(code, expected_code, "Encoded game does not match expected code");
    }

    #[test]
    fn move_1() {
        let original_game = MancalaGame::new_from_game(
            [
                [1, 5, 5, 5, 0, 4, 4],
                [0, 4, 4, 4, 4, 4, 4],
            ],
            Some(PLAYER1)
        );
        let next_move = Some(1);
        let expected_code: EncodedGameState = 0x121084210A528084;

        let code = MancalaGameCodec::encode(&original_game, next_move).unwrap();
        assert_eq!(code, expected_code, "Encoded game does not match expected code");
    }

    #[test]
    fn move_2() {
        let original_game = MancalaGame::new_from_game(
            [
                [2, 0, 5, 5, 0, 4, 4],
                [0, 4, 4, 5, 5, 5, 5],
            ],
            Some(PLAYER2)
        );
        let next_move = Some(5);
        let expected_code: EncodedGameState = 0x5014A021084294A5;

        let code = MancalaGameCodec::encode(&original_game, next_move).unwrap();
        assert_eq!(code, expected_code, "Encoded game does not match expected code");
    }

    #[test]
    fn some_board1() {
        let original_game = MancalaGame::new_from_game(
            [
                [0, 0, 1, 9, 0, 4, 1],
                [0, 10, 1, 1, 0, 7, 0],
            ],
            Some(PLAYER1)
        );
        let next_move = None;
        let expected_code: EncodedGameState = 0x0504203800148081;

        let code = MancalaGameCodec::encode(&original_game , next_move).unwrap();
        assert_eq!(code, expected_code, "Encoded game does not match expected code");
    }

    #[test]
    fn some_board2() {
        let original_game = MancalaGame::new_from_game(
            [
                [0, 0, 0, 1, 0, 2, 1],
                [0, 14, 1, 0, 1, 0, 1],
            ],
            Some(PLAYER2)
        );
        let next_move = None;
        let expected_code: EncodedGameState = 0x000020105C100401;

        let code = MancalaGameCodec::encode(&original_game, next_move).unwrap();
        assert_eq!(code, expected_code, "Encoded game does not match expected code");
    }
}

// #[cfg(test)]
// mod decode_tests {
//     use super::*;

//     #[test]
//     fn new_board() {
//         let original_code: EncodedGameState = 0x0210842108421084;
//         let expected_game = MancalaGame::new_from_game(
//             [
//                 [0, 4, 4, 4, 4, 4, 4],
//                 [0, 4, 4, 4, 4, 4, 4],
//             ],
//             Some(PLAYER1)
//         );

//         let decoded_game = MancalaGameCodec::decode(original_code, Some(PLAYER1)).unwrap();
//         assert_eq!(expected_game, decoded_game, "Decoded game does not match expected game");
//     }

//     #[test]
//     fn move_1() {
//         let original_code: EncodedGameState = 0x021084210A528084;
//         let expected_game = MancalaGame::new_from_game(
//             [
//                 [1, 5, 5, 5, 0, 4, 4],
//                 [0, 4, 4, 4, 4, 4, 4],
//             ],
//             Some(PLAYER1)
//         );

//         let decoded_game = MancalaGameCodec::decode(original_code, Some(PLAYER1)).unwrap();
//         assert_eq!(expected_game, decoded_game, "Decoded game does not match expected game");
//     }

//     #[test]
//     fn move_2() {
//         let original_code: EncodedGameState = 0x0014A021084294A5;
//         let expected_game = MancalaGame::new_from_game(
//             [
//                 [2, 0, 5, 5, 0, 4, 4],
//                 [0, 4, 4, 5, 5, 5, 5],
//             ],
//             Some(PLAYER2)
//         );
        
//         let decoded_game = MancalaGameCodec::decode(original_code, Some(PLAYER2)).unwrap();
//         assert_eq!(expected_game, decoded_game, "Decoded game does not match expected game");
//     }

//     #[test]
//     fn some_board1() {
//         let original_code: EncodedGameState = 0x0504203800148081;
//         let expected_game = MancalaGame::new_from_game(
//             [
//                 [0, 0, 1, 9, 0, 4, 1],
//                 [0, 10, 1, 1, 0, 7, 0],
//             ],
//             Some(PLAYER1)
//         );

//         let decoded_game = MancalaGameCodec::decode(original_code, Some(PLAYER1)).unwrap();
//         assert_eq!(expected_game, decoded_game, "Decoded game does not match expected game");
//     }

//     #[test]
//     fn some_board2() {
//         let original_code: EncodedGameState = 0x000020105C100401;
//         let expected_game = MancalaGame::new_from_game(
//             [
//                 [0, 0, 0, 1, 0, 2, 1],
//                 [0, 14, 1, 0, 1, 0, 1],
//             ],
//             Some(PLAYER2)
//         );

//         let decoded_game = MancalaGameCodec::decode(original_code, Some(PLAYER2)).unwrap();
//         assert_eq!(expected_game, decoded_game, "Decoded game does not match expected game");
//     }
// }

#[cfg(test)]
mod get_move_from_code_tests {
    use super::*;

    #[test]
    fn new_board() {
        let board_code: EncodedGameState = 0x4210842108421084;
        let move_pit = MancalaGameCodec::get_move_from_code(board_code);
        assert_eq!(move_pit, Some(4), "Code with first hex as 4 should return move 4");
    }

    #[test]
    fn move_1() {
        let board_code: EncodedGameState = 0x121084210A528084;
        let move_pit = MancalaGameCodec::get_move_from_code(board_code);
        assert_eq!(move_pit, Some(1), "Code with first hex as 1 should return move 1");
    }

    #[test]
    fn move_2() {
        let board_code: EncodedGameState = 0x5014A021084294A5;
        let move_pit = MancalaGameCodec::get_move_from_code(board_code);
        assert_eq!(move_pit, Some(5), "Code with first hex as 5 should return move 5");
    }

    #[test]
    fn no_move() {
        let board_code: EncodedGameState = 0x0014A021084294A5; // first hex is 0
        let move_pit = MancalaGameCodec::get_move_from_code(board_code);
        assert_eq!(move_pit, None, "Code with first hex as 0 should return None for move");
    }
}

#[cfg(test)]
mod check_free_turn_on_code_tests {
    use super::*;

    #[test]
    fn new_board() {
        let board_code: EncodedGameState = 0x0210842108421084;
        assert!(MancalaGameCodec::check_free_turn_on_code(board_code, Some(4)), "Move on pit 4 should result in free turn");
    }

    #[test]
    fn move_1() {
        let board_code: EncodedGameState = 0x021084210A528084;
        assert!(!MancalaGameCodec::check_free_turn_on_code(board_code, Some(1)), "Move on pit 1 should not result in free turn");
    }

    #[test]
    fn move_2() {
        let board_code: EncodedGameState = 0x0014A021084294A5;
        assert!(MancalaGameCodec::check_free_turn_on_code(board_code, Some(5)), "Move on pit 5 should result in free turn");
    }

    #[test]
    fn some_board() {
        let board_code: EncodedGameState = 0x000020105C100401;
        assert!(MancalaGameCodec::check_free_turn_on_code(board_code, Some(1)), "Move on pit 1 should result in free turn");
    }

    #[test]
    fn new_board_w_move() {
        let board_code: EncodedGameState = 0x4210842108421084;
        assert!(MancalaGameCodec::check_free_turn_on_code(board_code, None), "Move on pit 4 should result in free turn");
    }

    #[test]
    fn some_board_w_no_move() {
        let board_code: EncodedGameState = 0x000020105C100401;
        assert!(!MancalaGameCodec::check_free_turn_on_code(board_code, None), "No move passed and no move on code should return false");
    }
}

#[cfg(test)]
mod get_pebbles_on_code_tests {
    use super::*;

    #[test]
    fn new_board() {
        let code: EncodedGameState = 0x0210842108421084;
        let expected_pebbles: Pebbles = 48;
        assert_eq!(expected_pebbles, MancalaGameCodec::get_pebbles_on_code(code), "Pebbles count from code does not match expected pebbles");
    }

    #[test]
    fn move_1() {
        let code: EncodedGameState = 0x021084210A528084;
        let expected_pebbles: Pebbles = 47;
        assert_eq!(expected_pebbles, MancalaGameCodec::get_pebbles_on_code(code), "Pebbles count from code does not match expected pebbles");
    }

    #[test]
    fn move_2() {
        let code: EncodedGameState = 0x0014A021084294A5;
        let expected_pebbles: Pebbles = 46;
        assert_eq!(expected_pebbles, MancalaGameCodec::get_pebbles_on_code(code), "Pebbles count from code does not match expected pebbles");
    }

    #[test]
    fn some_board1() {
        let code: EncodedGameState = 0x0504203800148081;
        let expected_pebbles: Pebbles = 34;
        assert_eq!(expected_pebbles, MancalaGameCodec::get_pebbles_on_code(code), "Pebbles count from code does not match expected pebbles");
    }

    #[test]
    fn some_board2() {
        let code: EncodedGameState = 0x000020105C100401;
        let expected_pebbles: Pebbles = 21;
        assert_eq!(expected_pebbles, MancalaGameCodec::get_pebbles_on_code(code), "Pebbles count from code does not match expected pebbles");
    }
}