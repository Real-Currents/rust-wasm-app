use super::board::{Coordinate, GamePiece, Move, PieceColor};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct GameEngine {
    board: [[Option<GamePiece>; 8]; 8],
    current_turn: PieceColor,
    move_count: u32
}

pub struct MoveResult {
    pub mv: Move,
    pub crowned: bool
}
