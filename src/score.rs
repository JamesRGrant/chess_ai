use chess::Color;

/// Score a board using piece-square tables.
#[derive(Copy, Clone)]
pub struct Calc {
    values: [[[i16; 64]; 2]; 7],
}
impl Calc {
    /// Do a piece-square scoring of the entire board.
    pub fn score_board(&self, board: &chess::Board, is_white: bool) -> i16 {
        let mut score: i16 = 0;
        for c in chess::ALL_COLORS {
            let score_modifier =
                if (c == Color::White && is_white) || (c == Color::Black && !is_white) {
                    1
                } else {
                    -1
                };
            for p in chess::ALL_PIECES {
                for x in board.pieces(p) & board.color_combined(c) {
                    score += score_modifier * self.values[p.to_index()][c.to_index()][x.to_index()];
                }
            }
        }
        score
    }

    /// Score the board based only on the piece moved.  (Unless complex, then score the board.)
    pub fn score_move(
        &self,
        board: &chess::Board,
        is_white: bool,
        the_move: chess::ChessMove,
        score: i16,
    ) -> i16 {
        let p_idx = board.piece_on(the_move.get_source()).unwrap().to_index();
        let c_idx = board.side_to_move().to_index();
        let o_idx = (!board.side_to_move()).to_index();

        // Add the new remove the old
        let mut delta = self.values[p_idx][c_idx][the_move.get_dest().to_index()];
        delta -= self.values[p_idx][c_idx][the_move.get_source().to_index()];

        let start_file = the_move.get_source().get_file().to_index();
        let end_file = the_move.get_dest().get_file().to_index();
        let captured = board.piece_on(the_move.get_dest());

        // En Passant: moving a pawn, the file changed (capture), their is no captured piece on dest square
        let en_passant = p_idx == 0 && (start_file != end_file) && captured.is_none();

        // Castling: king moved two files
        let castle = if start_file > end_file {
            p_idx == 5 && (start_file - end_file == 2)
        } else {
            p_idx == 5 && (end_file - start_file == 2)
        };

        // If special case (castling, en passant, promotion ), just do a full score because it is easier
        // Otherwise, do the incremental scoring
        if en_passant || castle || the_move.get_promotion().is_some() {
            let new_board = board.make_move_new(the_move);
            self.score_board(&new_board, is_white)
        } else {
            if let Some(captured) = captured {
                delta += self.values[captured.to_index()][o_idx][the_move.get_dest().to_index()];
            }
            if (board.side_to_move() == chess::Color::White) == is_white {
                score + delta
            } else {
                score - delta
            }
        }
    }

    #[rustfmt::skip]
    pub fn new() -> Self {
        // Initialize the piece-position values in a 3D array
        // 1D: [0] = PAWN, [1...5] = KNIGHT, BISHOP, ROOK, QUEEN, KING, [6] = ENDKING
        // 2D: [0] = WHITE, [1] = BLACK
        // 3D: Board position
        // Source of values: https://www.chessprogramming.org/Simplified_Evaluation_Function
        Calc {
            values: [[
// Pawn                
[0,0,0,0,0,0,0,0, 5,10,10,-20,-20,10,10,5, 5,-5,-10,0,0,-10,-5,5,   0,0,0,20,20,0,0,0,   5,5,10,25,25,10,5,5, 10,10,20,30,30,20,10,10, 50,50,50,50,50,50,50,50, 0,0,0,0,0,0,0,0],
[0,0,0,0,0,0,0,0, 50,50,50,50,50,50,50,50, 10,10,20,30,30,20,10,10, 5,5,10,25,25,10,5,5, 0,0,0,20,20,0,0,0,   5,-5,-10,0,0,-10,-5,5,   5,10,10,-20,-20,10,10,5, 0,0,0,0,0,0,0,0]],[
// Knight    
[-50,-40,-30,-30,-30,-30,-40,-50, -40,-20,0,5,5,0,-20,-40, -30,5,10,15,15,10,5,-30, -30,0,15,20,20,15,0,-30, -30,5,15,20,20,15,5,-30, -30,0,10,15,15,10,0,-30, -40,-20,0,0,0,0,-20,-40, -50,-40,-30,-30,-30,-30,-40,-50],
[-50,-40,-30,-30,-30,-30,-40,-50, -40,-20,0,0,0,0,-20,-40, -30,0,10,15,15,10,0,-30, -30,5,15,20,20,15,5,-30, -30,0,15,20,20,15,0,-30, -30,5,10,15,15,10,5,-30, -40,-20,0,5,5,0,-20,-40, -50,-40,-30,-30,-30,-30,-40,-50]],[
// Bishop
[-20,-10,-10,-10,-10,-10,-10,-20, -10,5,0,0,0,0,5,-10, -10,10,10,10,10,10,10,-10, -10,0,10,10,10,10,0,-10, -10,5,5,10,10,5,5,-10,   -10,0,5,10,10,5,0,-10,     -10,0,0,0,0,0,0,-10, -20,-10,-10,-10,-10,-10,-10,-20],
[-20,-10,-10,-10,-10,-10,-10,-20, -10,0,0,0,0,0,0,-10, -10,0,5,10,10,5,0,-10,     -10,5,5,10,10,5,5,-10,   -10,0,10,10,10,10,0,-10, -10,10,10,10,10,10,10,-10, -10,5,0,0,0,0,5,-10, -20,-10,-10,-10,-10,-10,-10,-20]],[
// Rook
[0,0,0,5,5,0,0,0, -5,0,0,0,0,0,0,-5,     -5,0,0,0,0,0,0,-5, -5,0,0,0,0,0,0,-5, -5,0,0,0,0,0,0,-5, -5,0,0,0,0,0,0,-5, 5,10,10,10,10,10,10,5, 0,0,0,0,0,0,0,0],
[0,0,0,0,0,0,0,0, 5,10,10,10,10,10,10,5, -5,0,0,0,0,0,0,-5, -5,0,0,0,0,0,0,-5, -5,0,0,0,0,0,0,-5, -5,0,0,0,0,0,0,-5, -5,0,0,0,0,0,0,-5,     0,0,0,5,5,0,0,0]],[
// Queen
[-20,-10,-10,-5,-5,-10,-10,-20, -10,0,5,0,0,0,0,-10, -10,5,5,5,5,5,0,-10, 0,0,5,5,5,5,0,-5,  -5,0,5,5,5,5,0,-5, -10,0,5,5,5,5,0,-10, -10,0,0,0,0,0,0,-10, -20,-10,-10,-5,-5,-10,-10,-20],
[-20,-10,-10,-5,-5,-10,-10,-20, -10,0,0,0,0,0,0,-10, -10,0,5,5,5,5,0,-10, -5,0,5,5,5,5,0,-5, 0,0,5,5,5,5,0,-5,  -10,5,5,5,5,5,0,-10, -10,0,5,0,0,0,0,-10, -20,-10,-10,-5,-5,-10,-10,-20]],[
// King
[20,30,10,0,0,10,30,20,           20,20,0,0,0,0,20,20,             -10,-20,-20,-20,-20,-20,-20,-10, -20,-30,-30,-40,-40,-30,-30,-20, -30,-40,-40,-50,-50,-40,-40,-30, -30,-40,-40,-50,-50,-40,-40,-30, -30,-40,-40,-50,-50,-40,-40,-30, -30,-40,-40,-50,-50,-40,-40,-30],
[-30,-40,-40,-50,-50,-40,-40,-30, -30,-40,-40,-50,-50,-40,-40,-30, -30,-40,-40,-50,-50,-40,-40,-30, -30,-40,-40,-50,-50,-40,-40,-30, -20,-30,-30,-40,-40,-30,-30,-20, -10,-20,-20,-20,-20,-20,-20,-10, 20,20,0,0,0,0,20,20,             20,30,10,0,0,10,30,20]],[

[-50,-30,-30,-30,-30,-30,-30,-50, -30,-30,0,0,0,0,-30,-30,     -30,-10,20,30,30,20,-10,-30, -30,-10,30,40,40,30,-10,-30, -30,-10,30,40,40,30,-10,-30, -30,-10,20,30,30,20,-10,-30, -30,-20,-10,0,0,-10,-20,-30, -50,-40,-30,-20,-20,-30,-40,-50],
[-50,-40,-30,-20,-20,-30,-40,-50, -30,-20,-10,0,0,-10,-20,-30, -30,-10,20,30,30,20,-10,-30, -30,-10,30,40,40,30,-10,-30, -30,-10,30,40,40,30,-10,-30, -30,-10,20,30,30,20,-10,-30, -30,-30,0,0,0,0,-30,-30,     -50,-30,-30,-30,-30,-30,-30,-50]],]
        }
    }
}

//==============================================================================

#[cfg(test)]
mod test {
    use super::Calc;
    use chess::{ChessMove, Game, Square};

    #[test]
    fn score_board() {
        let mut game = Game::new();
        let calc = Calc::new();

        // The default board should be scores of 0
        assert_eq!(calc.score_board(&game.current_position(), true), 0);
        assert_eq!(calc.score_board(&game.current_position(), false), 0);

        // Move white pawn up, gains 40 (add 20, drop -20)
        assert!(game.make_move(ChessMove::new(Square::E2, Square::E4, None)));
        assert_eq!(calc.score_board(&game.current_position(), true), 40);
        assert_eq!(calc.score_board(&game.current_position(), false), -40);

        // Black moves his knight, gains 10 (add -30, drop -40)
        assert!(game.make_move(ChessMove::new(Square::B8, Square::A6, None)));
        assert_eq!(calc.score_board(&game.current_position(), true), 30);
        assert_eq!(calc.score_board(&game.current_position(), false), -30);

        // Move white pawn up again, gains 5 (add 25, drop 20)
        assert!(game.make_move(ChessMove::new(Square::E4, Square::E5, None)));
        assert_eq!(calc.score_board(&game.current_position(), true), 35);
        assert_eq!(calc.score_board(&game.current_position(), false), -35);

        // Black moves pawn up, gains 40 (add 20, drop -20)
        assert!(game.make_move(ChessMove::new(Square::D7, Square::D5, None)));
        assert_eq!(calc.score_board(&game.current_position(), true), -5);
        assert_eq!(calc.score_board(&game.current_position(), false), 5);

        // White does en passant!, gains 25 (add 30, drop 25, capture 20)
        assert!(game.make_move(ChessMove::new(Square::E5, Square::D6, None)));
        assert_eq!(calc.score_board(&game.current_position(), true), 20);
        assert_eq!(calc.score_board(&game.current_position(), false), -20);
    }

    #[test]
    fn score_move() {
        let mut game = Game::new();
        let calc = Calc::new();

        // The default board should be scores of 0
        assert_eq!(calc.score_board(&game.current_position(), true), 0);
        assert_eq!(calc.score_board(&game.current_position(), false), 0);

        // Move white pawn up, gains 40 (add 20, drop -20)
        let e2e4 = ChessMove::new(Square::E2, Square::E4, None);
        assert_eq!(calc.score_move(&game.current_position(), true, e2e4, 0), 40);
        assert_eq!(calc.score_move(&game.current_position(), false, e2e4, 0), -40);
        assert!(game.make_move(e2e4));

        // Black moves his knight, gains 10 (add -30, drop -40)
        let b8a6 = ChessMove::new(Square::B8, Square::A6, None);
        assert_eq!(calc.score_move(&game.current_position(), true, b8a6, 40), 30);
        assert_eq!(calc.score_move(&game.current_position(), false, b8a6, -40), -30);
        assert!(game.make_move(b8a6));

        // Move white pawn up again, gains 5 (add 25, drop 20)
        let e4e5 = ChessMove::new(Square::E4, Square::E5, None);
        assert_eq!(calc.score_move(&game.current_position(), true, e4e5, 30), 35);
        assert_eq!(calc.score_move(&game.current_position(), false, e4e5, -30), -35);
        assert!(game.make_move(e4e5));

        // Black moves pawn up, gains 40 (add 20, drop -20)
        let d7d5 = ChessMove::new(Square::D7, Square::D5, None);
        assert_eq!(calc.score_move(&game.current_position(), true, d7d5, 35), -5);
        assert_eq!(calc.score_move(&game.current_position(), false, d7d5, -35), 5);
        assert!(game.make_move(d7d5));

        // White does en passant!, gains 25 (add 30, drop 25, capture 20)
        let e5tod6 = ChessMove::new(Square::E5, Square::D6, None);
        assert_eq!(calc.score_move(&game.current_position(), true, e5tod6, -5), 20);
        assert_eq!(calc.score_move(&game.current_position(), false, e5tod6, 5), -20);
    }
}
