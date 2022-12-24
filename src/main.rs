#![allow(dead_code, unused_variables)]

fn main() {
    let mut clock = time_module::StopWatch::new();
    println!("Hello lets play ordinary tic tac toe");
    clock.get_elapsed_time();
    println!("1");
    println!("2");
    println!("3");
    clock.get_partition_time(3)
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
