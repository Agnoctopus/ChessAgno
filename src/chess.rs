use crate::board::Board;
use crate::board::color::Color;

pub struct ChessBoard {
    board: Box<dyn Board>,
    turn: Color,
}

impl ChessBoard {
    pub fn new(board: Box<dyn Board>) -> Self {
        Self {
            board,
            turn: Color::White,
        }
    }

    #[inline]
    pub fn turn_get(&self) -> Color {
        self.turn
    }
}