use super::position::Rank;


/// Color representation of a side
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum Color {
    None = 0,
    White,
    Black
}

/// Promotion rank associated with a color
const PROMOTION_RANK_COLOR: [Rank; 3] = [
    Rank::None,
    Rank::Seven,
    Rank::Two,
];

/// Double push rank associated with a color
const DOUBLE_PUSH_RANK_COLOR: [Rank; 3] = [
    Rank::None,
    Rank::Two,
    Rank::Seven,
];

impl Color {
    /// Returns the promotion rank f
    #[inline]
    pub fn promotion_rank(self) -> Rank {
        PROMOTION_RANK_COLOR[self as usize]
    }

    /// Returns the double push rank
    #[inline]
    pub fn double_push_rank(self) -> Rank {
        DOUBLE_PUSH_RANK_COLOR[self as usize]
    }
}