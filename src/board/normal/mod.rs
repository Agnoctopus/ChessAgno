use super::ChessBoard;
use crate::fen::Fen;

mod board;

/// Normal board
pub struct NormalBoard {}

impl NormalBoard {
    /// Create a new `NormalBoard` instance
    pub fn new() -> Self {
        Self {}
    }
}

impl ChessBoard for NormalBoard {
    fn setup_board(&mut self, fen: Fen) {
        todo!()
    }
}
