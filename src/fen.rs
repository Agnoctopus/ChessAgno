pub struct Fen {}

impl Fen {
    /// Create a new `Fen` instance
    pub fn new(fen_repr: &str) -> Result<Self, String> {
        // Split input into tokens
        let tokens: Vec<&str> = fen_repr.split(" ").collect();

        // Parse fen from tokens
        Self::from_tokens(tokens)
    }

    fn parse_pieces_from_token(token: &str) {}

    fn parse_side_to_mode_from_token(token: &str) {}

    fn parse_castling_from_token(token: &str) {}

    fn parse_en_passant_from_token(token: &str) {}

    fn parse_halfmove_clock_from_token(token: &str) {}

    fn parse_fullmove_counter_from_token(token: &str) {}

    fn from_tokens(tokens: Vec<&str>) -> Result<Self, String> {
        // Check correct number of tokens
        if tokens.len() != 6 {
            return Err(format!(
                "Bad number of token in fen: got {} (expected 6)",
                tokens.len()
            ));
        }

        // Parse each fields
        Self::parse_pieces_from_token(tokens[0]);
        Self::parse_side_to_mode_from_token(tokens[1]);
        Self::parse_castling_from_token(tokens[2]);
        Self::parse_en_passant_from_token(tokens[3]);
        Self::parse_halfmove_clock_from_token(tokens[4]);
        Self::parse_fullmove_counter_from_token(tokens[5]);

        Err("todo".to_string())
    }
}

impl TryFrom<&str> for Fen {
    type Error = String;

    fn try_from(fen_repr: &str) -> Result<Self, Self::Error> {
        Self::new(fen_repr)
    }
}
