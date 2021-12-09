mod common { pub mod common; }
mod day9;

fn main() {
    common::common::print_solution(
        day9::solution::get_solution("./src/day9/input.txt"),
        day9::solution::get_solution("./src/day9/input_test.txt"),
        "1134"
    );
}

