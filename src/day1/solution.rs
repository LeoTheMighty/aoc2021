use std::collections::LinkedList;
use crate::common::common::read_lines;

pub fn print_solution() {
  let mut num_increased = 0;
  let mut prev_depth: i32 = 999999999;

  let mut window: LinkedList<i32> = LinkedList::new();

  if let Ok(lines) = read_lines("./src/day1/input.txt") {
    for line in lines {
      if let Ok(line_result) = line {
        if let Ok(depth) = line_result.parse::<i32>() {
          window.push_front(depth);
          if window.len() > 3 {
            window.pop_back();
          }
          if window.len() == 3 {
            let depth_sum: i32 = window.iter().sum();
            println!("{}", depth_sum);
            if depth_sum > prev_depth {
              num_increased += 1;
            }
            prev_depth = depth_sum;
          }
        }
      }
    }
  }

  println!("{}", num_increased);
}
