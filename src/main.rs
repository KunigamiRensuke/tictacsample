#![allow(dead_code, unused_variables)]
use crate::environment::Game;
use std::{io, ops::ControlFlow};
fn main() {
    game_play()
}

/// To create gameplay environment to test two agents for certain number of gamecount(default 1)
/// This plays agent 1 vs agent 2 and shows their win counts in percent and also the run time
fn game_play() {
    let (agent_1, agent_2, game_count) = get_agents_to_play();
    let mut first_game = Game::new(agent_1, agent_2, game_count);
    if let ControlFlow::Break(_) = first_game.start() {
        return;
    }
    let mut second_game = Game::new(agent_2, agent_1, game_count);
    if let ControlFlow::Break(_) = second_game.start() {
        return;
    }
    println!("Games have ended with no problem!")
}

mod environment {
    use std::ops::ControlFlow;

    use crate::{agent::AgentType, game_module};

    pub(crate) struct Game {
        agent_1: AgentType,
        agent_2: AgentType,
        game_count: u32,
        agent1_win: u32,
        agent2_win: u32,
        tie: u32,
    }
    impl Game {
        /// Creates a new game implementation
        pub fn new(agent_1: AgentType, agent_2: AgentType, game_count: u32) -> Self {
            Self {
                agent_1,
                agent_2,
                game_count,
                agent1_win: 0,
                agent2_win: 0,
                tie: 0,
            }
        }
        /// Starts the game with agents mentioned
        pub fn start(&mut self) -> ControlFlow<()> {
            let any_human_agent = self.agent_1.check_human() | self.agent_2.check_human();
            for _game_iteration in 1..=self.game_count {
                let mut my_board = game_module::TicTacToeBoard::new();
                if self.agent_1.check_human() {
                    my_board.show_game_stage_and_end(true);
                }
                for turns in 1.. {
                    let action_chosen = match turns % 2 == 1 {
                        true => self.agent_1.get_action(&my_board),
                        false => self.agent_2.get_action(&my_board),
                    };
                    my_board = match my_board.get_move(action_chosen) {
                        Ok(board) => board,
                        Err(e) => match e.as_str() {
                            "Already placed, try again" => {
                                println!("{}", e);
                                continue;
                            }
                            _ => {
                                println!("{} Exiting", e);
                                return ControlFlow::Break(());
                            }
                        },
                    };
                    let (game_terminal, game_tied) =
                        my_board.show_game_stage_and_end(any_human_agent);
                    if game_terminal {
                        if game_tied {
                            self.tie += 1;
                        } else {
                            match turns % 2 == 1 {
                                true => self.agent1_win += 1,
                                false => self.agent2_win += 1,
                            }
                        }
                        break;
                    }
                }
            }
            self.agent_1.get_perf_stats();
            self.agent_2.get_perf_stats();
            self.show_summary();
            ControlFlow::Continue(())
        }
        /// Gives summary of win rate vs ties
        fn show_summary(&self) {
            macro_rules! win_percentage {
                ($agent1_win:expr, $game_count:expr) => {
                    format!("{:.2}%", 100.0 * $agent1_win as f64 / $game_count as f64)
                };
            }
            println!(
                "Total games planned: {}, {}",
                self.game_count,
                if self.game_count == (self.agent1_win + self.agent2_win + self.tie) {
                    "All according to plan"
                } else {
                    "Muda Muda!!"
                }
            );
            println!(
                "Total wins by Agent1({}):{} {}",
                self.agent_1.get_name(),
                self.agent1_win,
                win_percentage!(self.agent1_win, self.game_count)
            );
            println!(
                "Total wins by Agent2({}):{} {}",
                self.agent_2.get_name(),
                self.agent2_win,
                win_percentage!(self.agent2_win, self.game_count)
            );
            println!(
                "Total ties:{} {}",
                self.tie,
                win_percentage!(self.tie, self.game_count)
            );
        }
    }
}

/// Function to obtain from user agents to play and game count
fn get_agents_to_play() -> (agent::AgentType, agent::AgentType, u32) {
    println!(
        "Hello lets play ordinary tic tac toe\n
        Enter your 2 agents of choice in correct order of play and number of turns if any\n
        1.Human 2.RandomAgent 3.Chooser Winner 4.Choose no losing 5.MCTS agent 6.MCTS solver"
    );
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    let agent_1 = agent::AgentType::create_agent_from_id(
        user_input
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .next()
            .unwrap(),
        1,
    )
    .unwrap();
    let agent_2 = agent::AgentType::create_agent_from_id(
        user_input
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .nth(1)
            .unwrap(),
        2,
    )
    .unwrap();

    let turn_count = user_input
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .nth(2)
        .unwrap_or(1);
    (agent_1, agent_2, turn_count)
}

mod agent {
    use std::io;
    #[derive(Clone, Copy)]
    pub(crate) struct PerformanceStats {
        minimum: u128,
        maximum: u128,
        total_time_nanoseconds: u128,
        runs: u128,
    }
    impl PerformanceStats {
        pub fn new() -> Self {
            Self {
                minimum: 0,
                maximum: 0,
                total_time_nanoseconds: 0,
                runs: 0,
            }
        }
        pub fn show(&self) -> String {
            format!(
                "Min:{}, Max:{},Average:{},Turns:{}",
                reformat_nano_time(self.minimum),
                reformat_nano_time(self.maximum),
                reformat_nano_time(
                    ((self.total_time_nanoseconds as f64) / (self.runs as f64)) as u128
                ),
                self.runs
            )
        }
        pub fn increment(&mut self, total_time: u128) {
            self.minimum = if self.runs == 0 {
                total_time
            } else {
                self.minimum.min(total_time)
            };
            self.maximum = self.maximum.max(total_time);
            self.total_time_nanoseconds += total_time;
            self.runs += 1;
        }
    }

    use crate::time_module::reformat_nano_time;
    use crate::{game_module, monte_carlo_tree_search, time_module};
    #[derive(Clone, Copy)]
    pub(crate) enum AgentType {
        /// Human agent
        Human(PerformanceStats, usize),
        /// Random move choosing agent
        Random(PerformanceStats, usize),
        /// Winnable move prioritising random agent
        WinningMoveSelector(PerformanceStats, usize),
        /// Non losable move prioritising random agent
        NonLosingMoveSelector(PerformanceStats, usize),
        /// MCTS method choosing agent
        MCTSAgent(PerformanceStats, usize),
    }
    impl AgentType {
        pub fn create_agent_from_id(agent_id_tag: u32, index: usize) -> Option<AgentType> {
            match agent_id_tag {
                1 => Some(AgentType::Human(PerformanceStats::new(), index)),
                2 => Some(AgentType::Random(PerformanceStats::new(), index)),
                3 => Some(AgentType::WinningMoveSelector(
                    PerformanceStats::new(),
                    index,
                )),
                4 => Some(AgentType::NonLosingMoveSelector(
                    PerformanceStats::new(),
                    index,
                )),
                5 => Some(AgentType::MCTSAgent(PerformanceStats::new(), index)),
                _ => None,
            }
        }
        pub fn get_name(self) -> String {
            match self {
                AgentType::Human(_stats, index) => format!("Human agent {}", index),
                AgentType::Random(_stats, index) => format!("Random agent {}", index),
                AgentType::WinningMoveSelector(_stats, index) => format!("Winning agent {}", index),
                AgentType::NonLosingMoveSelector(_stats, index) => {
                    format!("Non losing agent {}", index)
                }
                AgentType::MCTSAgent(_stats, index) => {
                    format!("Monte carlo agent {}", index)
                }
            }
        }
        pub fn get_perf_stats(&self) {
            match self {
                AgentType::Human(_stats, _index) => (),
                AgentType::Random(stats, _index) => {
                    println!("{}->{}", self.get_name(), stats.show())
                }
                AgentType::WinningMoveSelector(stats, _index) => {
                    println!("{}->{}", self.get_name(), stats.show())
                }
                AgentType::NonLosingMoveSelector(stats, _index) => {
                    println!("{}->{}", self.get_name(), stats.show())
                }
                AgentType::MCTSAgent(stats, _index) => {
                    println!("{}->{}", self.get_name(), stats.show())
                }
            }
        }

        pub fn get_action(&mut self, board_stage: &game_module::TicTacToeBoard) -> (u8, u8) {
            let mut clock = time_module::StopWatch::new();
            let inputs = match self {
                AgentType::Human(my_stats, _index) => {
                    println!("Enter your move (row , column)");
                    let mut user_input = String::new();
                    io::stdin().read_line(&mut user_input).unwrap();
                    let numbers: Vec<u8> = user_input
                        .split_whitespace()
                        .map(|x| x.parse().unwrap())
                        .collect();
                    let (x, y) = (*numbers.first().unwrap(), *numbers.get(1).unwrap());
                    my_stats.increment(clock.get_elapsed_time());
                    (x, y)
                }
                AgentType::Random(my_stats, _index) => {
                    let moves = board_stage.get_random_move();
                    my_stats.increment(clock.get_elapsed_time());
                    moves
                }
                AgentType::WinningMoveSelector(my_stats, _index) => {
                    let moves = board_stage.show_winning_move();
                    my_stats.increment(clock.get_elapsed_time());
                    moves
                }
                AgentType::NonLosingMoveSelector(my_stats, _index) => {
                    let moves = board_stage.show_non_losing_move();
                    my_stats.increment(clock.get_elapsed_time());
                    moves
                }
                AgentType::MCTSAgent(my_stats, _index) => {
                    let moves = monte_carlo_tree_search::tree_searcher(board_stage.clone(), 48);
                    my_stats.increment(clock.get_elapsed_time());
                    moves
                }
            };
            inputs
        }
        pub fn check_human(&self) -> bool {
            matches!(self, AgentType::Human(_, _))
        }
    }
}

mod monte_carlo_tree_search {
    use std::time::SystemTime;

    use rand::{thread_rng, Rng};

    use crate::game_module;
    #[derive(Clone)]
    struct Node {
        state: game_module::TicTacToeBoard,
        parent_location: Option<usize>,
        parent_action: Option<(u8, u8)>,
        position: usize,
        children: Option<(usize, usize)>,
        wins: i32,
        visits: u32,
    }
    impl Node {
        /// Creates a new node
        fn new(state: game_module::TicTacToeBoard, position_to_push: usize) -> Self {
            Node {
                state,
                parent_location: None,
                parent_action: None,
                children: None,
                position: position_to_push,
                wins: 0,
                visits: 0,
            }
        }
        /// Creates a child
        fn child_creation(
            state: game_module::TicTacToeBoard,
            parent_location: usize,
            parent_action: (u8, u8),
            position_to_push: usize,
        ) -> Node {
            Node {
                state,
                parent_location: Some(parent_location),
                parent_action: Some(parent_action),
                children: None,
                position: position_to_push,
                wins: 0,
                visits: 0,
            }
        }
        /// Checks if the node is a terminal node
        fn terminal(&self) -> bool {
            self.state.game_over()
        }
        /// Checks if the given node is fully expanded
        fn fully_expanded(&self) -> bool {
            if let Some((a, b)) = self.children {
                return self.state.possible_moves().len() == b - a;
            }
            false
        }

        /// Gets the max ucb value for a given node
        fn ucb1(&self, parent_visits: u32, select_ending: bool) -> f64 {
            if select_ending {
                return self.wins as f64;
            }
            let q = self.wins as f64 / self.visits as f64;
            let p = 2.0 * (parent_visits as f64).ln() / (self.visits as f64);
            q + p.sqrt()
        }

        /// Selects the best child to pursue based on ucb values
        fn select_child(&self, tree_graph: Vec<Node>, select_ending: bool) -> Option<usize> {
            let parent_visits = self.visits;
            let mut max_ucb1 = f64::NEG_INFINITY;
            let mut selected = None;
            if let Some((start, end)) = self.children {
                for child in start..end {
                    let ucb1 = tree_graph[child].ucb1(parent_visits, select_ending);
                    if ucb1 > max_ucb1 {
                        max_ucb1 = ucb1;
                        selected = Some(child)
                    }
                }
            }

            selected
        }

        /// Rolls out a given node and gives the reward for the node
        fn rollout(&self) -> i32 {
            let mut state = self.state.clone();
            let original_state = self.state.clone();
            while !state.game_over() {
                let action = state.get_random_move();
                state = state.get_move(action).unwrap();
            }
            state.reward()
                * (if state.x_plays() == original_state.x_plays() {
                    -1
                } else {
                    1
                })
        }

        /// updates the given node based on reward from previous nodes
        fn update(&mut self, reward: i32) {
            self.visits += 1;
            self.wins += reward;
        }

        /// Adds detail of children
        fn refactor_children(&mut self, initial: usize, ending: usize) {
            self.children = Some((initial, ending))
        }
    }
    /// Searches the game tree  to get the most optimal action
    pub(crate) fn tree_searcher(
        beginning_state: game_module::TicTacToeBoard,
        time_limit_in_milli: u128,
    ) -> (u8, u8) {
        let start_time = SystemTime::now();
        let multiplier = if beginning_state.x_plays() { 1 } else { -1 };
        let mut tree_graph = Vec::new();
        let starter = Node::new(beginning_state, tree_graph.len());
        tree_graph.push(starter);
        while start_time.elapsed().unwrap().as_millis() < time_limit_in_milli {
            let mut index = 0;
            let mut leaf = tree_graph.get(index).unwrap();
            let mut history = Vec::new();
            history.push(index);
            while !leaf.terminal() & leaf.fully_expanded() {
                index = leaf.select_child(tree_graph.clone(), false).unwrap();
                history.push(index);
                leaf = tree_graph.get(index).unwrap();
            }
            if !leaf.terminal() {
                let initial_position = tree_graph.len();
                let current_state = leaf.state.clone();
                for (location, action) in current_state.possible_moves().iter().enumerate() {
                    tree_graph.push(Node::child_creation(
                        current_state.get_move(*action).unwrap(),
                        index,
                        *action,
                        location,
                    ));
                }
                let final_position = tree_graph.len();
                tree_graph[index].refactor_children(initial_position, final_position);
                let mut rng = thread_rng();
                let index = rng.gen_range(initial_position, final_position);
                history.push(index);
                leaf = tree_graph.get(index).unwrap();
            }
            let reward = leaf.rollout() * multiplier;
            for (index, node_index) in history.iter().rev().enumerate() {
                tree_graph[*node_index].update(if index % 2 == 0 { reward } else { -reward });
            }
        }
        // println!("Total MCTS nodes computed:{}", tree_graph.len());
        let child_selected = tree_graph[0]
            .select_child(tree_graph.clone(), true)
            .unwrap();
        tree_graph[child_selected].parent_action.unwrap()
    }
}

mod game_module {
    use rand::{seq::SliceRandom, thread_rng};
    #[derive(Clone)]
    enum PlayerToken {
        X,
        O,
    }
    impl PlayerToken {
        /// Gives the opponent for a given player token
        fn opponent(&self) -> PlayerToken {
            match self {
                PlayerToken::X => PlayerToken::O,
                PlayerToken::O => PlayerToken::X,
            }
        }
        /// Gives string value of a given token.
        fn value(&self) -> &str {
            match self {
                PlayerToken::X => "X",
                PlayerToken::O => "O",
            }
        }
    }
    #[derive(Clone)]
    pub struct TicTacToeBoard {
        x_value: u16,
        o_value: u16,
        x_is_player: PlayerToken,
    }
    impl TicTacToeBoard {
        /// Creates  a new tic tac toe board
        pub fn new() -> TicTacToeBoard {
            TicTacToeBoard {
                x_value: 0,
                o_value: 0,
                x_is_player: PlayerToken::X,
            }
        }
        /// Shows the game stage and whether it has ended
        pub fn show_game_stage_and_end(&self, human_output: bool) -> (bool, bool) {
            let mut output_vec = Vec::new();
            if human_output {
                output_vec.push(" _ _ _ ".to_string());
                for i in 0..3u8 {
                    let mut line_vec = Vec::new();
                    for j in 0..3u8 {
                        let position = 3 * i + j;
                        let marker = if (self.x_value >> position) & 1 == 1 {
                            "X"
                        } else if (self.o_value >> position) & 1 == 1 {
                            "O"
                        } else {
                            " "
                        };
                        line_vec.push(marker)
                    }
                    output_vec.push(format!("|{}|", line_vec.join("|")));
                }
                output_vec.push(" ‾ ‾ ‾ ".to_string());
            }
            if self.check_won_board() {
                output_vec.push(format!("{} has won", self.x_is_player.opponent().value()));
                get_output_full(&output_vec, human_output);
                (true, false)
            } else if self.check_tie_board() {
                output_vec.push("Game has ended in a draw".to_string());
                get_output_full(&output_vec, human_output);
                (true, true)
            } else {
                output_vec.push(format!("{} to play", self.x_is_player.value()));
                get_output_full(&output_vec, human_output);
                (false, false)
            }
        }

        pub fn get_random_move(&self) -> (u8, u8) {
            let mut rng = thread_rng();
            let moves = *self.possible_moves().choose(&mut rng).unwrap();
            moves
        }
        /// Gets the winning move if it exists otherwise shows a random move
        pub fn show_winning_move(&self) -> (u8, u8) {
            let last_player = &self.x_is_player;
            let winner_value = match last_player {
                PlayerToken::X => self.x_value,
                PlayerToken::O => self.o_value,
            };
            for action in self.possible_moves() {
                let new_value = winner_value + (1 << (3 * action.0 + action.1));
                if check_if_win(new_value) {
                    return action;
                }
            }
            self.get_random_move()
        }
        /// Get the non losing move if it exists(winning move first, otherwise block opponent winning move),
        ///  otherwise gives a random move
        pub fn show_non_losing_move(&self) -> (u8, u8) {
            let last_player = &self.x_is_player;
            let (winner_value, loser_value) = match last_player {
                PlayerToken::X => (self.x_value, self.o_value),
                PlayerToken::O => (self.o_value, self.x_value),
            };
            let mut non_losing_move = None;
            for action in self.possible_moves() {
                let new_value = winner_value + (1 << (3 * action.0 + action.1));
                if check_if_win(new_value) {
                    return action;
                }
                if non_losing_move.is_none() {
                    let new_value = loser_value + (1 << (3 * action.0 + action.1));
                    if check_if_win(new_value) {
                        non_losing_move = Some(action)
                    }
                }
            }
            if let Some(good_move) = non_losing_move {
                return good_move;
            }
            self.get_random_move()
        }
        /// Gives a new board when given an action.
        /// Can err when user gives wrong move
        pub fn get_move(&self, action: (u8, u8)) -> Result<TicTacToeBoard, String> {
            if !((action.0 < 3) & (action.1 < 3)) {
                return Err("Wrong coordinates".to_owned());
            }
            let action_point = 3 * action.0 + action.1;
            let shift = 1 << action_point;
            if !(((self.x_value >> action_point) & 1 == 0)
                & ((self.o_value >> action_point) & 1 == 0))
            {
                return Err("Already placed, try again".to_owned());
            }
            let result_board = match self.x_is_player {
                PlayerToken::X => TicTacToeBoard {
                    x_value: self.x_value + shift,
                    o_value: self.o_value,
                    x_is_player: self.x_is_player.opponent(),
                },
                PlayerToken::O => TicTacToeBoard {
                    x_value: self.x_value,
                    o_value: self.o_value + shift,
                    x_is_player: self.x_is_player.opponent(),
                },
            };
            Ok(result_board)
        }
        /// Generates the list of possible moves
        pub fn possible_moves(&self) -> Vec<(u8, u8)> {
            let combined_fill = self.x_value | self.o_value;
            let mut points = Vec::new();
            for i in 0..3u8 {
                for j in 0..3u8 {
                    let position = 3 * i + j;
                    if (combined_fill >> position) & 1 == 0 {
                        points.push((i, j))
                    }
                }
            }
            points
        }
        pub fn x_plays(&self) -> bool {
            matches!(self.x_is_player, PlayerToken::X)
        }
        /// Check if board has been won
        fn check_won_board(&self) -> bool {
            let last_player = self.x_is_player.opponent();
            let winner_value = match last_player {
                PlayerToken::X => self.x_value,
                PlayerToken::O => self.o_value,
            };
            check_if_win(winner_value)
        }
        /// Check if board has been tied
        fn check_tie_board(&self) -> bool {
            (self.x_value | self.o_value) == (1 << 9) - 1
        }
        /// Check if board game is over
        pub fn game_over(&self) -> bool {
            self.check_won_board() | self.check_tie_board()
        }
        /// Gives a reward assuming player is X
        pub fn reward(&self) -> i32 {
            if self.check_won_board() {
                if let PlayerToken::X = self.x_is_player.opponent() {
                    return 1;
                } else {
                    return -1;
                }
            }
            0
        }
    }

    /// Checks if the given representation is winning
    fn check_if_win(winner_value: u16) -> bool {
        let three_in_a_row = 0b111;
        let row_condition = (winner_value & three_in_a_row == three_in_a_row)
            | ((winner_value >> 3) & three_in_a_row == three_in_a_row)
            | ((winner_value >> 6) & three_in_a_row == three_in_a_row);
        let column_condition =
            (winner_value & (winner_value >> 3) & (winner_value >> 6) & three_in_a_row) > 0;
        let left_diagonal_condition =
            ((winner_value & (winner_value >> 4) & (winner_value >> 8)) & 1) > 0;
        let right_diagonal_condition =
            (((winner_value >> 2) & (winner_value >> 4) & (winner_value >> 6)) & 1) > 0;
        row_condition | column_condition | left_diagonal_condition | right_diagonal_condition
    }
    /// Gives output of vector only if human asks
    fn get_output_full(output_vec: &[String], human_output: bool) {
        if human_output {
            println!("{}", output_vec.join("\n"));
        }
    }
    /// Creates a board using a 3 x 3 array of lines
    pub fn obtain_board(lines: [&str; 3], x_to_play: bool) -> TicTacToeBoard {
        let joined_lines = lines.join("");
        let mut x_value = 0;
        let mut o_value = 0;
        for i in 0..9 {
            match joined_lines.as_str().chars().nth(i).unwrap() {
                'X' => x_value += 1 << i,
                'O' => o_value += 1 << i,
                _ => {}
            }
        }
        TicTacToeBoard {
            x_value,
            o_value,
            x_is_player: if x_to_play {
                PlayerToken::X
            } else {
                PlayerToken::O
            },
        }
    }
    #[cfg(test)]
    mod tests {
        use crate::{agent::AgentType, game_module::obtain_board};
        #[test]
        fn test_board_creation() {
            let my_board = ["XXO", "XOO", "   "];
            let new_board = obtain_board(my_board, true);
            assert_eq!(new_board.x_plays(), true);
            assert_eq!(new_board.x_value, 0b1011);
            assert_eq!(new_board.o_value, 0b110100);
            new_board.show_game_stage_and_end(true);
        }
        #[test]
        fn test_winning_agent_working() {
            let my_board = ["XX ", "   ", "   "];
            let winning_agent_number = 3;
            let new_board = obtain_board(my_board, true);
            let mut test_agent = AgentType::create_agent_from_id(winning_agent_number, 1).unwrap();
            assert_eq!(test_agent.get_action(&new_board), (0, 2));

            let my_board = ["   ", " O ", " O "];
            let new_board = obtain_board(my_board, false);
            let mut test_agent = AgentType::create_agent_from_id(winning_agent_number, 2).unwrap();
            assert_eq!(test_agent.get_action(&new_board), (0, 1));

            let my_board = [" OX", " XO", " O "];
            let new_board = obtain_board(my_board, true);
            let mut test_agent = AgentType::create_agent_from_id(winning_agent_number, 3).unwrap();
            assert_eq!(test_agent.get_action(&new_board), (2, 0));
        }
        #[test]
        fn test_non_losing_agent_working() {
            let my_board = ["XX ", "   ", "   "];
            let winning_agent_number = 4;
            let new_board = obtain_board(my_board, true);
            let mut test_agent = AgentType::create_agent_from_id(winning_agent_number, 1).unwrap();
            assert_eq!(test_agent.get_action(&new_board), (0, 2));

            let my_board = ["   ", " O ", " O "];
            let new_board = obtain_board(my_board, false);
            let mut test_agent = AgentType::create_agent_from_id(winning_agent_number, 2).unwrap();
            assert_eq!(test_agent.get_action(&new_board), (0, 1));

            let my_board = [" OX", " XO", " O "];
            let new_board = obtain_board(my_board, true);
            let mut test_agent = AgentType::create_agent_from_id(winning_agent_number, 3).unwrap();
            assert_eq!(test_agent.get_action(&new_board), (2, 0));

            let my_board = [" OX", " O ", "   "];
            let new_board = obtain_board(my_board, true);
            let mut test_agent = AgentType::create_agent_from_id(winning_agent_number, 3).unwrap();
            assert_eq!(test_agent.get_action(&new_board), (2, 1));
        }
    }
}
mod time_module {
    use std::time::Instant;
    pub(crate) struct StopWatch {
        start_time: Instant,
    }

    impl StopWatch {
        /// Creates new StopWatch instance
        pub fn new() -> StopWatch {
            let current_instant = Instant::now();
            StopWatch {
                start_time: current_instant,
            }
        }

        /// Gives the elapsed time since stopwatch is started.
        pub fn get_elapsed_time(&mut self) -> u128 {
            self.start_time.elapsed().as_nanos()
        }
    }

    /// Reformats the time in nano seconds to reasonable measure of time.
    pub fn reformat_nano_time(elapsed: u128) -> String {
        let ten = 10u128;
        let elapsed_float = elapsed as f64;
        if elapsed > ten.pow(9) {
            format!("{:.3} s", elapsed_float / (ten.pow(9) as f64))
        } else if elapsed > ten.pow(6) {
            format!("{:.3} ms", elapsed_float / (ten.pow(6) as f64))
        } else if elapsed > ten.pow(3) {
            format!("{:.3} µs", elapsed_float / (ten.pow(3) as f64))
        } else {
            format!("{:.3} ns", elapsed_float)
        }
    }
}
