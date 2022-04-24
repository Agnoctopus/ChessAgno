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

/// Pieces by color and piece type
const COLOR_TYPES_PIECES: [[Piece; 6]; 3] =
[
    [
        Piece::None, Piece::None, Piece::None,
        Piece::None, Piece::None, Piece::None,
    ], [
        Piece::WKing, Piece::WQueen, Piece::WRook,
        Piece::WBishop, Piece::WKnight, Piece::WPawn
    ], [
        Piece::BKing, Piece::BQueen, Piece::BRook,
        Piece::BBishop, Piece::BKnight, Piece::BPawn
    ],
];

/// Color pieces
const COLOR_PIECES: [Color; PIECES_NUM] = [
    Color::None,
    // White
    Color::White, Color::White, Color::White,
    Color::White, Color::White, Color::White,
    // Black
    Color::Black, Color::Black, Color::Black,
    Color::Black, Color::Black, Color::Black,
];

/// Piece types for pieces
const TYPES_PIECES: [PieceType; PIECES_NUM] = [
    PieceType::Pawn,
    // White
    PieceType::King, PieceType::Queen, PieceType::Rook,
    PieceType::Bishop, PieceType::Knight, PieceType::Pawn,
    // Black
    PieceType::King, PieceType::Queen, PieceType::Rook,
    PieceType::Bishop, PieceType::Knight, PieceType::Pawn,
];

/// Piece types char FEN representation
const CHAR_PIECES: [char; PIECES_NUM] = [
    '.',
    'K', 'Q', 'R', 'B', 'N', 'P',
    'k', 'q', 'r', 'b', 'n', 'p'
];


impl Piece {
    /// Get the piece associated to a color and a piece type
    #[inline]
    pub fn from_color_type(color: Color, piece_type: PieceType) -> Piece {
        COLOR_TYPES_PIECES[color as usize][piece_type as usize]
    }

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

    /// Returns the piece type
    #[inline]
    pub fn piece_type(self) -> PieceType {
        TYPES_PIECES[self as usize]
    }

    /// Returns the piece type char representation
    #[inline]
    pub fn to_fen_char(self) -> char {
        CHAR_PIECES[self as usize]
    }
}

impl TryFrom<char> for Piece {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        let piece = match c {
            '.' => Piece::None,
            // White
            'K' => Piece::WKing,
            'Q' => Piece::WQueen,
            'R' => Piece::WRook,
            'B' => Piece::WBishop,
            'N' => Piece::WKnight,
            'P' => Piece::WPawn,
            // Black
            'k' => Piece::BKing,
            'q' => Piece::BQueen,
            'r' => Piece::BRook,
            'b' => Piece::BBishop,
            'n' => Piece::BKnight,
            'p' => Piece::BPawn,
            _ => return Err(format!("Unknown piece: {}", c))
        };

        Ok(piece)
    }
}