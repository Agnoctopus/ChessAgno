//! Agno Chess Program

use chess_agno::board::{Board, NormalBoard};
use chess_agno::chess::ChessBoard;
use chess_agno::fen::Fen;

fn launch_game() -> Result<(), String> {
    let fen = Fen::try_from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")?;

    let mut board = NormalBoard::new();
    board.setup_board(fen);
    let chessboard = ChessBoard::new(Box::new(board));

    Ok(())
}

fn main() {
    if let Err(err) = launch_game() {
        eprintln!("Error while launching the game: {}", err);
    }
}
