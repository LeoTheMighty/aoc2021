use crate::common::common::read_lines;
use std::collections::LinkedList;

pub fn get_solution(input_path: &str) -> String {
    let mut solution: String = "".to_owned();

    // Represents how many fish exist of each life span.
    // index = 0 : how many fish have are at a life timer of 0.
    let mut initial_fish_lives: Vec<i64> = vec![0; 9];
    let mut fish_lives: LinkedList<i64> = LinkedList::new();

    let mut fish: Vec<u8> = vec![];
    if let Ok(lines) = read_lines(input_path) {
        for line_result in lines {
            if let Ok(line) = line_result {
                // fish = line.split(",").map(|p| p.parse::<i32>().unwrap() as u8).collect::<Vec<u8>>();
                for f in line.split(",").map(|p| p.parse::<usize>().unwrap()) {
                    initial_fish_lives[f] += 1;
                }
            }
        }
    }

    for f in initial_fish_lives {
        fish_lives.push_back(f);
    }

    for i in 0..256 {
        // skip_day(&mut fish);
        pass_day_lives(&mut fish_lives);
    }

    let mut s = 0;
    for f in fish_lives {
        s += f;
    }
    solution = s.to_string();
    // solution = fish.len().to_string();

    return solution;
}

fn pass_day_lives(fish_lives: &mut LinkedList<i64>) {
    let rebirth_fish = fish_lives.pop_front().unwrap();

    let f7 = fish_lives.pop_back().unwrap();
    let f6 = fish_lives.pop_back().unwrap();

    fish_lives.push_back(f6 + rebirth_fish);
    fish_lives.push_back(f7);
    fish_lives.push_back(rebirth_fish);
}

fn skip_day(fish: &mut Vec<u8>) {
    for i in 0..fish.len() {
        if fish[i] == 0 {
            fish[i] = 6;
            fish.push(8);
        } else {
            fish[i] -= 1;
        }
    }
}
