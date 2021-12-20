mod common { pub mod common; }
mod day11;

fn main() {
    common::common::print_solution(
        day11::solution::get_solution("./src/day11/input.txt"),
        day11::solution::get_solution("./src/day11/input_test.txt"),
        "1656"
    );
}

