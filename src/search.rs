use std::sync::mpsc::{Receiver, Sender};
use std::time::{Duration, Instant};
use arrayvec::ArrayVec;
use crate::board::Board;
use crate::board::piece::NUM_PIECES;
use crate::board::position::Position;
use crate::board::square::NUM_SQUARES;
use crate::ladybug::Message;
use crate::move_gen;
use crate::move_gen::ply::Ply;

pub mod perft;
pub mod negamax;
mod quiescence_search;

/// The maximum number of plies Ladybug is able to search.
/// This number shouldn't ever be reached.
pub const MAX_PLY: usize = 100;

/// Encodes the commands the search can receive from Ladybug.
pub enum SearchCommand {
    /// Search the given position for the given amount of milliseconds.
    SearchTime(Board, ArrayVec<u64, 1000>, u64),
    /// Search the given position until the given depth is reached.
    SearchDepth(Board, ArrayVec<u64, 1000>, u64),
    /// Perform a perft for the given position up to the specified depth.
    Perft(Position, u64),
    /// Stop the search immediately.
    Stop,
}

/// The search struct is responsible for performing all tasks involving calculation and search.
pub struct Search {
    /// Used to receive search commands from Ladybug.
    command_receiver: Receiver<SearchCommand>,
    /// Used to send search results to Ladybug.
    message_sender: Sender<Message>,
    /// Used to measure the total expired time across all iterations during search.
    total_time: Option<Instant>,
    /// Flag to signal that the search should stop immediately.
    stop: bool,
    /// Contains information collected and used during the search.
    search_info: SearchInfo,
}

/// Contains information collected and used during the search.
pub struct SearchInfo {
    /// The number of nodes evaluated during the current iteration of the search.
    pub node_count: u128,
    /// Stores the lengths of the principe variations.
    pub pv_length: [u8; MAX_PLY],
    /// Stores the principle variations.
    pub pv_table: [[Ply; MAX_PLY]; MAX_PLY],
    /// The search can store up to two killer moves per depth.
    /// Killer moves are quiet moves that caused a beta-cutoff in a similar position, and are worth searching first.
    pub killer_moves: [[Ply; MAX_PLY]; 2],
    /// Stores the history moves. These are moves that increased alpha in other positions, and are worth searching first.
    pub history_moves: [[i32; NUM_SQUARES as usize]; NUM_PIECES as usize],
    /// This flag signals whether the search is currently following the pv line from the previous iteration.
    pub follow_pv: bool,
}

impl Default for SearchInfo {
    /// Default constructor for `SearchInfo`.
    fn default() -> Self {
        Self {
            node_count: 0,
            pv_length: [0; MAX_PLY],
            // initialize the pv table with null moves (a1 to a1)
            pv_table: [[Ply::default(); MAX_PLY];MAX_PLY],
            // initialize the killer moves with null moves (a1 to a1)
            killer_moves: [[Ply::default(); MAX_PLY]; 2],
            history_moves: [[0; NUM_SQUARES as usize]; NUM_PIECES as usize],
            follow_pv: true,
        }
    }
}

impl SearchInfo {
    /// Clears the search information that is not relevant for the next iteration.
    pub fn clear_iteration(&mut self) {
        self.node_count = 0;
        self.pv_length = [0; MAX_PLY];
        self.follow_pv = true;
    }

    /// Clears all search information.
    pub fn clear_all(&mut self) {
        self.clear_iteration();
        self.killer_moves = [[Ply::default(); MAX_PLY]; 2];
        self.history_moves = [[0; NUM_SQUARES as usize]; NUM_PIECES as usize];
    }
}

impl Search {
    /// Constructs a new search instance.
    pub fn new(input_receiver: Receiver<SearchCommand>, output_sender: Sender<Message>) -> Self {
        Self {
            command_receiver: input_receiver,
            message_sender: output_sender,
            total_time: None,
            stop: true,
            search_info: SearchInfo::default(),
        }
    }

    /// Start accepting search commands from Ladybug.
    pub fn run(&mut self) {
        loop {
            // blocks until the search receives a command from Ladybug
            let input = self.command_receiver.recv();

            // if the main thread closes the connection, the search thread must not continue running
            if input.is_err() {
                return;
            }

            // get the input string from the result
            let command = input.unwrap();
            
            match command { 
                SearchCommand::Perft(position, depth) => self.handle_perft(position, depth),
                SearchCommand::SearchTime(board, board_history, time) => self.handle_search(board, None, Some(time), board_history),
                SearchCommand::SearchDepth(board, board_history, depth) => self.handle_search(board, Some(depth), None, board_history),
                _other => {},
            }
        }
    }

    /// Sends the given String to the main thread.
    fn send_output(&self, output: String) {
        let send_result = self.message_sender.send(Message::SearchMessage(output));

        // if the main thread closes the connection, the search thread must not continue running
        if send_result.is_err() {
            panic!("The main thread has unexpectedly closed the channel connection.")
        }
    }

    /// Handles the various "Search" commands.
    fn handle_search(&mut self, board: Board, depth_limit: Option<u64>, time_limit: Option<u64>, board_history: ArrayVec<u64, 1000>) {
        let move_list = move_gen::generate_moves(board.position);
        if move_list.is_empty() {
            self.send_output(String::from("info string no legal moves"));
            return;
        }

        // check if a depth value was provided, if not, use max depth
        let depth_limit = depth_limit.unwrap_or(MAX_PLY as u64);

        // check if a time limit was provided
        let time_limit = match time_limit {
            // if no time limit ws provided, use a default limit of 72 hours
            None => Duration::from_secs(72 * 60 * 60),
            Some(time) => Duration::from_millis(time),
        };

        self.iterative_search(board, depth_limit, time_limit, board_history);
    }
    
    /// Handles the "Perft" command.
    fn handle_perft(&self, position: Position, depth: u64) {
        self.perft(position, depth);
    }
}

#[cfg(test)]
mod tests {
    use crate::board::piece::{NUM_PIECES, Piece};
    use crate::board::square;
    use crate::board::square::NUM_SQUARES;
    use crate::move_gen::ply::Ply;
    use crate::search::{MAX_PLY, SearchInfo};

    #[test]
    fn test_default() {
        let search_info = SearchInfo::default();
        assert_eq!(0, search_info.node_count);
        assert_eq!([0; MAX_PLY], search_info.pv_length);
        assert_eq!([[Ply::default(); MAX_PLY];MAX_PLY], search_info.pv_table);
        assert_eq!([[Ply::default(); MAX_PLY]; 2], search_info.killer_moves);
        assert_eq!([[0; NUM_SQUARES as usize]; NUM_PIECES as usize], search_info.history_moves);
        assert!(search_info.follow_pv);
    }

    #[test]
    fn test_search_info_clear_iteration() {
        let mut search_info = SearchInfo::default();
        search_info.node_count = 50000;
        search_info.pv_length[0] = 5;
        let pv_ply = Ply {
            source: square::E2,
            target: square::E8,
            piece: Piece::Rook,
            captured_piece: None,
            promotion_piece: None,
        };
        search_info.pv_table[4][4] = pv_ply;
        let killer_move = Ply {
            source: square::H7,
            target: square::H8,
            piece: Piece::Pawn,
            captured_piece: None,
            promotion_piece: None,
        };
        search_info.killer_moves[0][5] = killer_move;
        search_info.history_moves[2][13] = 40;
        search_info.follow_pv = false;

        search_info.clear_iteration();

        // these should be cleared
        assert_eq!(0, search_info.node_count);
        assert_eq!([0; MAX_PLY], search_info.pv_length);
        assert!(search_info.follow_pv);

        // this should stay the same
        assert_eq!(pv_ply, search_info.pv_table[4][4]);
        assert_eq!(killer_move, search_info.killer_moves[0][5]);
        assert_eq!(40, search_info.history_moves[2][13]);
    }

    #[test]
    fn test_search_info_clear_all() {
        let mut search_info = SearchInfo::default();
        search_info.killer_moves[0][4] = Ply {
            source: square::H7,
            target: square::H8,
            piece: Piece::Pawn,
            captured_piece: None,
            promotion_piece: None,
        };

        search_info.clear_all();

        assert_eq!([[Ply::default(); MAX_PLY]; 2], search_info.killer_moves);
        assert_eq!([[0; NUM_SQUARES as usize]; NUM_PIECES as usize], search_info.history_moves);
    }
}
