use crate::common::common::read_lines;

pub fn get_solution(input_path: &str) -> String {
    let mut solution: String = "".to_owned();
    let mut positions: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines(input_path) {
        for line_result in lines {
            if let Ok(line) = line_result {
                positions = line.split(",")
                    .map(|s| s.parse().expect("Not a number!"))
                    .collect::<Vec<i32>>();

            }
        }
    }

    let mut min_value: i32 = 999999999;
    for test_p in *positions.iter().min().unwrap()..*positions.iter().max().unwrap() {
        // test
        let mut val = 0;
        for p in &positions {
            let n = i32::abs(p - test_p);
            val += (n * (n + 1) / 2);
            // val += i32::abs(p - test_p);
        }

        min_value = i32::min(val, min_value);
    }

    solution = min_value.to_string();
    // solution = ...;

    return solution;
}
