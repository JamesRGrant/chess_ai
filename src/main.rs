use chess::{Game, GameResult};
mod agent_depth;
mod agent_random;
mod agent_simple;
mod agent_thread;
mod score;
use agent_depth::DepthAgent;
use agent_random::RandomAgent;
use agent_simple::SimpleAgent;
use agent_thread::ThreadAgent;

/// Common interface for player agents.
trait Agent {
    /// Given a game, return the best move.
    fn make_move(&mut self, game: &chess::Game) -> Option<chess::ChessMove>;

    /// Return a custom name for the implementation.
    fn name(&self) -> String;
}

fn main() {
    play_game(Box::new(SimpleAgent::new()), Box::new(RandomAgent::new()), 100);
    play_game(Box::new(RandomAgent::new()), Box::new(SimpleAgent::new()), 100);
    // play_game(Box::new(ThreadAgent::new(3)), Box::new(RandomAgent::new()), 100);
    play_game(Box::new(ThreadAgent::new(3)), Box::new(DepthAgent::new(1)), 100);
    // play_game(Box::new(ThreadAgent::new(4)), Box::new(RandomAgent::new()), 100);
}

/// Run the specified number of games with the player agents provided.
#[allow(clippy::cast_precision_loss)] // for u32 to f32
fn play_game(mut white: Box<dyn Agent>, mut black: Box<dyn Agent>, iterations: u32) {
    let mut wins = [0, 0, 0];
    let mut moves: Vec<f32> = Vec::new();

    // Output the agent names so we know who is playing
    println!("{} vs {}", white.name(), black.name());

    // Play the requested number of games
    let start = std::time::Instant::now();
    for _i in 0..iterations {
        let mut move_count = 0.0;
        let mut game = Game::new();

        while game.result().is_none() {
            // Asserts are there to ensure a valid move was given and made
            if game.side_to_move() == chess::Color::White {
                assert!(game.make_move(white.make_move(&game).unwrap()));
            } else {
                assert!(game.make_move(black.make_move(&game).unwrap()));
            }
            move_count += 1.0;

            // This library requires you to declare a draw vs being automatic
            if game.can_declare_draw() {
                game.declare_draw();
            }
        }
        moves.push(move_count);

        match game.result().unwrap() {
            GameResult::WhiteCheckmates | GameResult::BlackResigns => wins[0] += 1,
            GameResult::WhiteResigns | GameResult::BlackCheckmates => wins[1] += 1,
            _ => wins[2] += 1,
        }
    }

    // Calculate the averages (use full turns, where white + black = 1 full turn)
    let elapsed = start.elapsed();
    let m_avg: f32 = moves.iter().sum::<f32>() / iterations as f32 / 2.0;
    let d_avg = elapsed / iterations;
    println!("  {:?}, avg {:.1} full turns, avg {:.2?}, total {:.2?}", wins, m_avg, d_avg, elapsed);
}
