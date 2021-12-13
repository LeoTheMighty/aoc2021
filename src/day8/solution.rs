use std::collections::{HashMap, HashSet};
use crate::common::common::read_lines;

pub fn get_solution(input_path: &str) -> String {
    assert_eq!("0123456", sort_string("6310254".to_owned()));
    let mut solution: String = "".to_owned();
    let mut sum = 0;

    let all_letters = HashSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g']);
    let mut len5missing = HashSet::new();
    let mut len6missing = HashSet::new();
    let mut pos_possibilities: Vec<HashSet<char>> = vec![HashSet::new(); 7];
    let mut letter_possible_positions: HashMap<char, HashSet<usize>> = HashMap::new();
    for c in 'a'..='g' {
        letter_possible_positions.insert(c, HashSet::from([0, 1, 2, 3, 4, 5, 6]));
    }
    let mut represented_letters: HashSet<char> = HashSet::new();
    let mut number_representation: Vec<HashSet<char>> = vec![HashSet::new(), 10];
    // let mut represented_letters: Vec<bool> = vec![false; 7];

    if let Ok(lines) = read_lines(input_path) {
        for line_result in lines {
            if let Ok(line) = line_result {

                let mut letter_pos_map: Vec<&str> = Vec::new();
                for (i, s) in line.split("|").enumerate() {
                    if i == 0 {
                        // not yet
                        let digit_len = digit.len();
                        if digit_len == 5 {

                        } else if digit_len == 6 {

                        }
                        let letters_set: HashSet<char> = digit.chars().collect();
                        let mut len5letters: HashSet<char> = all_letters.clone();
                        let mut len6letters: HashSet<char> = all_letters.clone();
                        for digit in s.split(" ") {
                            let possible: HashSet<usize> = len_to_possible_positions(digit_len);


                            for c in digit.chars() {
                                let intersect: HashSet<usize> = letter_possible_positions.get(&c).unwrap().intersection(&possible).map(|e| *e).collect::<HashSet<usize>>();
                                letter_possible_positions.insert(c, intersect);

                                if !represented_letters.contains(&c) {
                                    // just add to all
                                    for pos in &possible {
                                        pos_possibilities[*pos].insert(c);
                                    }
                                    represented_letters.insert(c);
                                } else {
                                    // do an AND

                                    let mut i = 0;
                                    for pos in &mut pos_possibilities {
                                        if pos.contains(&c) && !possible.contains(&i) {
                                            pos.remove(&c);
                                        }
                                        i += 1;
                                    }
                                }
                            }
                        }
                    } else if i == 1 {
                        //
                        for digit in s.split(" ") {
                            let l = digit.len();
                            if l == 2 || l == 3 || l == 4 || l == 7 {
                                sum += 1;
                            }
                        }
                    } else {
                        assert!(false);
                        println!("Ayo ?????????");
                    }
                }
            }
        }
    }

    solution = sum.to_string();

    return solution;
}

fn set_options(possible_positions: HashSet<usize>) {

}

fn len_to_possible_positions(len: usize) -> HashSet<usize> {
    return match len {
        2 => HashSet::from([2, 5]),
        3 => HashSet::from([0, 2, 5]),
        4 => HashSet::from([1, 2, 3, 5]),
        5 => HashSet::from([0, 2, 3, 4, 5, 6]),
        6 => HashSet::from([0, 1, 2, 3, 4, 5, 6]),
        7 => HashSet::from([0, 1, 2, 3, 4, 5, 6]),
        _ => HashSet::new(),
    }
}

fn sort_string(s: String) -> String {
    let mut l: Vec<char> = s.chars().collect::<Vec<char>>();
    l.sort();
    l.iter().fold(String::new(), |mut sum, v| { sum.push(*v); return sum; } )
}

fn get_number_from_positions(positions: String) -> i32 {
    return match positions.as_str() {
        "012456" => 0,
        "25" => 1,
        "02346" => 2,
        "02356" => 3,
        "1235" => 4,
        "01356" => 5,
        "013456" => 6,
        "025" => 7,
        "0123456" => 8,
        "012356" => 9,
        _ => -1,
    }
}