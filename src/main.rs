mod common { pub mod common; }
mod day10;

fn main() {
    common::common::print_solution(
        day10::solution::get_solution("./src/day10/input.txt"),
        day10::solution::get_solution("./src/day10/input_test.txt"),
        "288957"
    );
}

