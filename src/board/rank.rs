use std::fmt::{Display, Formatter};

/// Represents a rank on a chessboard.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rank {
    First = 0,
    Second = 1,
    Third = 2,
    Fourth = 3,
    Fifth = 4,
    Sixth = 5,
    Seventh = 6,
    Eighth = 7,
}

/// The number of ranks on a chessboard.
pub const NUM_RANKS: u8 = 8;

impl Rank {
    /// Returns the index of the rank, ranging from 0 (rank 1) to 7 (rank 8).
    pub fn to_index(&self) -> u8 {
        *self as u8
    }

    /// Returns a rank based on the rank's index.
    pub fn from_index(index: u8) -> Rank {
        match index % 8{
            0 => Rank::First,
            1 => Rank::Second,
            2 => Rank::Third,
            3 => Rank::Fourth,
            4 => Rank::Fifth,
            5 => Rank::Sixth,
            6 => Rank::Seventh,
            7 => Rank::Eighth,
            _ => unreachable!(),
        }
    }
    
    /// Returns the rank above.
    pub fn up(&self) -> Rank {
        Rank::from_index(self.to_index() + 1)
    }

    /// Returns the rank below.
    pub fn down(&self) -> Rank {
        match self {
            Rank::First => Rank::Eighth, // Wrap around
            other => Rank::from_index(other.to_index() - 1)
        }
    }
}

/// Prints the rank as text.
impl Display for Rank {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Rank::First => write!(f, "1"),
            Rank::Second => write!(f, "2"),
            Rank::Third => write!(f, "3"),
            Rank::Fourth => write!(f, "4"),
            Rank::Fifth => write!(f, "5"),
            Rank::Sixth => write!(f, "6"),
            Rank::Seventh => write!(f, "7"),
            Rank::Eighth => write!(f, "8"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::board::rank::{NUM_RANKS, Rank};

    #[test]
    fn to_index_returns_correct_index() {
        assert_eq!(0, Rank::First.to_index());
        assert_eq!(1, Rank::Second.to_index());
        assert_eq!(2, Rank::Third.to_index());
        assert_eq!(3, Rank::Fourth.to_index());
        assert_eq!(4, Rank::Fifth.to_index());
        assert_eq!(5, Rank::Sixth.to_index());
        assert_eq!(6, Rank::Seventh.to_index());
        assert_eq!(7, Rank::Eighth.to_index());
    }

    #[test]
    fn from_index_returns_correct_rank() {
        assert_eq!(Rank::First, Rank::from_index(0));
        assert_eq!(Rank::Second, Rank::from_index(1));
        assert_eq!(Rank::Third, Rank::from_index(2));
        assert_eq!(Rank::Fourth, Rank::from_index(3));
        assert_eq!(Rank::Fifth, Rank::from_index(4));
        assert_eq!(Rank::Sixth, Rank::from_index(5));
        assert_eq!(Rank::Seventh, Rank::from_index(6));
        assert_eq!(Rank::Eighth, Rank::from_index(7));

        assert_ne!(Rank::First, Rank::from_index(5));
        assert_ne!(Rank::Fifth, Rank::from_index(2));
        assert_ne!(Rank::Eighth, Rank::from_index(6));

        assert!(!(Rank::First == Rank::Fifth));
    }

    #[test]
    fn from_index_with_invalid_index_wraps_around() {
        assert_eq!(Rank::First, Rank::from_index(8));
    }

    #[test]
    fn up_returns_rank_above() {
        assert_eq!(Rank::Second, Rank::First.up());
        assert_eq!(Rank::Third, Rank::Second.up());
        assert_eq!(Rank::Fourth, Rank::Third.up());
        assert_eq!(Rank::Fifth, Rank::Fourth.up());
        assert_eq!(Rank::Sixth, Rank::Fifth.up());
        assert_eq!(Rank::Seventh, Rank::Sixth.up());
        assert_eq!(Rank::Eighth, Rank::Seventh.up());
        assert_eq!(Rank::First, Rank::Eighth.up());
        for rank_index in (0..NUM_RANKS).rev() {
            assert_eq!(Rank::from_index(rank_index + 1), Rank::from_index(rank_index).up())
        }
    }

    #[test]
    fn down_returns_rank_below() {
        assert_eq!(Rank::Second, Rank::First.up());
        assert_eq!(Rank::Third, Rank::Second.up());
        assert_eq!(Rank::Fourth, Rank::Third.up());
        assert_eq!(Rank::Fifth, Rank::Fourth.up());
        assert_eq!(Rank::Sixth, Rank::Fifth.up());
        assert_eq!(Rank::Seventh, Rank::Sixth.up());
        assert_eq!(Rank::Eighth, Rank::Seventh.up());
        assert_eq!(Rank::First, Rank::Eighth.up());
        for rank_index in (1..NUM_RANKS).rev() {
            assert_eq!(Rank::from_index(rank_index - 1), Rank::from_index(rank_index).down())
        }
    }

    #[test]
    fn rank_formats_correctly() {
        assert_eq!("1", format!("{}", Rank::First));
        assert_eq!("2", format!("{}", Rank::Second));
        assert_eq!("3", format!("{}", Rank::Third));
        assert_eq!("4", format!("{}", Rank::Fourth));
        assert_eq!("5", format!("{}", Rank::Fifth));
        assert_eq!("6", format!("{}", Rank::Sixth));
        assert_eq!("7", format!("{}", Rank::Seventh));
        assert_eq!("8", format!("{}", Rank::Eighth));
    }
}
