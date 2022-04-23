//! Agno Chess Program

use chess_agno::board::{ChessBoard, NormalBoard};
use chess_agno::fen::Fen;

fn launch_game() -> Result<(), String> {
    let fen = Fen::try_from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")?;

    let mut chessboard = NormalBoard::new();
    chessboard.setup_board(fen);

    Ok(())
}

fn main() {
    if let Err(err) = launch_game() {
        eprintln!("Error while launching the game: {}", err);
    }
}
