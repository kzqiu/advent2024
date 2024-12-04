pub mod problems;
pub mod util;

use problems::p1;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    assert_eq!(args.len(), 2, "Incorrect number of arguments.");

    let day: i32 = args[1].parse::<i32>().unwrap();
    let path = format!("data/p{}.txt", day);

    match day {
        1 => p1::run(&path),
        _ => {}
    }
}
