/// File representation of the board columns
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum File {
    A = 0,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    None,
}

/// Rank representation of the board rows
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Rank {
    One = 0,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    None,
}

/// Position representation on the board
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Position {
    /// File coordinate
    file: File,
    /// rank coordinate
    rank: Rank,
}

impl Position {
    /// Create a new `Position` instance
    #[inline]
    pub const fn new(file: File, rank: Rank) -> Self {
        Self { file, rank }
    }

    /// Returns the file position
    #[inline]
    pub fn file(&self) -> File {
        self.file
    }

    /// Returns the rank position
    #[inline]
    pub fn rank(&self) -> Rank {
        self.rank
    }
}
