use crate::score::Calc;
use crate::Agent;
use chess::MoveGen;

/// Tree search node containing move and score
#[derive(Clone)]
struct Node {
    the_move: Option<chess::ChessMove>,
    score: i16,
    propagate_score: i16,
    children: Vec<Node>,
}
impl Node {
    pub fn new(score: i16) -> Self {
        Self {
            the_move: None,
            score,
            propagate_score: score,
            children: Vec::new(),
        }
    }
}

/// A chess agent that looks a certain number of moves ahead.
/// Supports Monte Carlo sampling.
pub struct DepthAgent {
    calc: Calc,
    tree: Option<Node>,
    depth: u8,
}
impl Agent for DepthAgent {
    fn make_move(&mut self, game: &chess::Game) -> Option<chess::ChessMove> {
        // Prune the tree by going two levels down (our move, opponent move)
        // If first move of game or not found (due to sampling), create new root
        self.reroot(game);

        // Build the tree to the proper depth
        DepthAgent::build_tree(
            self.tree.as_mut().unwrap(),
            &game.current_position(),
            self.depth,
            game.side_to_move() == chess::Color::White,
            &self.calc,
        );

        // Find and return the best move
        let mut best_score = -1001;
        let mut best_move = None;
        for x in &self.tree.as_ref().unwrap().children {
            if x.propagate_score > best_score {
                best_score = x.propagate_score;
                best_move = x.the_move;
            }
        }
        best_move
    }

    fn name(&self) -> String {
        format!("DepthAgent({})", self.depth)
    }
}
impl DepthAgent {
    pub fn new(depth: u8) -> Self {
        DepthAgent {
            calc: Calc::new(),
            tree: None,
            depth,
        }
    }

    fn reroot(&mut self, game: &chess::Game) {
        let mut new_root = None;

        // Get the last two moves and attempt to find the node (may not exist if new or sampling)
        if game.actions().len() >= 2 {
            let move1 = match game.actions().get(game.actions().len() - 2).unwrap() {
                chess::Action::MakeMove(m) => Some(*m),
                _ => None,
            };
            let move2 = match game.actions().get(game.actions().len() - 1).unwrap() {
                chess::Action::MakeMove(m) => Some(*m),
                _ => None,
            };

            if move1.is_some() && move2.is_some() {
                for x in &mut self.tree.as_mut().unwrap().children {
                    if x.the_move == move1 {
                        for y in 0..x.children.len() {
                            if x.children[y].the_move == move2 {
                                new_root = Some(x.children.swap_remove(y));
                                break;
                            }
                        }
                    }
                }
            }
        }

        self.tree = new_root;

        if self.tree.is_none() {
            let s = self
                .calc
                .score_board(&game.current_position(), game.side_to_move() == chess::Color::White);
            self.tree = Some(Node::new(s));
        }
    }

    /// Build out the move tree to the specified depth, progating scores up
    #[allow(clippy::cast_possible_wrap)]
    #[allow(clippy::cast_possible_truncation)]
    fn build_tree(tree: &mut Node, board: &chess::Board, depth: u8, is_white: bool, calc: &Calc) {
        let movegen = MoveGen::new_legal(board);
        let is_board_white = board.side_to_move() == chess::Color::White;

        // If no more moves, check the status and stop
        // Avoid calling board.status directly because it does a MoveGen, so we might as well do it and reuse it.
        if movegen.len() == 0 {
            if *board.checkers() == chess::EMPTY {
                // Stalemate, keep score
                tree.propagate_score = tree.score;
            } else if (is_white && is_board_white) || (!is_white && !is_board_white) {
                // white lost and is scoring
                tree.propagate_score = -1000;
            } else if (!is_white && is_board_white) || (is_white && !is_board_white) {
                // White lost, black scoring, black lost, white scoring
                tree.propagate_score = 1000;
            }
        }
        // if level one exists, fill the next level
        else if tree.children.is_empty() && depth > 0 {
            // Reuse this board in the loop to avoid reallocating
            let mut new_board = chess::Board::default();
            let mut scores: Vec<i16> = Vec::new();
            for m in movegen {
                board.make_move(m, &mut new_board);
                let s = calc.score_move(board, is_white, m, tree.score);

                let mut new_node = Node {
                    the_move: Some(m),
                    score: s,
                    propagate_score: s,
                    children: Vec::new(),
                };
                DepthAgent::build_tree(&mut new_node, &new_board, depth - 1, is_white, calc);
                scores.push(new_node.propagate_score);
                tree.children.push(new_node);
            }
            tree.propagate_score = scores.iter().sum::<i16>() / scores.len() as i16;
        } else if depth > 1 {
            // The next level existed, so just build out any following if needed
            // Reuse this board in the loop to avoid reallocating
            let mut new_board = chess::Board::default();
            let mut scores: Vec<i16> = Vec::new();
            for n in &mut tree.children {
                board.make_move(n.the_move.unwrap(), &mut new_board);
                DepthAgent::build_tree(n, &new_board, depth - 1, is_white, calc);
                scores.push(n.propagate_score);
            }
            tree.propagate_score = scores.iter().sum::<i16>() / scores.len() as i16;
        }
    }
}

//==============================================================================
#[cfg(test)]
mod test {
    use super::*;
    use chess::{Board, Game};
    use more_asserts as ma;

    #[test]
    fn make_two_moves() {
        let mut game = Game::new();
        let calc = Calc::new();

        let mut white = DepthAgent::new(1);
        let mut black = DepthAgent::new(1);

        // Ensure a valid white and black move are made, and that their scores improve
        let mut s = calc.score_board(&game.current_position(), true);
        assert!(game.make_move(white.make_move(&game).unwrap()));
        ma::assert_le!(s, calc.score_board(&game.current_position(), true));

        s = calc.score_board(&game.current_position(), false);
        assert!(game.make_move(black.make_move(&game).unwrap()));
        ma::assert_le!(s, calc.score_board(&game.current_position(), false));
    }

    /// Test building a tree to depth 1
    #[test]
    fn build_tree_1() {
        let board = Board::default();
        let mut node = Node::new(0);
        let calc = Calc::new();

        // Starting board has 20 opening moves, and the average move is 6
        DepthAgent::build_tree(&mut node, &board, 1, true, &calc);
        // for n in &node.children {
        //     println!("--> {} {}", n.score, n.propagate_score);
        // }
        // println!("--> {} {}", node.score, node.propagate_score);
        assert_eq!(node.children.len(), 20);
        assert_eq!(node.score, 0);
        assert_eq!(node.propagate_score, 6);
    }

    /// Test building a tree to depth 2
    #[test]
    fn build_tree_2() {
        let board = Board::default();
        let mut node = Node::new(0);
        let calc = Calc::new();

        // Starting board has 20 opening moves, and the best looking two ahead is 70
        DepthAgent::build_tree(&mut node, &board, 2, true, &calc);
        assert_eq!(node.children.len(), 20);
        assert_eq!(node.score, 0);
        assert_eq!(node.propagate_score, 0);
    }

    /// Test building a tree to depth 3
    #[test]
    fn build_tree_3() {
        let board = Board::default();
        let mut node = Node::new(0);
        let calc = Calc::new();

        // Starting board has 20 opening moves, and the best looking three ahead is 120
        DepthAgent::build_tree(&mut node, &board, 3, true, &calc);
        assert_eq!(node.children.len(), 20);
        assert_eq!(node.score, 0);
        assert_eq!(node.propagate_score, 6);
    }

    /// Test building a tree to depth 4
    #[test]
    fn build_tree_4() {
        let board = Board::default();
        let mut node = Node::new(0);
        let calc = Calc::new();

        // Starting board has 20 opening moves, and the best looking four ahead is 140
        DepthAgent::build_tree(&mut node, &board, 4, true, &calc);
        assert_eq!(node.children.len(), 20);
        assert_eq!(node.score, 0);
        assert_eq!(node.propagate_score, 0);
    }

    /// Test building level two tree from a level 1
    #[test]
    fn build_tree_2_from_1() {
        let board = Board::default();
        let mut node = Node::new(0);
        let calc = Calc::new();

        DepthAgent::build_tree(&mut node, &board, 1, true, &calc);
        assert_eq!(node.children.len(), 20);
        assert_eq!(node.score, 0);
        assert_eq!(node.propagate_score, 6);

        // Starting board has 20 opening moves, and the best looking two ahead is 120
        DepthAgent::build_tree(&mut node, &board, 2, true, &calc);
        assert_eq!(node.children.len(), 20);
        assert_eq!(node.score, 0);
        assert_eq!(node.propagate_score, 0);
    }
}
