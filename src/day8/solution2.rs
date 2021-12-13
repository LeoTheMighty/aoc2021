use std::collections::HashSet;
use crate::common::common::read_lines;

pub fn get_solution(input_path: &str) -> String {
    let mut sum = 0;

    if let Ok(lines) = read_lines(input_path) {
        for line_result in lines {
            if let Ok(line) = line_result {
                let mut number_representation: Vec<HashSet<char>> = vec![HashSet::new(); 10];
                let mut len_5_sets: Vec<HashSet<char>> = vec![];
                let mut len_6_sets: Vec<HashSet<char>> = vec![];

                for (i, s) in line.split("|").enumerate() {
                    if i == 0 {
                        for pattern in s.split(" ") {
                            let len = pattern.len();
                            let set = set_of_char(pattern);
                            if len == 2 {
                                // 1
                                number_representation[1] = set;
                            } else if len == 3 {
                                // 7
                                number_representation[7] = set;
                            } else if len == 4 {
                                // 4
                                number_representation[4] = set;
                            } else if len == 5 {
                                len_5_sets.push(set);
                            } else if len == 6 {
                                len_6_sets.push(set);
                            } else if len == 7 {
                                // 8
                                number_representation[8] = set;
                            }
                        }
                    } else {
                        assert_eq!(len_5_sets.len(), 3);
                        for set in len_5_sets.clone() {
                            if set_difference_length(&number_representation[4], &set) == 2 {
                                number_representation[2] = set;
                            } else if set_difference_length(&number_representation[1], &set) == 0 {
                                number_representation[3] = set;
                            } else {
                                number_representation[5] = set;
                            }
                        }
                        assert_eq!(len_6_sets.len(), 3);
                        for set in len_6_sets.clone() {
                            if set_difference_length(&number_representation[7], &set) == 1 {
                                number_representation[6] = set;
                            } else if set_difference_length(&number_representation[4], &set) == 0 {
                                number_representation[9] = set;
                            } else {
                                number_representation[0] = set;
                            }
                        }

                        for n in 0..10 {
                            assert_ne!(number_representation[n].len(), 0);
                        }

                        let mut n = 0;
                        for pattern in s.split(" ") {
                            if !pattern.eq("") {
                                n *= 10;
                                n += number_from_pattern(&number_representation, pattern);
                            }
                        }

                        sum += n;
                    }
                }
            }
        }
    }

    return sum.to_string();
}

fn number_from_pattern(number_representation: &Vec<HashSet<char>>, pattern: &str) -> usize {
    for n in 0..10 {
        if set_of_char(pattern).eq(&number_representation[n]) {
            return n;
        }
    }
    return 10;
}

fn set_difference_length(s1: &HashSet<char>, s2: &HashSet<char>) -> usize {
    s1.difference(s2).collect::<HashSet<&char>>().len()
}

fn set_of_char(str: &str) -> HashSet<char> {
    str.chars().collect::<HashSet<char>>()
}