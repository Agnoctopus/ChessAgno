use crate::fen::Fen;

mod bit;
mod normal;
mod piece;

pub use bit::BitBoard;
pub use normal::NormalBoard;

pub trait ChessBoard {
    fn setup_board(&mut self, fen: Fen);
}
