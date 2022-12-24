#![allow(warnings, unused)]
use rand::seq::SliceRandom;
use rand::Rng;
use std::time::SystemTime;
fn main() {
    let mut root = Node::new(GameState {
        board: [
            [Player::Empty, Player::Empty, Player::Empty],
            [Player::Empty, Player::Empty, Player::Empty],
            [Player::Empty, Player::Empty, Player::Empty],
        ],
        player: Player::X,
    });

    let move_state = monte_carlo_tree_search(&mut root);
    println!("Best move: {:?}", move_state);
}
#[derive(Clone, Debug)]
struct GameState {
    board: [[Player; 3]; 3],
    player: Player,
}

#[derive(Clone, Debug, PartialEq)]
enum Player {
    X,
    O,
    Empty,
}
impl Player {
    fn is_valid(&self) -> bool {
        *self != Player::Empty
    }
}

#[derive(Clone, Debug)]
struct Node {
    state: GameState,
    parent: Option<Box<Node>>,
    children: Vec<Node>,
    wins: i32,
    visits: i32,
}

impl Node {
    fn new(state: GameState) -> Self {
        Self {
            state,
            parent: None,
            children: Vec::new(),
            wins: 0,
            visits: 0,
        }
    }

    fn fully_expanded(&self) -> bool {
        self.state.possible_moves().len() == self.children.len()
    }

    fn terminal(&self) -> bool {
        self.state.game_over()
    }

    fn update(&mut self, reward: i32) {
        self.visits += 1;
        self.wins += reward;
    }

    fn ucb1(&self, parent_visits: i32) -> f64 {
        let q = self.wins as f64 / self.visits as f64;
        let p = 2.0 * (parent_visits as f64).ln() / self.visits as f64;
        q + p.sqrt()
    }

    fn best_move(&self) -> (usize, usize) {
        self.children
            .iter()
            .max_by_key(|c| c.wins)
            .unwrap()
            .state
            .last_move()
    }
}

impl GameState {
    fn possible_moves(&self) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();
        for i in 0..3 {
            for j in 0..3 {
                if self.board[i][j] == Player::Empty {
                    moves.push((i, j));
                }
            }
        }
        moves
    }

    fn game_over(&self) -> bool {
        self.winner()!= Player::Empty || self.possible_moves().is_empty()
    }

    fn winner(&self) -> Player {
        for i in 0..3 {
            if self.board[i][0].is_valid() && self.board[i][0] == self.board[i][1] && self.board[i][0] == self.board[i][2] {
                return self.board[i][0].clone();
            }
            if self.board[0][i].is_valid() && self.board[0][i] == self.board[1][i] && self.board[0][i] == self.board[2][i] {
                return self.board[0][i].clone();
            }
        }
        if self.board[0][0].is_valid() && self.board[0][0] == self.board[1][1] && self.board[0][0] == self.board[2][2] {
            return self.board[0][0].clone();
        }
        if self.board[0][2].is_valid() && self.board[0][2] == self.board[1][1] && self.board[0][2] == self.board[2][0] {
            return self.board[0][2].clone();
        }
        Player::Empty
    }
    fn reward(&self) -> i32 {
        match self.winner() {
            Player::X => 1,
            Player::O => -1,
            Player::Empty=> 0,
        }
    }

    fn apply_move(&self, (row, col): (usize, usize)) -> Self {
        let mut new_board = self.board.clone();
        new_board[row][col] = self.player.clone();
        let new_player = match self.player {
            Player::X => Some(Player::O),
            Player::O => Some(Player::X),
            Player::Empty => None
        }.unwrap();
        Self {
            board: new_board,
            player: new_player,
        }
    }

    fn last_move(&self) -> (usize, usize) {
        for i in 0..3 {
            for j in 0..3 {
                if self.board[i][j] != self.board[i][j - 1] {
                    return (i, j);
                }
            }
        }
        (0, 0)
    }
}

fn monte_carlo_tree_search(root: &mut Node) -> (usize, usize) {
    while SystemTime::now().elapsed().unwrap().as_millis() < 1000 {
        let mut leaf = &mut *root;
        let mut history = Vec::new();

        // while !leaf.terminal() && !leaf.fully_expanded() {
        //     history.push(leaf);
        //     leaf = select_child(&mut leaf);
        // }

        if !leaf.terminal() {
            leaf = expand(leaf);
        }

        let reward = rollout(&leaf);
        backpropagate(history, reward);
    }
    todo!()
    // return root.best_move();
}

fn select_child(node: &mut Node) -> &mut Node {
    let parent_visits = node.visits;
    let mut max_ucb1 = f64::NEG_INFINITY;
    let mut selected = None;
    for child in &mut node.children {
        let ucb1 = child.ucb1(parent_visits);
        if ucb1 > max_ucb1 {
            max_ucb1 = ucb1;
            selected = Some(child);
        }
    }

    if let Some(child) = selected {
        return child;
    }
    let mut rng = rand::thread_rng();
    let mut moves = node.state.possible_moves();
    moves.shuffle(&mut rng);
    let move_place = moves[0];
    let new_state = node.state.apply_move(move_place);
    let mut new_node = Node::new(new_state);
    // new_node.parent = Some(Box::new(node.clone()));
    // node.children.push(new_node.clone());
    // return node.children.last_mut().unwrap();
    todo!()
}

fn backpropagate(mut history: Vec<&mut Node>, reward: i32) {
    while let Some(node) = history.pop() {
        node.update(reward);
    }
}
fn expand(node: &mut Node) -> &mut Node {
    let mut moves = node.state.possible_moves();
    let mut rng = rand::thread_rng();
    moves.shuffle(&mut rng);
    let move_place = moves[0];
    let new_state = node.state.apply_move(move_place);
    let mut new_node = Node::new(new_state);
    new_node.parent = Some(Box::new(node.clone()));
    node.children.push(new_node.clone());
    return node.children.last_mut().unwrap();
}

fn rollout(node: &Node) -> i32 {
    let mut rng = rand::thread_rng();
    let mut state = node.state.clone();
    while !state.game_over() {
        let moves = state.possible_moves();
        let move_place = moves[rng.gen_range(0, moves.len())];
        state = state.apply_move(move_place);
    }
    state.reward()
}
