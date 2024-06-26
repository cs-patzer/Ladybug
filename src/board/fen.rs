use crate::board::bitboard::Bitboard;
use crate::board::Board;
use crate::board::castling_rights::CastlingRights;
use crate::board::color::{Color, NUM_COLORS};
use crate::board::color::Color::{Black, White};
use crate::board::file::{File, NUM_FILES};
use crate::board::piece::Piece::{Bishop, King, Knight, Pawn, Queen, Rook};
use crate::board::position::Position;
use crate::board::rank::{NUM_RANKS, Rank};
use crate::board::square::Square;

impl Board {
    /// Parses a [FEN](https://www.chessprogramming.org/Forsyth-Edwards_Notation) string and returns a result.
    /// If the FEN could be parsed successfully, the result will contain a board. Otherwise, it will contain an error.
    pub(super) fn parse_fen(fen: &str) -> Result<Board, String> {
        // split fen into its six parts
        let fen_parts = Self::split_fen(fen)?;

        // parse pieces
        let pieces = Self::parse_pieces(fen_parts.first().unwrap())?;

        // parse color to move
        let color_to_move = Self::parse_color_to_move(fen_parts.get(1).unwrap())?;

        // parse castling rights
        let castling_rights = Self::parse_castling_rights(fen_parts.get(2).unwrap())?;

        // parse en passant
        let en_passant = Self::parse_en_passant(fen_parts.get(3).unwrap())?;

        // parse halfmove clock
        let halfmove_clock = Self::parse_halfmove_clock(fen_parts.get(4).unwrap())?;

        // parse fullmove counter
        let fullmove_counter = Self::parse_fullmove_counter(fen_parts.last().unwrap())?;

        // create the position
        let position = Position::new(
            pieces,
            castling_rights,
            en_passant,
            color_to_move,
        );

        // create the board
        let board = Board {
            position,
            fullmove_counter,
            halfmove_clock,
        };

        Ok(board)
    }

    /// Builds a FEN string representing the board state.
    pub fn to_fen(&self) -> String {
        let mut fen = String::from("");

        // pieces
        for rank in (0..NUM_RANKS).rev() {
            let mut files_to_skip = 0;
            for file in 0..NUM_FILES {
                let piece = self.position.get_piece(Square::from_file_rank(File::from_index(file), Rank::from_index(rank)));
                match piece {
                    Some((piece, color)) => {
                        if files_to_skip > 0 {
                            fen.push_str(format!("{files_to_skip}").as_str());
                            files_to_skip = 0;
                        }
                        fen.push(piece.to_char(color));
                    }
                    None => {
                        files_to_skip += 1;
                    }
                }
            }
            if files_to_skip > 0 {
                fen.push_str(format!("{files_to_skip}").as_str());
            }
            if rank != 0 {
                fen.push('/');
            }
        }

        // color
        match self.position.color_to_move {
            White => fen.push_str(" w"),
            Black => fen.push_str(" b"),
        }

        // castling rights
        let mut castling_rights_str_both = String::from("");
        for color_index in 0..NUM_COLORS {
            let mut castling_rights_str = String::from("");
            match self.position.castling_rights[color_index as usize] {
                CastlingRights::NoRights => {}
                CastlingRights::KingSide => castling_rights_str.push('K'),
                CastlingRights::QueenSide => castling_rights_str.push('Q'),
                CastlingRights::Both => castling_rights_str.push_str("KQ"),
            }
            if Color::from_index(color_index) == Black {
                castling_rights_str = castling_rights_str.to_ascii_lowercase();
            }
            castling_rights_str_both += castling_rights_str.as_str();
        }
        match castling_rights_str_both.as_str() {
            "" => fen.push_str(" -"),
            other => {
                fen.push_str(format!(" {other}").as_str());
            }
        }

        // en passant
        match self.position.en_passant {
            None => fen.push_str(" -"),
            Some(square) => fen.push_str(format!(" {square}").as_str()),
        }

        // halfmove clock
        fen.push_str(format!(" {}", self.halfmove_clock).as_str());

        // fullmove counter
        fen.push_str(format!(" {}", self.fullmove_counter).as_str());

        fen
    }

    /// Takes a FEN and splits it into its 6 parts.
    /// If the FEN has more than 4 but less than 6 parts, default parameters will be added for the remaining parts.
    fn split_fen(fen: &str) -> Result<Vec<String>, String> {
        let mut fen_parts: Vec<String> = fen.split_whitespace().map(|s| s.to_string()).collect();
        match fen_parts.len() {
            4 => {
                fen_parts.push(String::from("0"));
                fen_parts.push(String::from("1"));
                Ok(fen_parts)
            }
            5 => {
                fen_parts.push(String::from("1"));
                Ok(fen_parts)
            }
            6 => Ok(fen_parts),
            _other => Err(String::from("Invalid FEN")),
        }
    }

    /// Parses the first part of the FEN (pieces).
    fn parse_pieces(piece_fen: &str) -> Result<[[Bitboard; 6]; 2], String> {
        let mut pieces = [[Bitboard::new(0); 6]; 2];
        let piece_parts: Vec<String> = piece_fen.split('/').map(|s| s.to_string()).collect();
        if piece_parts.len() != 8 {
            return Err(String::from("Invalid FEN"));
        }
        for (rank_index, piece_str) in piece_parts.iter().enumerate() {
            let mut file_index: usize = 0;
            for char in piece_str.chars() {
                match char {
                    'p' => pieces[Black.to_index() as usize][Pawn.to_index() as usize].set_bit(Square::from_file_rank(File::from_index(file_index as u8), Rank::from_index(7 - rank_index as u8))),
                    'n' => pieces[Black.to_index() as usize][Knight.to_index() as usize].set_bit(Square::from_file_rank(File::from_index(file_index as u8), Rank::from_index(7 - rank_index as u8))),
                    'b' => pieces[Black.to_index() as usize][Bishop.to_index() as usize].set_bit(Square::from_file_rank(File::from_index(file_index as u8), Rank::from_index(7 - rank_index as u8))),
                    'r' => pieces[Black.to_index() as usize][Rook.to_index() as usize].set_bit(Square::from_file_rank(File::from_index(file_index as u8), Rank::from_index(7 - rank_index as u8))),
                    'q' => pieces[Black.to_index() as usize][Queen.to_index() as usize].set_bit(Square::from_file_rank(File::from_index(file_index as u8), Rank::from_index(7 - rank_index as u8))),
                    'k' => pieces[Black.to_index() as usize][King.to_index() as usize].set_bit(Square::from_file_rank(File::from_index(file_index as u8), Rank::from_index(7 - rank_index as u8))),
                    'P' => pieces[White.to_index() as usize][Pawn.to_index() as usize].set_bit(Square::from_file_rank(File::from_index(file_index as u8), Rank::from_index(7 - rank_index as u8))),
                    'N' => pieces[White.to_index() as usize][Knight.to_index() as usize].set_bit(Square::from_file_rank(File::from_index(file_index as u8), Rank::from_index(7 - rank_index as u8))),
                    'B' => pieces[White.to_index() as usize][Bishop.to_index() as usize].set_bit(Square::from_file_rank(File::from_index(file_index as u8), Rank::from_index(7 - rank_index as u8))),
                    'R' => pieces[White.to_index() as usize][Rook.to_index() as usize].set_bit(Square::from_file_rank(File::from_index(file_index as u8), Rank::from_index(7 - rank_index as u8))),
                    'Q' => pieces[White.to_index() as usize][Queen.to_index() as usize].set_bit(Square::from_file_rank(File::from_index(file_index as u8), Rank::from_index(7 - rank_index as u8))),
                    'K' => pieces[White.to_index() as usize][King.to_index() as usize].set_bit(Square::from_file_rank(File::from_index(file_index as u8), Rank::from_index(7 - rank_index as u8))),
                    '1' => (),
                    '2'..='8' => {
                        let files_to_skip = char.to_digit(10);
                        match files_to_skip {
                            Some(files_to_skip) => file_index += files_to_skip as usize - 1,
                            None => return Err(String::from("Invalid FEN")),
                        }
                    }
                    _other => return Err(String::from("Invalid FEN")),
                }
                if file_index > 7 {
                    // In a FEN string, pieces are specified using letters (P for a white pawn for example),
                    // while one or more empty squares are notated with a number (2 for two empty squares for example).
                    // If the file_index is larger than seven before the increment below,
                    // it means that the number of piece letters plus the sum of numbers used to notate empty squares was larger than 8.
                    // Since a chessboard only has 8 files, the FEN must be invalid.
                    return Err(String::from("Invalid FEN"));
                }
                file_index += 1;
            }
        }
        Ok(pieces)
    }

    /// Parses the second part of the FEN (color to move).
    fn parse_color_to_move(color_fen: &str) -> Result<Color, String> {
        match color_fen {
            "w" => Ok(White),
            "b" => Ok(Black),
            _other => Err(String::from("Invalid FEN")),
        }
    }

    /// Parses the third part of the FEN (castling rights).
    fn parse_castling_rights(castling_rights_fen: &str) -> Result<[CastlingRights; 2], String> {
        match castling_rights_fen {
            "-" => Ok([CastlingRights::NoRights, CastlingRights::NoRights]),
            "q" => Ok([CastlingRights::NoRights, CastlingRights::QueenSide]),
            "k" => Ok([CastlingRights::NoRights, CastlingRights::KingSide]),
            "kq" => Ok([CastlingRights::NoRights, CastlingRights::Both]),
            "Q" => Ok([CastlingRights::QueenSide, CastlingRights::NoRights]),
            "Qq" => Ok([CastlingRights::QueenSide, CastlingRights::QueenSide]),
            "Qk" => Ok([CastlingRights::QueenSide, CastlingRights::KingSide]),
            "Qkq" => Ok([CastlingRights::QueenSide, CastlingRights::Both]),
            "K" => Ok([CastlingRights::KingSide, CastlingRights::NoRights]),
            "Kq" => Ok([CastlingRights::KingSide, CastlingRights::QueenSide]),
            "Kk" => Ok([CastlingRights::KingSide, CastlingRights::KingSide]),
            "Kkq" => Ok([CastlingRights::KingSide, CastlingRights::Both]),
            "KQ" => Ok([CastlingRights::Both, CastlingRights::NoRights]),
            "KQq" => Ok([CastlingRights::Both, CastlingRights::QueenSide]),
            "KQk" => Ok([CastlingRights::Both, CastlingRights::KingSide]),
            "KQkq" => Ok([CastlingRights::Both, CastlingRights::Both]),
            _other => Err(String::from("Invalid FEN")),
        }
    }

    /// Parses the fourth part of the FEN (en passant).
    fn parse_en_passant(en_passant_fen: &str) -> Result<Option<Square>, String> {
        match en_passant_fen {
            "-" => Ok(None),
            other => {
                Square::from_string(other)
                    .map(Some)
                    .map_err(|_| String::from("Invalid FEN"))
            }
        }
    }

    /// Parses the fifth part of the FEN (halfmove clock).
    fn parse_halfmove_clock(halfmove_clock_fen: &str) -> Result<u32, String> {
        let halfmove_clock: Result<u32, _> = halfmove_clock_fen.parse();
        match halfmove_clock {
            Ok(halfmove_clock) => Ok(halfmove_clock),
            Err(_) => Err(String::from("Invalid FEN")),
        }
    }

    /// Parses the sixth part of the FEN (fullmove counter).
    fn parse_fullmove_counter(fullmove_counter_fen: &str) -> Result<u32, String> {
        let fullmove_counter: Result<u32, _> = fullmove_counter_fen.parse();
        match fullmove_counter {
            Ok(halfmove_clock) => match halfmove_clock {
                0 => Err(String::from("Invalid FEN")), // The fullmove counter starts at 1, so it can't be 0.
                other => Ok(other),
            }
            Err(_) => Err(String::from("Invalid FEN")),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::board::bitboard::Bitboard;
    use crate::board::castling_rights::CastlingRights;
    use crate::board::color::Color::{Black, White};
    use crate::board::piece::Piece::{Bishop, King, Knight, Pawn, Queen, Rook};
    use crate::board::{Board, square};
    use crate::lookup::LOOKUP_TABLE;
    use crate::lookup::lookup_table::LookupTable;

    #[test]
    fn parse_fen_with_valid_fen_returns_board() {
        let mut lookup = LookupTable::default();
        lookup.initialize_tables();
        let _ = LOOKUP_TABLE.set(lookup);

        // -----------------------------------------------------------------------------------------
        // Test the parse_fen function with a lot of different positions to make sure it's working.
        // -----------------------------------------------------------------------------------------

        // -----------------------------------------------------------------------------------------
        // position 1 (starting position)
        // -----------------------------------------------------------------------------------------

        let board = Board::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        // expected piece bitboards of the resulting position
        let bitboards = [
            [Bitboard::new(0xff00), Bitboard::new(0x42), Bitboard::new(0x24), Bitboard::new(0x81), Bitboard::new(0x8), Bitboard::new(0x10)],
            [Bitboard::new(0xff000000000000), Bitboard::new(0x4200000000000000), Bitboard::new(0x2400000000000000), Bitboard::new(0x8100000000000000), Bitboard::new(0x800000000000000), Bitboard::new(0x1000000000000000)]
        ];
        assert_eq!(bitboards, board.position.pieces);
        assert_eq!(White, board.position.color_to_move);
        assert_eq!([CastlingRights::Both; 2], board.position.castling_rights);
        assert_eq!(None, board.position.en_passant);
        assert_eq!(0, board.halfmove_clock);
        assert_eq!(1, board.fullmove_counter);

        // -----------------------------------------------------------------------------------------
        // position 2
        // -----------------------------------------------------------------------------------------

        let board = Board::parse_fen("2r3k1/1p4pp/8/p2NPp2/3PnB2/b4Q2/Pqr3PP/R4RK1 b - - 2 23").unwrap();
        // expected piece bitboards of the resulting position
        let bitboards = [
            [Bitboard::new(0x100800c100), Bitboard::new(0x800000000), Bitboard::new(0x20000000), Bitboard::new(0x21), Bitboard::new(0x200000), Bitboard::new(0x40)],
            [Bitboard::new(0xc2002100000000), Bitboard::new(0x10000000), Bitboard::new(0x10000), Bitboard::new(0x400000000000400), Bitboard::new(0x200), Bitboard::new(0x4000000000000000)]
        ];
        assert_eq!(bitboards, board.position.pieces);
        assert_eq!(Black, board.position.color_to_move);
        assert_eq!([CastlingRights::NoRights; 2], board.position.castling_rights);
        assert_eq!(None, board.position.en_passant);
        assert_eq!(2, board.halfmove_clock);
        assert_eq!(23, board.fullmove_counter);

        // -----------------------------------------------------------------------------------------
        // position 3
        // -----------------------------------------------------------------------------------------

        let board = Board::parse_fen("r1q3kr/5pQ1/1p1p2p1/p2P2PN/2P5/P7/1P5P/5RK1 b - - 4 33").unwrap();
        // expected piece bitboards of the resulting position
        let bitboards = [
            [Bitboard::new(0x4804018200), Bitboard::new(0x8000000000), Bitboard::new(0), Bitboard::new(0x20), Bitboard::new(0x40000000000000), Bitboard::new(0x40)],
            [Bitboard::new(0x204a0100000000), Bitboard::new(0), Bitboard::new(0), Bitboard::new(0x8100000000000000), Bitboard::new(0x400000000000000), Bitboard::new(0x4000000000000000)]
        ];
        assert_eq!(bitboards, board.position.pieces);
        assert_eq!(Black, board.position.color_to_move);
        assert_eq!([CastlingRights::NoRights; 2], board.position.castling_rights);
        assert_eq!(None, board.position.en_passant);
        assert_eq!(4, board.halfmove_clock);
        assert_eq!(33, board.fullmove_counter);

        // -----------------------------------------------------------------------------------------
        // position 4
        // -----------------------------------------------------------------------------------------

        let board = Board::parse_fen("2k2b1r/2qr1ppp/1pN1pn2/pBPp1b2/Q2P4/P1N5/1P3PPP/R1B1K2R w KQ a6 0 13").unwrap();
        // expected piece bitboards of the resulting position
        let bitboards = [
            [Bitboard::new(0x40801e200), Bitboard::new(0x40000040000), Bitboard::new(0x200000004), Bitboard::new(0x81), Bitboard::new(0x1000000), Bitboard::new(0x10)],
            [Bitboard::new(0xe0120900000000), Bitboard::new(0x200000000000), Bitboard::new(0x2000002000000000), Bitboard::new(0x8008000000000000), Bitboard::new(0x4000000000000), Bitboard::new(0x400000000000000)]
        ];
        assert_eq!(bitboards, board.position.pieces);
        assert_eq!(White, board.position.color_to_move);
        assert_eq!([CastlingRights::Both, CastlingRights::NoRights], board.position.castling_rights);
        assert_eq!(Some(square::A6), board.position.en_passant);
        assert_eq!(0, board.halfmove_clock);
        assert_eq!(13, board.fullmove_counter);
    }

    #[test]
    fn parse_fen_with_invalid_fen_returns_error() {
        assert_eq!(Err(String::from("Invalid FEN")), Board::parse_fen(""));
        assert_eq!(Err(String::from("Invalid FEN")), Board::parse_fen("Rust is awesome!"));
        assert_eq!(Err(String::from("Invalid FEN")), Board::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQKQ - 0 1"));
        assert_eq!(Err(String::from("Invalid FEN")), Board::parse_fen("rnbqkbnr/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 1"));
        assert_eq!(Err(String::from("Invalid FEN")), Board::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR B KQkq - 0 1"));
        assert_eq!(Err(String::from("Invalid FEN")), Board::parse_fen("rnbqkbnr/pppppppp/9/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"));
        assert_eq!(Err(String::from("Invalid FEN")), Board::parse_fen("rnbqkbnr/ppppp1ppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"));
    }

    #[test]
    fn split_fen_with_valid_fen_returns_vec_with_6_strings() {
        // starting position
        let fen_parts = Board::split_fen("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2").unwrap();
        assert_eq!(6, fen_parts.len());
        assert_eq!("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R", fen_parts[0]);
        assert_eq!("b", fen_parts[1]);
        assert_eq!("KQkq", fen_parts[2]);
        assert_eq!("-", fen_parts[3]);
        assert_eq!("1", fen_parts[4]);
        assert_eq!("2", fen_parts[5]);

        // starting position with missing full move counter
        let fen_parts = Board::split_fen("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1").unwrap();
        assert_eq!(6, fen_parts.len());
        assert_eq!("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R", fen_parts[0]);
        assert_eq!("b", fen_parts[1]);
        assert_eq!("KQkq", fen_parts[2]);
        assert_eq!("-", fen_parts[3]);
        assert_eq!("1", fen_parts[4]);
        assert_eq!("1", fen_parts[5]);

        // starting position with missing halfmove clock and full move counter
        let fen_parts = Board::split_fen("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq -").unwrap();
        assert_eq!(6, fen_parts.len());
        assert_eq!("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R", fen_parts[0]);
        assert_eq!("b", fen_parts[1]);
        assert_eq!("KQkq", fen_parts[2]);
        assert_eq!("-", fen_parts[3]);
        assert_eq!("0", fen_parts[4]);
        assert_eq!("1", fen_parts[5]);
    }

    #[test]
    fn split_fen_with_invalid_fen_returns_error() {
        assert_eq!(Err(String::from("Invalid FEN")), Board::split_fen("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq"));
        assert_eq!(Err(String::from("Invalid FEN")), Board::split_fen("one two three four five six seven"));
        assert_ne!(Err(String::from("Invalid FEN")), Board::split_fen("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2"));
    }

    #[test]
    fn parse_pieces_with_valid_fen_returns_piece_bitboards() {
        let pieces = Board::parse_pieces("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR").unwrap();
        assert_eq!(0xff000000000000, pieces[Black.to_index() as usize][Pawn.to_index() as usize].value); // black pawns
        assert_eq!(0x4200000000000000, pieces[Black.to_index() as usize][Knight.to_index() as usize].value); // black knights
        assert_eq!(0x2400000000000000, pieces[Black.to_index() as usize][Bishop.to_index() as usize].value); // black bishops
        assert_eq!(0x8100000000000000, pieces[Black.to_index() as usize][Rook.to_index() as usize].value); // black rooks
        assert_eq!(0x800000000000000, pieces[Black.to_index() as usize][Queen.to_index() as usize].value); // black queens
        assert_eq!(0x1000000000000000, pieces[Black.to_index() as usize][King.to_index() as usize].value); // black kings
        assert_eq!(0xff00, pieces[White.to_index() as usize][Pawn.to_index() as usize].value); // white pawns
        assert_eq!(0x42, pieces[White.to_index() as usize][Knight.to_index() as usize].value); // white knights
        assert_eq!(0x24, pieces[White.to_index() as usize][Bishop.to_index() as usize].value); // white bishops
        assert_eq!(0x81, pieces[White.to_index() as usize][Rook.to_index() as usize].value); // white rooks
        assert_eq!(0x8, pieces[White.to_index() as usize][Queen.to_index() as usize].value); // white queens
        assert_eq!(0x10, pieces[White.to_index() as usize][King.to_index() as usize].value); // white kings

        let pieces = Board::parse_pieces("r5k1/p1p1q1pp/1p1P4/3n1r2/2Q2B2/2N5/PP3PPP/R4RK1").unwrap();
        assert_eq!(0xc5020000000000, pieces[Black.to_index() as usize][Pawn.to_index() as usize].value); // black pawns
        assert_eq!(0x800000000, pieces[Black.to_index() as usize][Knight.to_index() as usize].value); // black knights
        assert_eq!(0x0, pieces[Black.to_index() as usize][Bishop.to_index() as usize].value); // black bishops
        assert_eq!(0x100002000000000, pieces[Black.to_index() as usize][Rook.to_index() as usize].value); // black rooks
        assert_eq!(0x10000000000000, pieces[Black.to_index() as usize][Queen.to_index() as usize].value); // black queens
        assert_eq!(0x4000000000000000, pieces[Black.to_index() as usize][King.to_index() as usize].value); // black kings
        assert_eq!(0x8000000e300, pieces[White.to_index() as usize][Pawn.to_index() as usize].value); // white pawns
        assert_eq!(0x40000, pieces[White.to_index() as usize][Knight.to_index() as usize].value); // white knights
        assert_eq!(0x20000000, pieces[White.to_index() as usize][Bishop.to_index() as usize].value); // white bishops
        assert_eq!(0x21, pieces[White.to_index() as usize][Rook.to_index() as usize].value); // white rooks
        assert_eq!(0x4000000, pieces[White.to_index() as usize][Queen.to_index() as usize].value); // white queens
        assert_eq!(0x40, pieces[White.to_index() as usize][King.to_index() as usize].value); // white kings

        let pieces = Board::parse_pieces("8/8/8/8/8/8/8/8").unwrap();
        assert_eq!([[Bitboard::new(0); 6]; 2], pieces);
    }

    #[test]
    fn parse_pieces_with_invalid_fen_returns_error() {
        assert_eq!(Err(String::from("Invalid FEN")), Board::parse_pieces("/rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R/"));
        assert_eq!(Err(String::from("Invalid FEN")), Board::parse_pieces("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP"));
        assert_eq!(Err(String::from("Invalid FEN")), Board::parse_pieces("rnbqk1bnr/8/8/8/8/8/8/8"));
    }

    #[test]
    fn parse_color_with_valid_fen_returns_color() {
        assert_eq!(White, Board::parse_color_to_move("w").unwrap());
        assert_eq!(Black, Board::parse_color_to_move("b").unwrap());
    }

    #[test]
    fn parse_color_with_invalid_fen_returns_error() {
        assert_eq!(Err(String::from("Invalid FEN")), Board::parse_color_to_move("W"));
        assert_eq!(Err(String::from("Invalid FEN")), Board::parse_color_to_move(""));
        assert_eq!(Err(String::from("Invalid FEN")), Board::parse_color_to_move("nonsense"));
    }

    #[test]
    fn parse_castling_rights_with_valid_fen_returns_castling_rights() {
        assert_eq!([CastlingRights::NoRights, CastlingRights::NoRights], Board::parse_castling_rights("-").unwrap());
        assert_eq!([CastlingRights::NoRights, CastlingRights::QueenSide], Board::parse_castling_rights("q").unwrap());
        assert_eq!([CastlingRights::NoRights, CastlingRights::KingSide], Board::parse_castling_rights("k").unwrap());
        assert_eq!([CastlingRights::NoRights, CastlingRights::Both], Board::parse_castling_rights("kq").unwrap());
        assert_eq!([CastlingRights::QueenSide, CastlingRights::NoRights], Board::parse_castling_rights("Q").unwrap());
        assert_eq!([CastlingRights::QueenSide, CastlingRights::QueenSide], Board::parse_castling_rights("Qq").unwrap());
        assert_eq!([CastlingRights::QueenSide, CastlingRights::KingSide], Board::parse_castling_rights("Qk").unwrap());
        assert_eq!([CastlingRights::QueenSide, CastlingRights::Both], Board::parse_castling_rights("Qkq").unwrap());
        assert_eq!([CastlingRights::KingSide, CastlingRights::NoRights], Board::parse_castling_rights("K").unwrap());
        assert_eq!([CastlingRights::KingSide, CastlingRights::QueenSide], Board::parse_castling_rights("Kq").unwrap());
        assert_eq!([CastlingRights::KingSide, CastlingRights::KingSide], Board::parse_castling_rights("Kk").unwrap());
        assert_eq!([CastlingRights::KingSide, CastlingRights::Both], Board::parse_castling_rights("Kkq").unwrap());
        assert_eq!([CastlingRights::Both, CastlingRights::NoRights], Board::parse_castling_rights("KQ").unwrap());
        assert_eq!([CastlingRights::Both, CastlingRights::QueenSide], Board::parse_castling_rights("KQq").unwrap());
        assert_eq!([CastlingRights::Both, CastlingRights::KingSide], Board::parse_castling_rights("KQk").unwrap());
        assert_eq!([CastlingRights::Both, CastlingRights::Both], Board::parse_castling_rights("KQkq").unwrap());
    }

    #[test]
    fn parse_castling_rights_with_invalid_fen_returns_error() {
        assert_eq!(Err(String::from("Invalid FEN")), Board::parse_castling_rights("KQkqq"));
        assert_eq!(Err(String::from("Invalid FEN")), Board::parse_castling_rights("kqKQ"));
        assert_eq!(Err(String::from("Invalid FEN")), Board::parse_castling_rights("nonsense"));
        assert_eq!(Err(String::from("Invalid FEN")), Board::parse_castling_rights("12345"));
    }

    #[test]
    fn parse_en_passant_with_valid_fen_returns_square() {
        assert_eq!(Ok(Some(square::A5)), Board::parse_en_passant("a5"));
        assert_eq!(Ok(Some(square::E4)), Board::parse_en_passant("e4"));
        assert_eq!(Ok(Some(square::F1)), Board::parse_en_passant("f1"));
        assert_eq!(Ok(Some(square::E4)), Board::parse_en_passant("e4"));
        assert_eq!(Ok(Some(square::B5)), Board::parse_en_passant("b5"));
        assert_eq!(Ok(Some(square::H8)), Board::parse_en_passant("h8"));
        assert_eq!(Ok(Some(square::E8)), Board::parse_en_passant("e8"));
        assert_eq!(Ok(Some(square::D3)), Board::parse_en_passant("d3"));
        assert_eq!(Ok(Some(square::A1)), Board::parse_en_passant("a1"));
    }

    #[test]
    fn parse_en_passant_with_invalid_fen_returns_error() {
        assert_eq!(Err(String::from("Invalid FEN")), Board::parse_en_passant(""));
        assert_eq!(Err(String::from("Invalid FEN")), Board::parse_en_passant("12345"));
        assert_eq!(Err(String::from("Invalid FEN")), Board::parse_en_passant("Nonsense"));
        assert_eq!(Err(String::from("Invalid FEN")), Board::parse_en_passant("G5"));
        assert_eq!(Err(String::from("Invalid FEN")), Board::parse_en_passant("a9"));
        assert_eq!(Err(String::from("Invalid FEN")), Board::parse_en_passant("e0"));
        assert_eq!(Err(String::from("Invalid FEN")), Board::parse_en_passant("f-"));
        assert_eq!(Err(String::from("Invalid FEN")), Board::parse_en_passant("ab"));
    }

    #[test]
    fn parse_halfmove_clock_with_valid_fen_returns_halfmove_clock() {
        assert_eq!(Ok(0), Board::parse_halfmove_clock("0"));
        assert_eq!(Ok(15), Board::parse_halfmove_clock("15"));
        assert_eq!(Ok(463), Board::parse_halfmove_clock("463"));
        assert_eq!(Ok(97173), Board::parse_halfmove_clock("97173"));
        assert_eq!(Ok(15_491_392), Board::parse_halfmove_clock("15491392"));
    }

    #[test]
    fn parse_halfmove_clock_with_invalid_fen_returns_error() {
        assert_eq!(Err(String::from("Invalid FEN")), Board::parse_halfmove_clock("-5"));
        assert_eq!(Err(String::from("Invalid FEN")), Board::parse_halfmove_clock("Nonsense"));
        assert_eq!(Err(String::from("Invalid FEN")), Board::parse_halfmove_clock("a"));
        assert_eq!(Err(String::from("Invalid FEN")), Board::parse_halfmove_clock("I like Rust"));
    }

    #[test]
    fn parse_fullmove_counter_with_valid_fen_returns_fullmove_counter() {
        assert_eq!(Ok(1), Board::parse_fullmove_counter("1"));
        assert_eq!(Ok(15), Board::parse_fullmove_counter("15"));
        assert_eq!(Ok(463), Board::parse_fullmove_counter("463"));
        assert_eq!(Ok(97173), Board::parse_fullmove_counter("97173"));
        assert_eq!(Ok(15_491_392), Board::parse_fullmove_counter("15491392"));
    }

    #[test]
    fn parse_fullmove_counter_with_invalid_fen_returns_error() {
        assert_eq!(Err(String::from("Invalid FEN")), Board::parse_fullmove_counter("-5"));
        assert_eq!(Err(String::from("Invalid FEN")), Board::parse_fullmove_counter("Nonsense"));
        assert_eq!(Err(String::from("Invalid FEN")), Board::parse_fullmove_counter("a"));
        assert_eq!(Err(String::from("Invalid FEN")), Board::parse_fullmove_counter("I like Rust"));
        assert_eq!(Err(String::from("Invalid FEN")), Board::parse_fullmove_counter("0"));
    }

    #[test]
    fn test_to_fen() {
        let mut lookup = LookupTable::default();
        lookup.initialize_tables();
        let _ = LOOKUP_TABLE.set(lookup);

        // position 1 (starting position)
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        assert_eq!(board, Board::from_fen(board.to_fen().as_str()).unwrap());

        // position 2
        let board = Board::from_fen("1kr5/R2Q2pp/8/4p1p1/2BpP3/8/2P1KP2/1R6 b - - 0 38").unwrap();
        assert_eq!(board, Board::from_fen(board.to_fen().as_str()).unwrap());

        // position 3
        let board = Board::from_fen("rnbqk2r/ppp2Npp/3p1n2/2b5/2B1P3/8/PPPP1PPP/RNBQK2R b KQkq - 0 5").unwrap();
        assert_eq!(board, Board::from_fen(board.to_fen().as_str()).unwrap());

        // position 4
        let board = Board::from_fen("r6k/4Qpp1/b5qp/8/PP2PP2/1B6/6PP/R3R1K1 b - - 0 26").unwrap();
        assert_eq!(board, Board::from_fen(board.to_fen().as_str()).unwrap());

        // position 5
        let board = Board::from_fen("r1bq1b1r/ppp1n1pp/4k3/3np3/2B5/2N2Q2/PPPP1PPP/R1B1K2R w KQ - 4 9").unwrap();
        assert_eq!(board, Board::from_fen(board.to_fen().as_str()).unwrap());

        // position 6
        let board = Board::from_fen("r1bqkb1r/pp3ppp/2n2n2/4p3/2P5/3P4/PP3PPP/RNBQKBNR w KQkq e6 0 6").unwrap();
        assert_eq!(board, Board::from_fen(board.to_fen().as_str()).unwrap());

        // position 7
        let board = Board::from_fen("3q1r1k/p3b1pp/4Q3/2r1p3/3p4/3P1N2/PPP2PPP/R4RK1 b - - 0 18").unwrap();
        assert_eq!(board, Board::from_fen(board.to_fen().as_str()).unwrap());

        // position 8
        let board = Board::from_fen("1r3rk1/2RR1p1p/p3pQp1/1p6/6P1/1P5P/5PBK/1q6 w - - 0 28").unwrap();
        assert_eq!(board, Board::from_fen(board.to_fen().as_str()).unwrap());

        // position 9
        let board = Board::from_fen("8/8/8/8/8/8/8/8 w - - 0 1").unwrap();
        assert_eq!(board, Board::from_fen(board.to_fen().as_str()).unwrap());

        // position 10
        let board = Board::from_fen("8/1k6/8/8/5K2/8/8/8 w - e3 0 1").unwrap();
        assert_eq!(board, Board::from_fen(board.to_fen().as_str()).unwrap());
    }
}