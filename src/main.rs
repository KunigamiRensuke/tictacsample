#![allow(dead_code, unused_variables)]
use std::io;
fn main() {
    println!("Hello lets play ordinary tic tac toe");
    let mut my_board = game_module::TicTacToeBoard::new();
    my_board.show();
    for turn in 0..9 {
        println!("{}.Enter your move (row , column)", turn);
        let (x, y) = get_human_agent_input();
        let mut clock = time_module::StopWatch::new();
        my_board = my_board.get_move((x, y));
        my_board.show();
        clock.get_elapsed_time()
    }
}

fn get_human_agent_input() -> (u8, u8) {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    let numbers: Vec<u8> = user_input
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let (x, y) = (*numbers.first().unwrap(), *numbers.get(1).unwrap());
    (x, y)
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
        pub fn show(&self) {
            println!("{} to play", self.x_is_player.value());
            println!(" _ _ _ ");
            for i in 0..3 {
                let mut line_vec = Vec::new();
                for j in 0..3 {
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
                println!("|{}|{}|{}|", line_vec[0], line_vec[1], line_vec[2]);
            }
            println!(" ‾ ‾ ‾ ")
        }
        pub fn get_move(&self, action: (u8, u8)) -> TicTacToeBoard {
            assert!((action.0 < 3) & (action.1 < 3));
            let action_point = 3 * action.0 + action.1;
            let shift = 1 << action_point;
            assert!((self.x_value >> action_point) & 1 == 0);
            assert!((self.o_value >> action_point) & 1 == 0);
            match self.x_is_player {
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
            }
        }
        pub fn check_won_board(&self) -> bool {
            let last_player = self.x_is_player.opponent().value();
            todo!()
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

        pub fn get_elapsed_time(&mut self) {
            let elapsed_nano = self.start_time.elapsed().as_nanos();
            reformat_nano_time(elapsed_nano);
            eprintln!("Elapsed time :{}", reformat_nano_time(elapsed_nano));
            self.start_time = Instant::now();
        }
        pub fn get_partition_time(&mut self, slabs: u128) {
            let elapsed_nano = self.start_time.elapsed().as_nanos();
            eprintln!(
                "Elapsed time :{}, each part took :{}",
                reformat_nano_time(elapsed_nano),
                reformat_nano_time(elapsed_nano / slabs)
            );
            self.start_time = Instant::now();
        }
    }

    fn reformat_nano_time(elapsed: u128) -> String {
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
