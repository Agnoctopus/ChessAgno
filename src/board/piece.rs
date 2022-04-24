
pub const PIECES_NUM: usize = 13;
pub const PIECES_COLOR_NUM: usize = 6;

/// Piece representation
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum Piece {
    None = 0,
    WKing,
    WQueen,
    WRook,
    WBishop,
    WKnight,
    WPawn,
    BKing,
    BQueen,
    BRook,
    BBishop,
    BKnight,
    BPawn,
}

/// Sliding pieces
pub const SLIDING_PIECES: [bool; PIECES_NUM] = [
    false,
    false, true, true, true, false, false,
    false, true, true, true, false, false
];

/// Sliding diagonal pieces
pub const SLIDING_DIAG_PIECES: [bool; PIECES_NUM] = [
    false,
    false, true, false, true, false, false,
    false, true, false, true, false, false
];

/// Sliding no diagonal pieces
pub const SLIDING_NO_DIAG_PIECES: [bool; PIECES_NUM] = [
    false,
    false, true, true, false, false, false,
    false, true, true, false, false, false
];

impl Piece {
    /// Returns whether or not the piece is sliding
    #[inline]
    pub fn is_sliding(self) -> bool {
        SLIDING_PIECES[self as usize]
    }

    /// Returns whether or not the piece is diagonal sliding
    #[inline]
    pub fn is_sliding_diag(self) -> bool {
        SLIDING_DIAG_PIECES[self as usize]
    }

    /// Returns whether or not the piece is no diagonal sliding
    #[inline]
    pub fn is_sliding_no_diag(self) -> bool {
        SLIDING_NO_DIAG_PIECES[self as usize]
    }
}