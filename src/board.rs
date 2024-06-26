//! The board module deals with basic board representation.
//! It contains the important position and bitboard submodules, as well other useful ones such as color, file, rank, and square.
//! This module is the foundation on which the rest of the engine builds upon.

use arrayvec::ArrayVec;
use position::Position;
use crate::board::color::Color;
use crate::board::piece::Piece;
use crate::move_gen::ply::Ply;

pub mod bitboard;
pub mod color;
pub mod file;
pub mod rank;
pub mod square;
pub mod castling_rights;
pub mod piece;
pub mod position;
pub mod fen;

/// The board struct holds the current position of the board.
/// It also keeps track of the full move counter, the halfmove clock (50 move rule),
/// and a list of all positions that have been on the board before (threefold repetition).
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Board {
    /// The current position of the chess board.
    pub position: Position,
    /// The current full move count (incremented after Black's play).
    pub fullmove_counter: u32,
    /// The number of reversible ply (no pawn moves or captures).
    pub halfmove_clock: u32,
}

impl Default for Board {
    /// Default constructor for Board.
    /// Returns a board with default values.
    fn default() -> Self {
        Self {
            position: Position::default(),
            halfmove_clock: 0,
            fullmove_counter: 1,
        }
    }
}

impl Board {
    /// Constructs a new board from a FEN string.
    /// If the FEN could be parsed successfully, the result will contain the newly constructed board.
    /// Otherwise, it will contain an error.
    pub fn from_fen(fen: &str) -> Result<Board, String> {
        Self::parse_fen(fen)
    }

    /// Returns a new board that reflects the board state where the given move (ply) has been played.
    pub fn make_move(&self, ply: Ply) -> Board {
        let mut board = *self;
        
        board.position = board.position.make_move(ply);
        
        // update the halfmove clock
        if ply.piece != Piece::Pawn && ply.captured_piece.is_none() {
            // if the move is neither a pawn move nor a capture, increment the halfmove clock
            board.halfmove_clock += 1;
        } else {
            // otherwise, reset it
            board.halfmove_clock = 0;
        }
        
        // update the fullmove counter
        if self.position.color_to_move == Color::Black {
            board.fullmove_counter += 1;
        }
        
        board
    }
    
    /// Checks whether the position is a draw by either threefold repetition or the 50 move rule, based on the given board history.
    pub fn is_draw(&self, board_history: &ArrayVec<u64, 1000>) -> bool {
        // check for draw by 50 move role
        if self.halfmove_clock >= 100 {
            return true;
        }
        
        if board_history.is_empty() {
            return false;
        }
        
        // loop over the board history from the end, but go no further back than the halfmove clock
        // (captures and pawn moves reset the halfmove clock, so we don't have to look any further)
        let mut repetition_count = 0;
        let mut i = 0;
        for hash in board_history.iter().rev() {
            if *hash == self.position.hash {
                repetition_count += 1;
            }
            i += 1;
            if i > self.halfmove_clock {
                break;
            }
        }
        repetition_count >= 3
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::field_reassign_with_default)]
    
    use arrayvec::ArrayVec;
    use crate::board::bitboard::Bitboard;
    use crate::board::{Board, square};
    use crate::board::castling_rights::CastlingRights;
    use crate::board::color::Color::{Black, White};
    use crate::board::piece::Piece;
    use crate::board::position::Position;
    use crate::lookup::LOOKUP_TABLE;
    use crate::lookup::lookup_table::LookupTable;
    use crate::move_gen::ply::Ply;
    use crate::zobrist;

    #[test]
    fn default_returns_board_with_default_values() {
        let mut lookup = LookupTable::default();
        lookup.initialize_tables();
        let _ = LOOKUP_TABLE.set(lookup);
        
        let board = Board::default();
        assert_eq!(Position::default(), board.position);
        assert_eq!(0, board.halfmove_clock);
        assert_eq!(1, board.fullmove_counter);
    }

    #[test]
    fn from_fen_with_valid_fen_returns_board() {
        let mut lookup = LookupTable::default();
        lookup.initialize_tables();
        let _ = LOOKUP_TABLE.set(lookup);

        // -----------------------------------------------------------------------------------------
        // position 1
        // -----------------------------------------------------------------------------------------

        let board = Board::from_fen("3r2k1/ppp2p1p/3p2p1/3P2P1/5P2/4r3/P1B3P1/5RK1 w - - 0 34").unwrap();
        // expected piece bitboards of the resulting position
        let bitboards = [
            [Bitboard::new(0x4820004100), Bitboard::new(0), Bitboard::new(0x400), Bitboard::new(0x20), Bitboard::new(0), Bitboard::new(0x40)],
            [Bitboard::new(0xa7480000000000), Bitboard::new(0), Bitboard::new(0), Bitboard::new(0x800000000100000), Bitboard::new(0), Bitboard::new(0x4000000000000000)]
        ];
        assert_eq!(bitboards, board.position.pieces);
        assert_eq!(White, board.position.color_to_move);
        assert_eq!([CastlingRights::NoRights; 2], board.position.castling_rights);
        assert_eq!(None, board.position.en_passant);
        assert_eq!(0, board.halfmove_clock);
        assert_eq!(34, board.fullmove_counter);

        // -----------------------------------------------------------------------------------------
        // position 2
        // -----------------------------------------------------------------------------------------

        let board = Board::from_fen("r2qk2r/pp3Qpp/2n1p3/3pN1b1/3P4/2P5/PP3PPP/RN2K2R b KQkq - 0 13").unwrap();
        // expected piece bitboards of the resulting position
        let bitboards = [
            [Bitboard::new(0x804e300), Bitboard::new(0x1000000002), Bitboard::new(0), Bitboard::new(0x81), Bitboard::new(0x20000000000000), Bitboard::new(0x10)],
            [Bitboard::new(0xc3100800000000), Bitboard::new(0x40000000000), Bitboard::new(0x4000000000), Bitboard::new(0x8100000000000000), Bitboard::new(0x800000000000000), Bitboard::new(0x1000000000000000)]
        ];
        assert_eq!(bitboards, board.position.pieces);
        assert_eq!(Black, board.position.color_to_move);
        assert_eq!([CastlingRights::Both; 2], board.position.castling_rights);
        assert_eq!(None, board.position.en_passant);
        assert_eq!(0, board.halfmove_clock);
        assert_eq!(13, board.fullmove_counter);

        // -----------------------------------------------------------------------------------------
        // position 3
        // -----------------------------------------------------------------------------------------

        let board = Board::from_fen("4k3/1pp4r/2p2P1b/p1n1P1Rp/6p1/P1N3P1/1PP4P/2K5 w - - 4 29").unwrap();
        // expected piece bitboards of the resulting position
        let bitboards = [
            [Bitboard::new(0x201000418600), Bitboard::new(0x40000), Bitboard::new(0), Bitboard::new(0x4000000000), Bitboard::new(0), Bitboard::new(0x4)],
            [Bitboard::new(0x6048140000000), Bitboard::new(0x400000000), Bitboard::new(0x800000000000), Bitboard::new(0x80000000000000), Bitboard::new(0), Bitboard::new(0x1000000000000000)]
        ];
        assert_eq!(bitboards, board.position.pieces);
        assert_eq!(White, board.position.color_to_move);
        assert_eq!([CastlingRights::NoRights; 2], board.position.castling_rights);
        assert_eq!(None, board.position.en_passant);
        assert_eq!(4, board.halfmove_clock);
        assert_eq!(29, board.fullmove_counter);
    }
    
    #[test]
    fn test_make_move() {
        let mut lookup = LookupTable::default();
        lookup.initialize_tables();
        let _ = LOOKUP_TABLE.set(lookup);
        
        // g1-f3
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        let board = board.make_move(Ply {
            source: square::G1,
            target: square::F3,
            piece: Piece::Knight,
            captured_piece: None,
            promotion_piece: None,
        });
        assert_eq!(Board::from_fen("rnbqkbnr/pppppppp/8/8/8/5N2/PPPPPPPP/RNBQKB1R b KQkq - 1 1").unwrap(), board);

        // b8-c6
        let board = board.make_move(Ply {
            source: square::B8,
            target: square::C6,
            piece: Piece::Knight,
            captured_piece: None,
            promotion_piece: None,
        });
        assert_eq!(Board::from_fen("r1bqkbnr/pppppppp/2n5/8/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 2 2").unwrap(), board);

        // b1-c3
        let board = board.make_move(Ply {
            source: square::B1,
            target: square::C3,
            piece: Piece::Knight,
            captured_piece: None,
            promotion_piece: None,
        });
        assert_eq!(Board::from_fen("r1bqkbnr/pppppppp/2n5/8/8/2N2N2/PPPPPPPP/R1BQKB1R b KQkq - 3 2").unwrap(), board);

        // g8-f6
        let board = board.make_move(Ply {
            source: square::G8,
            target: square::F6,
            piece: Piece::Knight,
            captured_piece: None,
            promotion_piece: None,
        });
        assert_eq!(Board::from_fen("r1bqkb1r/pppppppp/2n2n2/8/8/2N2N2/PPPPPPPP/R1BQKB1R w KQkq - 4 3").unwrap(), board);

        // f3-e5
        let board = board.make_move(Ply {
            source: square::F3,
            target: square::E5,
            piece: Piece::Knight,
            captured_piece: None,
            promotion_piece: None,
        });
        assert_eq!(Board::from_fen("r1bqkb1r/pppppppp/2n2n2/4N3/8/2N5/PPPPPPPP/R1BQKB1R b KQkq - 5 3").unwrap(), board);

        // c6-e5
        let board = board.make_move(Ply {
            source: square::C6,
            target: square::E5,
            piece: Piece::Knight,
            captured_piece: Some(Piece::Knight),
            promotion_piece: None,
        });
        assert_eq!(Board::from_fen("r1bqkb1r/pppppppp/5n2/4n3/8/2N5/PPPPPPPP/R1BQKB1R w KQkq - 0 4").unwrap(), board);

        // d2-d4
        let board = board.make_move(Ply {
            source: square::D2,
            target: square::D4,
            piece: Piece::Pawn,
            captured_piece: None,
            promotion_piece: None,
        });
        assert_eq!(Board::from_fen("r1bqkb1r/pppppppp/5n2/4n3/3P4/2N5/PPP1PPPP/R1BQKB1R b KQkq d3 0 4").unwrap(), board);

        // e5-c6
        let board = board.make_move(Ply {
            source: square::E5,
            target: square::C6,
            piece: Piece::Knight,
            captured_piece: None,
            promotion_piece: None,
        });
        assert_eq!(Board::from_fen("r1bqkb1r/pppppppp/2n2n2/8/3P4/2N5/PPP1PPPP/R1BQKB1R w KQkq - 1 5").unwrap(), board);

        // e5-c6
        let board = board.make_move(Ply {
            source: square::D4,
            target: square::D5,
            piece: Piece::Pawn,
            captured_piece: None,
            promotion_piece: None,
        });
        assert_eq!(Board::from_fen("r1bqkb1r/pppppppp/2n2n2/3P4/8/2N5/PPP1PPPP/R1BQKB1R b KQkq - 0 5").unwrap(), board);
    }
    
    #[test]
    fn test_is_draw() {
        let mut lookup = LookupTable::default();
        lookup.initialize_tables();
        let _ = LOOKUP_TABLE.set(lookup);
        
        let mut board_history: ArrayVec<u64, 1000> = ArrayVec::new();
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 3 1").unwrap();
        
        board_history.push(zobrist::get_hash(&board.position));
        board_history.push(zobrist::get_hash(&board.position));
        assert!(!board.is_draw(&board_history));

        board_history.push(zobrist::get_hash(&board.position));
        assert!(board.is_draw(&board_history));
        
        board_history.pop();
        
        let mut board = Board::default();
        board.halfmove_clock = 99;
        assert!(!board.is_draw(&board_history));
        board.halfmove_clock = 100;
        assert!(board.is_draw(&board_history));
    }
}
