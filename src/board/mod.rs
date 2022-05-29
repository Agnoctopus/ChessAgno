use crate::fen::Fen;

mod bit;
pub mod color;
mod normal;
mod moving;
mod piece;
mod position;

pub use bit::BitBoard;
pub use normal::NormalBoard;

/// Board representation shared behavior
pub trait Board {
    fn setup_board(&mut self, fen: Fen);
}
