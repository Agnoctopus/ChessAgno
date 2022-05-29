//! Movement on board

use std::intrinsics::transmute;

use super::{piece::Piece, position::Position};

/// Movement type
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum MoveType {
    Normal,
    Promotion,
    Capture,
    DoublePawnPush,
    KCastling,
    QCastling,
    EnPassant,
    PromotionCapture,
}

impl MoveType {
    /// Number of differntes moves
    pub const NUM: usize = 8;
}

impl TryFrom<u8> for MoveType {
    type Error = String;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        if val as usize >= Self::NUM {
            return Err(format!("Unknown move type value: {}", val));
        }
        let mtype = unsafe { transmute(val) };

        Ok(mtype)
    }
}

/// Movement representation
/// [00:07]: current_position
/// [08:15]: next_position
/// [16:19]: Piece
/// [20:23]: Type
/// [24:27]: Promotion
/// [28:31]: Capture
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Move {
    val: u32,
}

impl Move {
    /// Create a new instane of `Move`
    pub fn new(current_position: u8, next_position: u8, piece: Piece, mtype: MoveType) -> Self {
        let mut val = 0u32;
        val |= current_position as u32;
        val |= (next_position as u32) << 8;
        val |= (piece as u32) << 16;
        val |= (mtype as u32) << 20;

        Self { val }
    }

    /// Returns the current position
    #[inline]
    pub fn current_position_get(self) -> u8 {
        self.val as u8
    }

    /// Returns the next position
    #[inline]
    pub fn next_position_get(self) -> u8 {
        (self.val >> 8) as u8
    }

    /// Returns the piece
    #[inline]
    pub fn piece_get(self) -> Piece {
        Piece::try_from(((self.val >> 16) & 0xF) as u8).unwrap()
    }

    /// Returns the piece type
    #[inline]
    pub fn mtype_get(self) -> MoveType {
        MoveType::try_from(((self.val >> 20) & 0xF) as u8).unwrap()
    }

    /// Returns the promotion piece
    #[inline]
    pub fn promotion_get(self) -> Piece {
        Piece::try_from(((self.val >> 24) & 0xF) as u8).unwrap()
    }

    /// Sets the promotion piece
    #[inline]
    pub fn promotion_set(&mut self, piece: Piece) {
        self.val |= (piece as u32) << 24;
    } 

    /// Returns the captured piece
    #[inline]
    pub fn capture_get(self) -> Piece {
        Piece::try_from(((self.val >> 28) & 0xF) as u8).unwrap()
    }

    /// Sets the captured piece
    #[inline]
    pub fn capture_set(&mut self, piece: Piece) {
        self.val |= (piece as u32) << 28;
    }
}
