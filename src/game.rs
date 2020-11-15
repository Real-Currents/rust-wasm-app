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

impl GameEngine {
    pub fn new() -> GameEngine {
        let mut engine = GameEngine {
            board: [[None; 8]; 8],
            current_turn: PieceColor::Black,
            move_count: 0,
        };
        engine.initialize_pieces();
        engine
    }

    pub fn initialize_pieces(&mut self) {
        [ 1, 3, 5, 7, 0, 2, 4, 6, 1, 3, 5, 7 ]
            .iter()
            .zip([ 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2 ].iter())
            .map(|(a, b)| (*a as usize, *b as usize))
            .for_each(|(x, y)| {
                self.board[x][y] = Some(GamePiece::new(PieceColor::White));
            });

        [ 0, 2, 4, 6, 1, 3, 5, 7, 0, 2, 4, 6 ]
            .iter()
            .zip([ 5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7 ].iter())
            .map(|(a, b)| (*a as usize, *b as usize))
            .for_each(|(x, y)| {
                self.board[x][y] = Some(GamePiece::new(PieceColor::Black));
            })
    }

    fn legal_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        for col in 0..8 {
            for row in 0..8 {
                if let Some(piece) = self.board[col][row] {
                    if piece.color == self.current_turn {
                        let loc = Coordinate(col, row);
                        let &mut vmoves: Vec<Move> = self.valid_moves_from(loc);
                        moves.append(vmoves);
                    }
                }
            }
        }
        moves
    }

    fn valid_moves_from(&self, loc: Coordinate) -> Vec<Moves> {
        let Coordinate(x, y) = loc;

        if let Some(p) = self.board[x][y] {
            let mut jumps = loc
                .jump_targets_from()
                .filter(|t| self.valid_jump(&p, &loc, &t))
                .map(|ref t| Move {
                    from: loc.clone(),
                    to: t.clone()
                }).collect::<Vec<Move>>();
            let mut moves = loc
                .move_targets_from()
                .filter(|t| self.valid_move(&p, &loc, &t))
                .map(|ref t| Move {
                    from: loc.clone(),
                    to: t.clone()
                }).collect::<Vec<Move>>();
            jumps.append(&mut moves);
            jumps

        } else {
            Vec::new()
        }
    }

    pub fn move_piece(&mut self, mv: &Move) -> Result<MoveResult, ()> {
        let legal_moves = self.local_moves();

        if (!legal_moves.contains(mv)) {
            return Err(());
        } else {
            let Coordinate(fx, fy) = mv.from;
            let Coordinate(tx, ty) = mv.to;
            let piece = self.board[fx][fy].unwrap();
            let midpiece_coordinate = self.midpiece_coordinate(fx, fy, tx, ty);

            if let Some(Coordinate(x, y)) = midpiece_coordiante {
                self.board[x][y] = None;
            }

            self.board[tx][ty] = Some(piece);
            self.board[fx][fy] = None;

            let crowned = if self.should_crown(piece, mv.to) {
                self.crown_piece(mv.to);
                true
            } else {
                false
            };
            self.advance_turn();

            Ok(MoveResult {
                mv: mv.clone(),
                crowned: crowned
            })
        }
    }
}
