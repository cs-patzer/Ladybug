use std::cmp::Reverse;
use arrayvec::ArrayVec;
use crate::move_gen::ply::Ply;
use crate::search::SearchInfo;

/// The move list can hold up to 255 ply, encoded as unsigned 32-bit integers.
pub struct MoveList {
    /// The array of encoded moves.
    moves: ArrayVec<u32, 255>,
}

impl Default for MoveList {
    /// Constructs a new move list.
    fn default() -> Self{
        MoveList {
            moves: ArrayVec::new(),
        }
    }
}

impl MoveList {
    /// Adds a ply to the move list.
    pub fn push(&mut self, ply: Ply) {
        self.moves.push(ply.encode());
    }

    /// Returns the ply with the given index.
    pub fn get(&self, index: u8) -> Ply {
        Ply::decode(self.moves[index as usize])
    }
    
    
    /// Returns the length of the move list.
    pub fn len(&self) -> u8 {
        self.moves.len() as u8
    }
    
    /// Returns true if the move list ist empty.
    pub fn is_empty(&self) -> bool {
        self.moves.is_empty()
    }
    
    /// Sorts the move list by MVV-LVA and various other heuristics.
    pub fn sort(&mut self, search_info: &mut SearchInfo, ply_index: u64) {
        // flag to signal whether the pv move of the last search iteration is contained in this move list
        let mut contains_pv = false;
        
        self.moves.sort_by_key(|encoded_ply| {
            // score the move based on MVV-LVA
            let ply = Ply::decode(*encoded_ply);
            let mut score = ply.score();

            // check if move the move is quiet, if yes, apply move ordering heuristics
            if ply.captured_piece.is_none() {
                // first killer move
                if search_info.killer_moves[0][ply_index as usize] == ply {
                    score += 70;
                }
                // second killer move
                else if search_info.killer_moves[1][ply_index as usize] == ply {
                    score += 50;
                } 
                // history move
                else {
                    score += search_info.history_moves[ply.piece.to_index() as usize][ply.target.index as usize];
                }
            }
            
            // check if we are following the pv line
            if search_info.follow_pv && ply == search_info.pv_table[0][ply_index as usize] {
                contains_pv = true;
                score += 1_000_000;
            }

            Reverse(score)
        });
        
        // If the move list does not contain the pv move from the last iteration, we are no longer following the pv line
        if !contains_pv {
            search_info.follow_pv = false;
        }
    }
    
    /// Returns a new move list that only contains capture moves.
    pub fn get_captures(&self) -> MoveList {
        let mut capture_list = MoveList::default();

        for ply in &self.moves {
            if Ply::decode(*ply).captured_piece.is_some() {
                capture_list.moves.push(*ply);
            }
        }
        
        capture_list
    }
}

#[cfg(test)]
mod tests {
    use crate::board::piece::Piece;
    use crate::board::square;
    use crate::move_gen::move_list::MoveList;
    use crate::move_gen::ply::Ply;
    use crate::search::SearchInfo;

    #[test]
    fn test_move_list() {
        let ply1 = Ply {source: square::A1, target: square::A2, piece: Piece::Rook, captured_piece: None, promotion_piece: None};
        let ply2 = Ply {source: square::H8, target: square::A8, piece: Piece::Rook, captured_piece: Some(Piece::Rook), promotion_piece: None};
        let ply3 = Ply {source: square::E4, target: square::D5, piece: Piece::Pawn, captured_piece: Some(Piece::Pawn), promotion_piece: None};
        let ply4 = Ply {source: square::G7, target: square::H8, piece: Piece::Pawn, captured_piece: Some(Piece::Queen), promotion_piece: Some(Piece::Knight)};
        let ply5 = Ply {source: square::H3, target: square::C8, piece: Piece::Bishop, captured_piece: Some(Piece::Rook), promotion_piece: None};
        
        let mut move_list = MoveList::default();
        assert_eq!(0, move_list.len());
        assert!(move_list.is_empty());
        
        move_list.push(ply1);
        assert_eq!(1, move_list.len());
        assert_eq!(ply1, move_list.get(0));
        assert!(!move_list.is_empty());
        
        move_list.push(ply1);
        move_list.push(ply2);
        move_list.push(ply3);
        move_list.push(ply4);
        move_list.push(ply5);
        
        assert_eq!(6, move_list.len());
        assert_eq!(ply1, move_list.get(0));
        assert_eq!(ply1, move_list.get(1));
        assert_eq!(ply2, move_list.get(2));
        assert_eq!(ply3, move_list.get(3));
        assert_eq!(ply4, move_list.get(4));
        assert_eq!(ply5, move_list.get(5));
        
        let mut move_list = MoveList::default();
        for _i in 0..255 {
            move_list.push(Ply {source: square::G7, target: square::H8, piece: Piece::Pawn, captured_piece: Some(Piece::Queen), promotion_piece: Some(Piece::Knight)});
        }
        assert!(!move_list.is_empty());
        assert_eq!(255, move_list.len());
    }
    
    #[test]
    fn test_sort() {
        let mut search_info = SearchInfo::default();
        
        let ply1 = Ply {source: square::A1, target: square::A2, piece: Piece::Rook, captured_piece: None, promotion_piece: None};
        let ply2 = Ply {source: square::H8, target: square::A8, piece: Piece::Rook, captured_piece: Some(Piece::Rook), promotion_piece: None};
        let ply3 = Ply {source: square::E4, target: square::D5, piece: Piece::Pawn, captured_piece: Some(Piece::Pawn), promotion_piece: None};
        let ply4 = Ply {source: square::G7, target: square::H8, piece: Piece::Pawn, captured_piece: Some(Piece::Queen), promotion_piece: Some(Piece::Knight)};
        let ply5 = Ply {source: square::H3, target: square::C8, piece: Piece::Bishop, captured_piece: Some(Piece::Rook), promotion_piece: None};
        
        let mut move_list = MoveList::default();
        
        move_list.push(ply1);
        move_list.push(ply2);
        move_list.push(ply3);
        move_list.push(ply4);
        move_list.push(ply5);
        
        assert_eq!(5, move_list.len());
        
        move_list.sort(&mut search_info, 0);

        assert_eq!(5, move_list.len());
        
        assert_eq!(ply4, move_list.get(0));
        assert_eq!(ply5, move_list.get(1));
        assert_eq!(ply2, move_list.get(2));
        assert_eq!(ply3, move_list.get(3));
        assert_eq!(ply1, move_list.get(4));
    }
    
    #[test]
    fn test_get_captures() {
        let ply1 = Ply {source: square::A1, target: square::A2, piece: Piece::Rook, captured_piece: None, promotion_piece: None};
        let ply2 = Ply {source: square::H8, target: square::A8, piece: Piece::Rook, captured_piece: Some(Piece::Rook), promotion_piece: None};
        let ply3 = Ply {source: square::E4, target: square::D5, piece: Piece::Pawn, captured_piece: Some(Piece::Pawn), promotion_piece: None};
        let ply4 = Ply {source: square::G7, target: square::H8, piece: Piece::Pawn, captured_piece: Some(Piece::Queen), promotion_piece: Some(Piece::Knight)};
        let ply5 = Ply {source: square::H3, target: square::C8, piece: Piece::Bishop, captured_piece: Some(Piece::Rook), promotion_piece: None};

        let mut move_list = MoveList::default();

        move_list.push(ply1);
        move_list.push(ply2);
        move_list.push(ply3);
        move_list.push(ply4);
        move_list.push(ply5);
        
        let capture_list = move_list.get_captures();
        
        assert_eq!(4, capture_list.len())
    }
}