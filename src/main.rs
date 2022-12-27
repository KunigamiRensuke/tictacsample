// #![allow(dead_code, unused_variables)]
use std::io;

use crate::agent::{AgentType, PerformanceStats};
fn main() {
    println!(
        "Hello lets play ordinary tic tac toe\n
        Enter your 2 agents of choice in correct order of play and number of turns if any\n
        1.Human 2.RandomAgent 3.Chooser Winner 4.Choose no losing 5.MCTS agent 6.MCTS solver"
    );
    let (mut agent_1, mut agent_2, game_count) = get_agents_to_play();
    let any_human_agent = agent_1.check_human() | agent_2.check_human();
    let (agent1_stats, agent2_stats) = (PerformanceStats::new(), PerformanceStats::new());
    let (mut agent1_win, mut agent2_win, mut tie) = (0, 0, 0);
    for _game_iteration in 1..=game_count {
        let mut my_board = game_module::TicTacToeBoard::new();
        if agent_1.check_human() {
            my_board.show_game_stage_and_end(true);
        }
        for turns in 1.. {
            let action_chosen = match turns % 2 == 1 {
                true => agent_1.get_action(&my_board),
                false => agent_2.get_action(&my_board),
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
                        return;
                    }
                },
            };
            let (game_terminal, game_tied) = my_board.show_game_stage_and_end(any_human_agent);
            if game_terminal {
                get_calculation(agent_1, agent1_stats);
                get_calculation(agent_2, agent2_stats);

                if game_tied {
                    tie += 1;
                } else {
                    match turns % 2 == 1 {
                        true => agent1_win += 1,
                        false => agent2_win += 1,
                    }
                }
                break;
            }
        }
    }
    show_stats(agent_1, agent1_stats, "Agent 1");
    show_stats(agent_2, agent2_stats, "Agent 2");
    show_summary(game_count, agent1_win, agent2_win, tie);
}

fn get_calculation(agent: AgentType, agent_stats: PerformanceStats) {
    match agent {
        AgentType::HumanAgent(_) => {}
        AgentType::RandomAgent(mut a) => a.update(agent_stats),
    }
}

fn show_stats(agent: AgentType, agent_stats: PerformanceStats, agent_label: &str) {
    if !agent.check_human() {
        println!("{}->{}", agent_label, agent_stats.show());
    }
}

fn show_summary(game_count: u32, agent1_win: u32, agent2_win: u32, tie: u32) {
    macro_rules! win_percentage {
        ($agent1_win:expr, $game_count:expr) => {
            format!("{:.2}%", 100.0 * $agent1_win as f64 / $game_count as f64)
        };
    }
    println!(
        "Total games planned: {}, {}",
        game_count,
        if game_count == (agent1_win + agent2_win + tie) {
            "All according to plan"
        } else {
            "Muda Muda!!"
        }
    );
    println!(
        "Total wins by Agent1:{} {}",
        agent1_win,
        win_percentage!(agent1_win, game_count)
    );
    println!(
        "Total wins by Agent2:{} {}",
        agent2_win,
        win_percentage!(agent2_win, game_count)
    );
    println!("Total ties:{} {}", tie, win_percentage!(tie, game_count));
}

fn get_agents_to_play() -> (agent::AgentType, agent::AgentType, u32) {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    let numbers: Vec<agent::AgentType> = user_input
        .split_whitespace()
        .map(|x| match x.parse().unwrap() {
            1 => agent::AgentType::HumanAgent(PerformanceStats::new()),
            _ => agent::AgentType::RandomAgent(PerformanceStats::new()),
        })
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
        pub fn update(&mut self, other: PerformanceStats) {
            self.minimum = if self.runs == 0 {
                other.minimum
            } else {
                self.minimum.min(other.minimum)
            };
            self.maximum = self.maximum.max(other.maximum);
            self.total_time_nanoseconds += other.total_time_nanoseconds;
            self.runs += other.runs
        }
        pub fn show(&self) -> String {
            format!(
                "Min:{}, Max:{},Average:{},Runs:{}",
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
        HumanAgent(PerformanceStats),
        RandomAgent(PerformanceStats),
    }
    impl AgentType {
        pub fn get_action(&mut self, board_stage: &game_module::TicTacToeBoard) -> (u8, u8) {
            let mut clock = time_module::StopWatch::new();
            let inputs = match self {
                AgentType::HumanAgent(my_stats) => {
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
                AgentType::RandomAgent(my_stats) => {
                    let mut rng = thread_rng();
                    let moves = *board_stage.possible_moves().choose(&mut rng).unwrap();
                    my_stats.increment(clock.get_elapsed_time());
                    moves
                }
            };
            inputs
        }
        pub fn check_human(&self) -> bool {
            matches!(self, AgentType::HumanAgent(_))
        }
    }
}

mod game_module {
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
        fn check_tie_board(&self) -> bool {
            (self.x_value | self.o_value) == (1 << 9) - 1
        }
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
