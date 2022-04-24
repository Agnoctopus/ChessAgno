//! Pieces

use std::slice::Iter;

use super::color::Color;

/// Number of pieces
pub const PIECES_NUM: usize = 13;
/// Number of color pieces
pub const PIECES_COLOR_NUM: usize = 6;


/// Piece type of every possible piece present on the board.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn
}

/// Piece types char representation
const CHAR_PIECE_TYPES: [char; 6] = [
    'K', 'Q', 'R', 'B', 'N', 'P'
];


impl PieceType {
    /// All possible piece types, usefull for iteratation
    const PIECE_TYPES: [PieceType; 6] = [
        PieceType::King, PieceType::Queen, PieceType::Rook,
        PieceType::Bishop, PieceType::Knight, PieceType::Pawn,
    ];

    /// Returns an iterator over all possible pieces types
    #[inline]
    pub fn all() -> Iter<'static, PieceType> {
        Self::PIECE_TYPES.iter()
    }

    /// Returns the piece type char representation
    #[inline]
    pub fn to_char(self) -> char {
        CHAR_PIECE_TYPES[self as usize]
    }
}

impl TryFrom<char> for PieceType {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        let piece_type = match c {
            'K' => PieceType::King,
            'Q' => PieceType::Queen,
            'R' => PieceType::Rook,
            'B' => PieceType::Bishop,
            'N' => PieceType::Knight,
            'P' => PieceType::Pawn,
            _ => return Err(format!("Unknown piece type: {}", c))
        };

        Ok(piece_type)
    }
}

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
const SLIDING_PIECES: [bool; PIECES_NUM] = [
    false,
    false, true, true, true, false, false,
    false, true, true, true, false, false
];

/// Sliding diagonal pieces
const SLIDING_DIAG_PIECES: [bool; PIECES_NUM] = [
    false,
    false, true, false, true, false, false,
    false, true, false, true, false, false
];

/// Sliding no diagonal pieces
const SLIDING_NO_DIAG_PIECES: [bool; PIECES_NUM] = [
    false,
    false, true, true, false, false, false,
    false, true, true, false, false, false
];

const COLOR_PIECES: [Color; PIECES_NUM] = [
    Color::None,
    Color::White, Color::White, Color::White,
    Color::White, Color::White, Color::White,

    Color::Black, Color::Black, Color::Black,
    Color::Black, Color::Black, Color::Black,
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

    /// Returns the color
    #[inline]
    pub fn color(self) -> Color {
        COLOR_PIECES[self as usize]
    }
}