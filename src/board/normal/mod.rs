use super::ChessBoard;

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
    fn setup_board(fen: crate::fen::Fen) {
        todo!()
    }
}
