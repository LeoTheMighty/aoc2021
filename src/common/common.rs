use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn just_print_solution(solution: String) {
    println!("{}", solution);
}

pub fn print_solution(solution: String, test_solution: String, test_answer: &str) -> bool {
    just_print_solution(solution);
    println!("Testing Expected: {} vs Actual: {}", test_solution, test_answer);
    if test_solution.eq(test_answer) {
        eprintln!("DOES NOT PASS TEST CASE!!! Expected: \"{}\", Actual: \"{}\"", test_answer, test_solution);
        return false;
    }
    println!("passes the test case :)");
    return true;
}