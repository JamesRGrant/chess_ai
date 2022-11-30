use crate::score::Calc;
use crate::Agent;
use chess::{BoardStatus, Color, MoveGen};

/// A chess agent that looks at every possible next move, and selects the best based on a piece-square scoring function.
pub struct SimpleAgent {
    calc: Calc,
}
impl Agent for SimpleAgent {
    fn make_move(&mut self, game: &chess::Game) -> Option<chess::ChessMove> {
        let board = game.current_position();
        let movegen = MoveGen::new_legal(&board);
        let mut score = -1000;
        let mut the_move = Option::None;

        for m in movegen {
            let b = board.make_move_new(m);
            let s = if b.status() == BoardStatus::Checkmate {
                1000
            } else {
                self.calc.score_board(&b, board.side_to_move() == Color::White)
            };
            if s > score {
                score = s;
                the_move = Some(m);
            }
        }
        the_move
    }

    fn name(&self) -> String {
        "SimpleAgent".to_string()
    }
}
impl SimpleAgent {
    pub fn new() -> Self {
        SimpleAgent {
            calc: Calc::new(),
        }
    }
}

//==============================================================================
#[cfg(test)]
mod test {
    use super::*;
    use chess::Game;
    use more_asserts as ma;

    #[test]
    fn make_two_moves() {
        let mut game = Game::new();
        let calc = Calc::new();

        let mut white = SimpleAgent::new();
        let mut black = SimpleAgent::new();

        // Ensure a valid white and black move are made, and that their scores improve
        let mut s = calc.score_board(&game.current_position(), true);
        assert!(game.make_move(white.make_move(&game).unwrap()));
        ma::assert_le!(s, calc.score_board(&game.current_position(), true));

        s = calc.score_board(&game.current_position(), false);
        assert!(game.make_move(black.make_move(&game).unwrap()));
        ma::assert_le!(s, calc.score_board(&game.current_position(), false));
    }
}
