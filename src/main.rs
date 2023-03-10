use std::env;
use std::fs;
use advent_of_code_2022_09::count_tail_positions;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should have been able to read {file_path}");

    println!("The short tail visited {} positions!", count_tail_positions(&contents, 2));
    println!("The long tail visited {} positions!", count_tail_positions(&contents, 10));
}
