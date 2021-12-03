use crate::common::common::read_lines;

pub fn get_solution(input_path: &str) -> String {
  let mut pos = 0;
  let mut depth = 0;
  let mut aim = 0;
  if let Ok(lines) = read_lines(input_path) {
    for line_result in lines {
      if let Ok(line) = line_result {
        let parts = line.split(" ").collect::<Vec<&str>>();
        let command = parts[0];
        let amount: i32 = parts[1].parse::<i32>().unwrap();

        if command == "forward" {
          pos += amount;
          depth += aim * amount;
        }
        else if command == "up" {
          aim -= amount;
        }
        else if command == "down" {
          aim += amount;
        }
      }
    }
  }

  return (pos * depth).to_string();
}
