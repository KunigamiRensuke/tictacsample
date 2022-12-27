// #![allow(dead_code, unused_variables)]
use crate::environent::Game;
use std::{io, ops::ControlFlow};
fn main() {
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

mod environent {
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
                "Total wins by Agent1:{} {}",
                self.agent1_win,
                win_percentage!(self.agent1_win, self.game_count)
            );
            println!(
                "Total wins by Agent2:{} {}",
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

fn get_agents_to_play() -> (agent::AgentType, agent::AgentType, u32) {
    println!(
        "Hello lets play ordinary tic tac toe\n
        Enter your 2 agents of choice in correct order of play and number of turns if any\n
        1.Human 2.RandomAgent 3.Chooser Winner 4.Choose no losing 5.MCTS agent 6.MCTS solver"
    );
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    let numbers: Vec<agent::AgentType> = user_input
        .split_whitespace()
        .enumerate()
        .map(|(i, x)| agent::AgentType::create_agent_from_id(x.parse::<u32>().unwrap(), i).unwrap())
        .collect();
    let (agent_1, agent_2) = (*numbers.first().unwrap(), *numbers.get(1).unwrap());
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
    use crate::{game_module, time_module};
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    #[derive(Clone, Copy)]
    pub(crate) enum AgentType {
        Human(PerformanceStats, usize),
        Random(PerformanceStats, usize),
        WinningMoveSelector(PerformanceStats, usize),
    }
    impl AgentType {
        pub fn create_agent_from_id(id: u32, index: usize) -> Option<AgentType> {
            match id {
                1 => Some(AgentType::Human(PerformanceStats::new(), index)),
                2 => Some(AgentType::Random(PerformanceStats::new(), index)),
                3 => Some(AgentType::WinningMoveSelector(
                    PerformanceStats::new(),
                    index,
                )),
                _ => None,
            }
        }
        pub fn get_perf_stats(&self) {
            match self {
                AgentType::Human(_stats, _index) => (),
                AgentType::Random(stats, index) => {
                    println!("Random agent {}->{}", index, stats.show())
                }
                AgentType::WinningMoveSelector(stats, index) => {
                    println!("Winning agent{}->{}", index, stats.show())
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
                    let mut rng = thread_rng();
                    let moves = *board_stage.possible_moves().choose(&mut rng).unwrap();
                    my_stats.increment(clock.get_elapsed_time());
                    moves
                }
                AgentType::WinningMoveSelector(my_stats, _index) => {
                    let moves = board_stage.show_winning_move();
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

mod game_module {
    use rand::{seq::SliceRandom, thread_rng};

    enum PlayerToken {
        X,
        O,
    }
    impl PlayerToken {
        fn opponent(&self) -> PlayerToken {
            match self {
                PlayerToken::X => PlayerToken::O,
                PlayerToken::O => PlayerToken::X,
            }
        }
        fn value(&self) -> &str {
            match self {
                PlayerToken::X => "X",
                PlayerToken::O => "O",
            }
        }
    }
    pub struct TicTacToeBoard {
        x_value: u16,
        o_value: u16,
        x_is_player: PlayerToken,
    }
    impl TicTacToeBoard {
        pub fn new() -> TicTacToeBoard {
            TicTacToeBoard {
                x_value: 0,
                o_value: 0,
                x_is_player: PlayerToken::X,
            }
        }
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
            let mut rng = thread_rng();
            let moves = *self.possible_moves().choose(&mut rng).unwrap();
            moves
        }
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
        fn check_won_board(&self) -> bool {
            let last_player = self.x_is_player.opponent();
            let winner_value = match last_player {
                PlayerToken::X => self.x_value,
                PlayerToken::O => self.o_value,
            };
            check_if_win(winner_value)
        }
        fn check_tie_board(&self) -> bool {
            (self.x_value | self.o_value) == (1 << 9) - 1
        }
    }

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

    fn get_output_full(output_vec: &[String], human_output: bool) {
        if human_output {
            println!("{}", output_vec.join("\n"));
        }
    }
}
mod time_module {
    use std::time::Instant;
    pub(crate) struct StopWatch {
        start_time: Instant,
    }

    impl StopWatch {
        pub fn new() -> StopWatch {
            let current_instant = Instant::now();
            StopWatch {
                start_time: current_instant,
            }
        }

        pub fn get_elapsed_time(&mut self) -> u128 {
            self.start_time.elapsed().as_nanos()
        }
    }

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
