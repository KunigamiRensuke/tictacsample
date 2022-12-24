#![allow(dead_code, unused_variables)]

fn main() {
    println!("Hello lets play ordinary tic tac toe");
    for i in 0..3 {
        println!("{}", i)
    }
    let my_board = TicTacToeBoard::new();
    my_board.show()
}

struct TicTacToeBoard {
    x_value: u16,
    o_value: u16,
}
impl TicTacToeBoard {
    fn new() -> TicTacToeBoard {
        TicTacToeBoard {
            x_value: 0,
            o_value: 0,
        }
    }
    fn show(&self) {
        println!(" _ _ _ ");
        for i in 0..3 {
            let mut line_vec = Vec::new();
            for j in 0..3 {
                if self.x_value >> (3 * i + j) == 1 {
                    line_vec.push("X")
                } else if self.o_value >> (3 * i + j) == 1 {
                    line_vec.push("O")
                } else {
                    line_vec.push(" ")
                }
            }
            println!("|{}|{}|{}|", line_vec[0], line_vec[1], line_vec[2]);
        }
        println!(" ‾ ‾ ‾ ")
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
            println!("Elapsed time :{}", reformat_nano_time(elapsed_nano));
            self.start_time = Instant::now();
        }
        pub fn get_partition_time(&mut self, slabs: u128) {
            let elapsed_nano = self.start_time.elapsed().as_nanos();
            println!(
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
            return format!("{:.3} s", elapsed_float / (ten.pow(9) as f64));
        } else if elapsed > ten.pow(6) {
            return format!("{:.3} ms", elapsed_float / (ten.pow(6) as f64));
        } else if elapsed > ten.pow(3) {
            return format!("{:.3} µs", elapsed_float / (ten.pow(3) as f64));
        } else {
            return format!("{:.3} ns", elapsed_float);
        }
    }
}
