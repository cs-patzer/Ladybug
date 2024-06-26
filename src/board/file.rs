use std::fmt::{Display, Formatter};

/// Represents a file on a chessboard.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum File {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}

/// The number of files on a chessboard.
pub const NUM_FILES: u8 = 8;

impl File {
    /// Returns the index of the file, ranging from 0 (file a) to 7 (file h).
    pub fn to_index(&self) -> u8 {
        *self as u8
    }

    /// Returns a file based on the file's index.
    pub fn from_index(index: u8) -> File {
        match index % 8{
            0 => File::A,
            1 => File::B,
            2 => File::C,
            3 => File::D,
            4 => File::E,
            5 => File::F,
            6 => File::G,
            7 => File::H,
            _ => unreachable!(),
        }
    }
    
    /// Constructs a new file from a char.
    pub fn from_char(char: &char) -> Result<File, String> {
        let char = char.to_ascii_lowercase();
        match char {
            'a' => Ok(File::A),
            'b' => Ok(File::B),
            'c' => Ok(File::C),
            'd' => Ok(File::D),
            'e' => Ok(File::E),
            'f' => Ok(File::F),
            'g' => Ok(File::G),
            'h' => Ok(File::H),
            _other => Err(String::from("Invalid char string")),
        }
    }

    /// Returns the file to the right.
    pub fn right(&self) -> File {
        File::from_index(self.to_index() + 1)
    }

    /// Returns the file to the left.
    pub fn left(&self) -> File {
        match self {
            File::A => File::H, // Wrap around
            other=> File::from_index(other.to_index() - 1)
        }
    }
}

/// Prints the file as text.
impl Display for File {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            File::A => write!(f, "a"),
            File::B => write!(f, "b"),
            File::C => write!(f, "c"),
            File::D => write!(f, "d"),
            File::E => write!(f, "e"),
            File::F => write!(f, "f"),
            File::G => write!(f, "g"),
            File::H => write!(f, "h"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::board::file::File;
    use crate::board::file::{NUM_FILES};

    #[test]
    fn to_index_returns_correct_index() {
        assert_eq!(0, File::A.to_index());
        assert_eq!(1, File::B.to_index());
        assert_eq!(2, File::C.to_index());
        assert_eq!(3, File::D.to_index());
        assert_eq!(4, File::E.to_index());
        assert_eq!(5, File::F.to_index());
        assert_eq!(6, File::G.to_index());
        assert_eq!(7, File::H.to_index());
    }

    #[test]
    fn from_index_returns_correct_file() {
        assert_eq!(File::A, File::from_index(0));
        assert_eq!(File::B, File::from_index(1));
        assert_eq!(File::C, File::from_index(2));
        assert_eq!(File::D, File::from_index(3));
        assert_eq!(File::E, File::from_index(4));
        assert_eq!(File::F, File::from_index(5));
        assert_eq!(File::G, File::from_index(6));
        assert_eq!(File::H, File::from_index(7));

        assert_ne!(File::A, File::from_index(1));
        assert_ne!(File::B, File::from_index(6));
        assert_ne!(File::C, File::from_index(0));

        assert!(!(File::A == File::B));
    }

    #[test]
    fn from_index_with_invalid_index_wraps_around() {
        assert_eq!(File::A, File::from_index(8));
    }

    #[test]
    fn from_char_with_valid_char_returns_file() {
        assert_eq!(Ok(File::A), File::from_char(&'a'));
        assert_eq!(Ok(File::B), File::from_char(&'b'));
        assert_eq!(Ok(File::C), File::from_char(&'c'));
        assert_eq!(Ok(File::D), File::from_char(&'d'));
        assert_eq!(Ok(File::E), File::from_char(&'e'));
        assert_eq!(Ok(File::F), File::from_char(&'f'));
        assert_eq!(Ok(File::G), File::from_char(&'g'));
        assert_eq!(Ok(File::H), File::from_char(&'h'));

        assert_eq!(Ok(File::A), File::from_char(&'A'));
        assert_eq!(Ok(File::B), File::from_char(&'B'));
        assert_eq!(Ok(File::C), File::from_char(&'C'));
        assert_eq!(Ok(File::D), File::from_char(&'D'));
        assert_eq!(Ok(File::E), File::from_char(&'E'));
        assert_eq!(Ok(File::F), File::from_char(&'F'));
        assert_eq!(Ok(File::G), File::from_char(&'G'));
        assert_eq!(Ok(File::H), File::from_char(&'H'));
    }
    
    #[test]
    fn from_char_with_invalid_char_returns_error() {
        assert_eq!(Err(String::from("Invalid char string")), File::from_char(&'1'));
        assert_eq!(Err(String::from("Invalid char string")), File::from_char(&'k'));
        assert_eq!(Err(String::from("Invalid char string")), File::from_char(&'-'));
    }
    
    #[test]
    fn right_returns_file_to_the_right() {
        assert_eq!(File::B, File::A.right());
        assert_eq!(File::C, File::B.right());
        assert_eq!(File::D, File::C.right());
        assert_eq!(File::E, File::D.right());
        assert_eq!(File::F, File::E.right());
        assert_eq!(File::G, File::F.right());
        assert_eq!(File::H, File::G.right());
        assert_eq!(File::A, File::H.right());
        for file_index in 0..NUM_FILES {
            assert_eq!(File::from_index(file_index + 1), File::from_index(file_index).right())
        }
    }

    #[test]
    fn left_returns_file_to_the_left() {
        assert_eq!(File::H, File::A.left());
        assert_eq!(File::A, File::B.left());
        assert_eq!(File::B, File::C.left());
        assert_eq!(File::C, File::D.left());
        assert_eq!(File::D, File::E.left());
        assert_eq!(File::E, File::F.left());
        assert_eq!(File::F, File::G.left());
        assert_eq!(File::G, File::H.left());
        for file_index in (1..NUM_FILES).rev() {
            assert_eq!(File::from_index(file_index - 1), File::from_index(file_index).left())
        }
    }

    #[test]
    fn file_formats_correctly() {
        assert_eq!("a", format!("{}", File::A));
        assert_eq!("b", format!("{}", File::B));
        assert_eq!("c", format!("{}", File::C));
        assert_eq!("d", format!("{}", File::D));
        assert_eq!("e", format!("{}", File::E));
        assert_eq!("f", format!("{}", File::F));
        assert_eq!("g", format!("{}", File::G));
        assert_eq!("h", format!("{}", File::H));
    }
}