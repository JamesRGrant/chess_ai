use crate::Agent;
use chess::MoveGen;
use rand::seq::IteratorRandom;

/// A chess agent that makes random moves.
pub struct RandomAgent {
    rng: rand::rngs::ThreadRng,
}
impl Agent for RandomAgent {
    fn make_move(&mut self, game: &chess::Game) -> Option<chess::ChessMove> {
        MoveGen::new_legal(&game.current_position()).choose(&mut self.rng)
    }

    fn name(&self) -> String {
        "RandomAgent".to_string()
    }
}
impl RandomAgent {
    pub fn new() -> Self {
        RandomAgent {
            rng: rand::thread_rng(),
        }
    }
}

//==============================================================================
#[cfg(test)]
mod test {
    use super::*;
    use chess::Game;

    #[test]
    fn make_two_moves() {
        let mut game = Game::new();

        // Ensure a valid white and black move are made.
        assert!(game.make_move(RandomAgent::new().make_move(&game).unwrap()));
        assert!(game.make_move(RandomAgent::new().make_move(&game).unwrap()));
    }
}
