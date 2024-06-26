use crate::board::color::Color;
use crate::board::file::File;
use crate::board::piece::Piece;
use crate::board::position::Position;
use crate::board::square::Square;
use crate::lookup::LOOKUP_TABLE;
use crate::move_gen::move_list::MoveList;
use crate::move_gen::ply::Ply;

/// Generates all legal pawn moves for the given position.
pub fn generate_pawn_moves(position: Position, move_list: &mut MoveList)  {
    generate_quiet_pawn_moves(position, move_list);
    generate_attacking_pawn_moves(position, move_list);
    generate_en_passant_moves(position, move_list);
}

/// Generates all legal quiet pawn moves for the given position.
fn generate_quiet_pawn_moves(position: Position, move_list: &mut MoveList) {
    // get occupancies
    let occupancies = position.get_occupancies();

    // get pawn bitboard for the color to move
    let pawn_bb = position.pieces[position.color_to_move.to_index() as usize][Piece::Pawn.to_index() as usize];

    // get all squares with a pawn on it
    let active_squares = pawn_bb.get_active_bits();

    // loop over squares and calculate possible moves
    for source in active_squares {
        let target = match position.color_to_move {
            Color::White => source.up(),
            Color::Black => source.down(),
        };

        // check if target square is empty
        if occupancies.get_bit(target) {
            continue;
        }

        // check if target square is on the promotion rank
        if target.get_rank() == position.color_to_move.promotion_rank() {
            // move is a promotion - add all possible promotion moves
            for piece_index in Piece::Knight.to_index() as usize..Piece::Queen.to_index() as usize + 1 {
                let ply = Ply { source, target, piece: Piece::Pawn, captured_piece: None, promotion_piece: Some(Piece::from_index(piece_index as u8))};
                if position.make_move(ply).is_legal() {
                    move_list.push(ply);
                }
            }
        } else {
            // move is not a promotion
            let ply = Ply { source, target, piece: Piece::Pawn, captured_piece: None, promotion_piece: None};
            if position.make_move(ply).is_legal() {
                move_list.push(ply);
            }

            // check if double pawn push is possible
            if source.get_rank() == position.color_to_move.pawn_rank() {
                let mut double_pawn_push_target = target;
                match position.color_to_move {
                    Color::White => double_pawn_push_target = double_pawn_push_target.up(),
                    Color::Black => double_pawn_push_target = double_pawn_push_target.down(),
                }
                if !occupancies.get_bit(double_pawn_push_target) {
                    // no piece on double pawn push target square, so double pawn move is possible
                    let ply = Ply { source, target: double_pawn_push_target, piece: Piece::Pawn, captured_piece: None, promotion_piece: None};
                    if position.make_move(ply).is_legal() {
                        move_list.push(ply);
                    }
                }
            }
        }
    }
}

/// Generates all legal attacking pawn moves for the given position.
fn generate_attacking_pawn_moves(position: Position, move_list: &mut MoveList) {
    // get a reference to the lookup table
    let lookup = LOOKUP_TABLE.get().unwrap();

    // get opposite color occupancy
    let occupancy = position.get_occupancy(position.color_to_move.other());

    // get pawn bitboard for the color to move
    let pawn_bb = position.pieces[position.color_to_move.to_index() as usize][Piece::Pawn.to_index() as usize];

    // get all squares with a pawn on it
    let active_squares = pawn_bb.get_active_bits();

    // loop over source squares and calculate possible moves
    for source in active_squares {
        // lookup the attack bb for the pawn on the source square
        let mut target_attack_bb = lookup.get_pawn_attacks(source, position.color_to_move);
        
        // `and` the attack bb with the opponent's occupancy (because a capture is only possible if an enemy pawn occupies the target square)
        target_attack_bb.value &= occupancy.value;

        // these are the targets that we know are occupied by an enemy pawn
        let active_squares = target_attack_bb.get_active_bits();

        // loop over target squares and create moves
        for target in active_squares {
            // get the type of the attacked piece
            let attacked_piece= match position.get_piece(target) {
                Some((piece, _color)) => piece,
                None => continue,
            };
            
            // check if target square is on the promotion rank
            if target.get_rank() == position.color_to_move.promotion_rank() {
                // move is a promotion - add all possible promotion moves
                for piece_index in Piece::Knight.to_index() as usize..Piece::Queen.to_index() as usize + 1 {
                    let ply = Ply { source, target, piece: Piece::Pawn, captured_piece: Some(attacked_piece), promotion_piece: Some(Piece::from_index(piece_index as u8))};
                    if position.make_move(ply).is_legal() {
                        move_list.push(ply);
                    }
                }
            } else {
                // move is not a promotion
                let ply = Ply { source, target, piece: Piece::Pawn, captured_piece: Some(attacked_piece), promotion_piece: None};
                if position.make_move(ply).is_legal() {
                    move_list.push(ply);
                }
            }
        }
    }
}

/// Generates all legal en passant moves for the given position.
fn generate_en_passant_moves(position: Position, move_list: &mut MoveList) {
    if let Some(target_square) = position.en_passant {
        // get pawn bitboard for the color to move
        let pawn_bb = position.pieces[position.color_to_move.to_index() as usize][Piece::Pawn.to_index() as usize];
        
        // the rank of the pawns that can capture en passant
        let source_rank = position.color_to_move.other().double_pawn_push_target_rank();
        
        // check file to the left for pawn that can capture en passant
        if target_square.get_file() != File::A {
            let source = Square::from_file_rank(target_square.get_file().left(), source_rank);
            if pawn_bb.get_bit(source) {
                let ply = Ply { source, target: target_square, piece: Piece::Pawn, captured_piece: Some(Piece::Pawn), promotion_piece: None};
                if position.make_move(ply).is_legal() {
                    move_list.push(ply);
                }
            }
        }
        // check file to the right for pawn that can capture en passant
        if target_square.get_file() != File::H {
            let source = Square::from_file_rank(target_square.get_file().right(), source_rank);
            if pawn_bb.get_bit(source) {
                let ply = Ply { source, target: target_square, piece: Piece::Pawn, captured_piece: Some(Piece::Pawn), promotion_piece: None};
                if position.make_move(ply).is_legal() {
                    move_list.push(ply);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::board::{Board, square};
    use crate::lookup::LOOKUP_TABLE;
    use crate::lookup::lookup_table::LookupTable;
    use crate::move_gen::move_list::MoveList;
    use crate::move_gen::pawn_moves;

    #[test]
    fn test_generate_quiet_pawn_moves() {
        let mut lookup = LookupTable::default();
        lookup.initialize_tables();
        let _ = LOOKUP_TABLE.set(lookup);

        // position 1 (starting position)

        let position = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap().position;
        let mut move_list = MoveList::default();
        pawn_moves::generate_quiet_pawn_moves(position, &mut move_list);
        assert_eq!(16, move_list.len());

        // position 2

        let position = Board::from_fen("r4rk1/6pp/pp2b3/3pPp2/4nP1q/1PNQ2bP/PB2B1PK/R4R2 w - - 11 22").unwrap().position;
        let mut move_list = MoveList::default();
        pawn_moves::generate_quiet_pawn_moves(position, &mut move_list);
        assert_eq!(0, move_list.len());

        // position 3

        let position = Board::from_fen("r1bqkbnr/1pp3pp/p1np4/4pp2/2P5/1P2PN2/PB1P1PPP/RN1QKB1R w KQkq - 0 6").unwrap().position;
        let mut move_list = MoveList::default();
        pawn_moves::generate_quiet_pawn_moves(position, &mut move_list);
        assert_eq!(11, move_list.len());

        // position 4

        let position = Board::from_fen("r1b1kbnr/1pp3pp/p1n5/4Bp2/2P4q/1P2P3/P2P1PPP/RN1QKB1R w KQkq - 1 8").unwrap().position;
        let mut move_list = MoveList::default();
        pawn_moves::generate_quiet_pawn_moves(position, &mut move_list);
        assert_eq!(10, move_list.len());

        // position 5

        let position = Board::from_fen("r3kbnr/1p4pp/2p5/p1PbB3/Pn1PPp1q/1P3PPP/8/RN1QKB1R w KQkq - 1 14").unwrap().position;
        let mut move_list = MoveList::default();
        pawn_moves::generate_quiet_pawn_moves(position, &mut move_list);
        assert_eq!(0, move_list.len());

        // position 6

        let position = Board::from_fen("r3kbnr/8/8/2PbB3/Pn1PP2q/1P3PPP/7R/RN1QKB2 b Qkq - 2 14").unwrap().position;
        let mut move_list = MoveList::default();
        pawn_moves::generate_quiet_pawn_moves(position, &mut move_list);
        assert_eq!(0, move_list.len());

        // position 7

        let position = Board::from_fen("r3kbnr/8/8/p1PbB3/Pn1PP2q/1P3PPP/7R/RN1QKB2 b Qkq - 2 14").unwrap().position;
        let mut move_list = MoveList::default();
        pawn_moves::generate_quiet_pawn_moves(position, &mut move_list);
        assert_eq!(0, move_list.len());

        // position 8

        let position = Board::from_fen("r3kbnr/1p6/8/2PbB3/Pn1PP2q/1P3PPP/7R/RN1QKB2 b Qkq - 2 14").unwrap().position;
        let mut move_list = MoveList::default();
        pawn_moves::generate_quiet_pawn_moves(position, &mut move_list);
        assert_eq!(2, move_list.len());

        // position 9

        let position = Board::from_fen("r3kbnr/1p6/8/1QPbB3/Pn1PP2q/1P3PPP/7R/R3KB2 b Qkq - 2 14").unwrap().position;
        let mut move_list = MoveList::default();
        pawn_moves::generate_quiet_pawn_moves(position, &mut move_list);
        assert_eq!(0, move_list.len());

        // position 10

        let position = Board::from_fen("r3kbnr/1p4Q1/8/1RPbB3/Pn1PP2q/1P3PPP/7R/4KB2 b kq - 2 14").unwrap().position;
        let mut move_list = MoveList::default();
        pawn_moves::generate_quiet_pawn_moves(position, &mut move_list);
        assert_eq!(1, move_list.len());

        // position 11

        let position = Board::from_fen("rnb1kb1r/ppp2ppp/3pp2n/3P4/3KP1q1/8/PPP2PPP/RNBQ1BNR b kq - 4 6").unwrap().position;
        let mut move_list = MoveList::default();
        pawn_moves::generate_quiet_pawn_moves(position, &mut move_list);
        assert_eq!(11, move_list.len());
    }

    #[test]
    fn test_generate_attacking_pawn_moves() {
        let mut lookup = LookupTable::default();
        lookup.initialize_tables();
        let _ = LOOKUP_TABLE.set(lookup);

        // position 1 (starting position)

        let position = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap().position;
        let mut move_list = MoveList::default();
        pawn_moves::generate_attacking_pawn_moves(position, &mut move_list);
        assert_eq!(0, move_list.len());

        // position 2

        let position = Board::from_fen("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2").unwrap().position;
        let mut move_list = MoveList::default();
        pawn_moves::generate_attacking_pawn_moves(position, &mut move_list);
        assert_eq!(1, move_list.len());

        // position 3

        let position = Board::from_fen("rnbqkbnr/pp3ppp/8/2ppp3/1P2P1P1/2N5/P1PP1P1P/R1BQKBNR b KQkq - 1 4").unwrap().position;
        let mut move_list = MoveList::default();
        pawn_moves::generate_attacking_pawn_moves(position, &mut move_list);
        assert_eq!(2, move_list.len());

        // position 4

        let position = Board::from_fen("rnbqkbnr/1p5p/8/p2pppp1/1p1PPPPP/P1N5/2P5/R1BQKBNR b KQkq - 0 8").unwrap().position;
        let mut move_list = MoveList::default();
        pawn_moves::generate_attacking_pawn_moves(position, &mut move_list);
        assert_eq!(9, move_list.len());

        // position 5

        let position = Board::from_fen("rnbqkbnr/1p5p/8/p2pppp1/3PPPPP/P1N5/2p4R/1RBQKBN1 b kq - 1 10").unwrap().position;
        let mut move_list = MoveList::default();
        pawn_moves::generate_attacking_pawn_moves(position, &mut move_list);
        assert_eq!(15, move_list.len());

        // position 6

        let position = Board::from_fen("rnb1kbnr/1p2q2p/8/p2p1pp1/3PPpPP/PpN5/2P4R/1RBQKBN1 w kq - 2 11").unwrap().position;
        let mut move_list = MoveList::default();
        pawn_moves::generate_attacking_pawn_moves(position, &mut move_list);
        assert_eq!(3, move_list.len());

        // position 7

        let position = Board::from_fen("rnb1kbnr/1p5p/8/p2p1pp1/3PqpPP/PpN4N/2P4R/1RBQKB2 w kq - 0 12").unwrap().position;
        let mut move_list = MoveList::default();
        pawn_moves::generate_attacking_pawn_moves(position, &mut move_list);
        assert_eq!(0, move_list.len());

        // position 8

        let position = Board::from_fen("rnb1kbnr/1p5p/8/p2p1pp1/3P1pPP/PpNq3N/2PK3R/1RBQ1B2 w kq - 2 13").unwrap().position;
        let mut move_list = MoveList::default();
        pawn_moves::generate_attacking_pawn_moves(position, &mut move_list);
        assert_eq!(1, move_list.len());

        // position 9

        let position = Board::from_fen("rnb1k1n1/1p4P1/8/3p1p1r/p2P1pP1/PpNP3N/3K3R/1RBQ1B2 w q - 1 17").unwrap().position;
        let mut move_list = MoveList::default();
        pawn_moves::generate_attacking_pawn_moves(position, &mut move_list);
        assert_eq!(2, move_list.len());

        // position 10

        let position = Board::from_fen("rnb3n1/1p2k1P1/8/1N1p1P1r/p2P1p2/P2P3N/1p1K4/1RBQ1B2 b - - 0 20").unwrap().position;
        let mut move_list = MoveList::default();
        pawn_moves::generate_attacking_pawn_moves(position, &mut move_list);
        assert_eq!(4, move_list.len());

        // position 11

        let position = Board::from_fen("r1b3n1/1p2k1P1/8/1N1pnPNr/p2P1p2/P2P4/8/1RKQ1B2 w - - 1 23").unwrap().position;
        let mut move_list = MoveList::default();
        pawn_moves::generate_attacking_pawn_moves(position, &mut move_list);
        assert_eq!(1, move_list.len());
    }

    #[test]
    fn test_generate_en_passant_moves() {
        let mut lookup = LookupTable::default();
        lookup.initialize_tables();
        let _ = LOOKUP_TABLE.set(lookup);

        // position 1 (starting position)

        let position = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap().position;
        let mut move_list = MoveList::default();
        pawn_moves::generate_en_passant_moves(position, &mut move_list);
        assert_eq!(0, move_list.len());

        // position 2

        let position = Board::from_fen("rnbqkbnr/1pp1pppp/p7/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 3").unwrap().position;
        let mut move_list = MoveList::default();
        pawn_moves::generate_en_passant_moves(position, &mut move_list);
        assert_eq!(1, move_list.len());
        assert_eq!(square::E5, move_list.get(0).source);
        assert_eq!(square::D6, move_list.get(0).target);

        // position 3

        let position = Board::from_fen("rnbqkbnr/1pp1p1pp/8/p2pPpP1/8/8/PPPP1P1P/RNBQKBNR w KQkq f6 0 5").unwrap().position;
        let mut move_list = MoveList::default();
        pawn_moves::generate_en_passant_moves(position, &mut move_list);
        assert_eq!(2, move_list.len());
        assert_eq!(square::E5, move_list.get(0).source);
        assert_eq!(square::F6, move_list.get(0).target);
        assert_eq!(square::G5, move_list.get(1).source);
        assert_eq!(square::F6, move_list.get(1).target);

        // position 4

        let position = Board::from_fen("rnbqkbnr/1pp1p1p1/8/p2pPpPp/8/5P2/PPPP3P/RNBQKBNR w KQkq h6 0 6").unwrap().position;
        let mut move_list = MoveList::default();
        pawn_moves::generate_en_passant_moves(position, &mut move_list);
        assert_eq!(1, move_list.len());
        assert_eq!(square::G5, move_list.get(0).source);
        assert_eq!(square::H6, move_list.get(0).target);

        // position 5

        let position = Board::from_fen("rn1qkbn1/1bpp1ppr/1p5p/p2Pp3/8/P3PK1P/1PP2PP1/RNBQ1BNR w q e6 0 8").unwrap().position;
        let mut move_list = MoveList::default();
        pawn_moves::generate_en_passant_moves(position, &mut move_list);
        assert_eq!(0, move_list.len());

        // position 6

        let position = Board::from_fen("rn1qkbn1/1b1ppppr/1p5p/p1pP4/8/P3PK1P/1PP2PP1/RNBQ1BNR w q c6 0 8").unwrap().position;
        let mut move_list = MoveList::default();
        pawn_moves::generate_en_passant_moves(position, &mut move_list);
        assert_eq!(1, move_list.len());
        assert_eq!(square::D5, move_list.get(0).source);
        assert_eq!(square::C6, move_list.get(0).target);

        // position 7

        let position = Board::from_fen("rnbqkbnr/1p1ppppp/8/7P/pPp5/3P4/P1P1PPP1/RNBQKBNR b KQkq b3 0 5").unwrap().position;
        let mut move_list = MoveList::default();
        pawn_moves::generate_en_passant_moves(position, &mut move_list);
        assert_eq!(2, move_list.len());
        assert_eq!(square::A4,  move_list.get(0).source);
        assert_eq!(square::B3,  move_list.get(0).target);
        assert_eq!(square::C4,  move_list.get(1).source);
        assert_eq!(square::B3,  move_list.get(1).target);

        // position 8

        let position = Board::from_fen("rnbqkbnr/1p1pppp1/7p/7P/pPp5/3P4/P1P1PPP1/RNBQKBNR w KQkq - 0 6").unwrap().position;
        let mut move_list = MoveList::default();
        pawn_moves::generate_en_passant_moves(position, &mut move_list);
        assert_eq!(0, move_list.len());

        // position 9

        let position = Board::from_fen("rnbqkbnr/ppppppp1/8/8/6Pp/2N2N2/PPPPPP1P/R1BQKB1R b KQkq g3 0 3").unwrap().position;
        let mut move_list = MoveList::default();
        pawn_moves::generate_en_passant_moves(position, &mut move_list);
        assert_eq!(1, move_list.len());
        assert_eq!(square::H4,  move_list.get(0).source);
        assert_eq!(square::G3,  move_list.get(0).target);

        // position 10

        let position = Board::from_fen("1nbqkbnr/rp1p1p2/7p/7P/pPp1pPp1/N2PR3/PBP1P1P1/R2QKBN1 b Qk f3 0 11").unwrap().position;
        let mut move_list = MoveList::default();
        pawn_moves::generate_en_passant_moves(position, &mut move_list);
        assert_eq!(1, move_list.len());
        assert_eq!(square::G4,  move_list.get(0).source);
        assert_eq!(square::F3,  move_list.get(0).target);
    }

    #[test]
    fn test_generate_pawn_moves() {
        let mut lookup = LookupTable::default();
        lookup.initialize_tables();
        let _ = LOOKUP_TABLE.set(lookup);

        // position 1 (starting position)

        let position = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap().position;
        let mut move_list = MoveList::default();
        pawn_moves::generate_pawn_moves(position, &mut move_list);
        assert_eq!(16, move_list.len());

        // position 2

        let position = Board::from_fen("1r6/1p1R2pk/2pp3p/p3p3/4P3/P2P3P/1PP3PN/7K b - - 2 27").unwrap().position;
        let mut move_list = MoveList::default();
        pawn_moves::generate_pawn_moves(position, &mut move_list);
        assert_eq!(6, move_list.len());

        // position 3

        let position = Board::from_fen("2k2b1r/ppp1pppp/5n2/q3P3/6b1/2N5/PPP1BPPP/R1Br1RK1 w - - 0 10").unwrap().position;
        let mut move_list = MoveList::default();
        pawn_moves::generate_pawn_moves(position, &mut move_list);
        assert_eq!(11, move_list.len());

        // position 4

        let position = Board::from_fen("1nkrr3/5pp1/1bp2q1p/p2p4/3P1PB1/P3B2P/1PPQ4/2KRR3 b - - 1 22").unwrap().position;
        let mut move_list = MoveList::default();
        pawn_moves::generate_pawn_moves(position, &mut move_list);
        assert_eq!(0, move_list.len());

        // position 5

        let position = Board::from_fen("1r3rk1/p2p2pp/b1p2n2/4p3/4pP2/7P/PPP3P1/2K1R1NR b - f3 0 16").unwrap().position;
        let mut move_list = MoveList::default();
        pawn_moves::generate_pawn_moves(position, &mut move_list);
        assert_eq!(10, move_list.len());

        // position 6

        let position = Board::from_fen("8/2p5/1pp1k1p1/p3P1Pp/P1nP3K/2P4P/2b5/2B5 w - - 0 32").unwrap().position;
        let mut move_list = MoveList::default();
        pawn_moves::generate_pawn_moves(position, &mut move_list);
        assert_eq!(1, move_list.len());

        // position 7

        let position = Board::from_fen("8/1p3nk1/p2p2pp/P2P4/2P2PN1/1P5P/4R1K1/8 b - - 0 36").unwrap().position;
        let mut move_list = MoveList::default();
        pawn_moves::generate_pawn_moves(position, &mut move_list);
        assert_eq!(4, move_list.len());

        // position 8

        let position = Board::from_fen("6k1/1PQ2pp1/4p2p/4P3/8/7P/r3rPK1/8 w - - 1 39").unwrap().position;
        let mut move_list = MoveList::default();
        pawn_moves::generate_pawn_moves(position, &mut move_list);
        assert_eq!(5, move_list.len());

        // position 9

        let position = Board::from_fen("rnb2rk1/1p3pp1/1bpp1q1p/p3p3/P2PP3/1NP2N2/1P2BPPP/R2QK2R b KQ - 0 11").unwrap().position;
        let mut move_list = MoveList::default();
        pawn_moves::generate_pawn_moves(position, &mut move_list);
        assert_eq!(6, move_list.len());

        // position 10

        let position = Board::from_fen("r1bqk1nr/pp1pbppp/2nP4/8/8/8/PP2QPPP/RNB1KBNR w KQkq - 3 9").unwrap().position;
        let mut move_list = MoveList::default();
        pawn_moves::generate_pawn_moves(position, &mut move_list);
        assert_eq!(11, move_list.len());
    }
}