use std::collections::LinkedList;
use crate::common::common::read_lines;

pub fn get_solution(input_path: &str) -> String {
    let mut solution: String = "".to_owned();
    let mut sum = 0;
    let mut scores: Vec<usize> = vec![];

    if let Ok(lines) = read_lines(input_path) {
        for line_result in lines {
            if let Ok(line) = line_result {
                let mut character_stack: LinkedList<char> = LinkedList::new();

                let mut is_corrupt = false;
                for c in line.chars() {
                    if c == '[' || c == '(' || c == '<' || c == '{' {
                        push(&mut character_stack, closing_element(c));
                    } else {
                        let el = pop(&mut character_stack);
                        if el == c {
                            // cool
                        } else {
                            // corrupted
                            let score = score_from_char(c);
                            sum += score;
                            is_corrupt = true;
                            break;
                        }
                    }
                }

                if !is_corrupt {
                    let mut autocomplete_sum = 0;
                    while character_stack.len() > 0 {
                        let el = pop(&mut character_stack);
                        let score = score2_from_char(el);
                        autocomplete_sum *= 5;
                        autocomplete_sum += score;
                    }
                    scores.push(autocomplete_sum);
                }
            }
        }
    }

    scores.sort();

    solution = scores[scores.len() / 2].to_string();
    // solution = ...;

    // return sum.to_string();
    return solution;
}

fn closing_element(for_element: char) -> char {
    return match for_element {
        '(' => ')',
        '{' => '}',
        '[' => ']',
        '<' => '>',
        _ => 'x'
    }
}

fn score_from_char(element: char) -> usize {
    return match element {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0
    }
}

fn score2_from_char(element: char) -> usize {
    return match element {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0
    }
}

fn pop(stack: &mut LinkedList<char>) -> char {
    stack.pop_back().expect("couldn't pop from the stack")
}

fn push(stack: &mut LinkedList<char>, element: char) {
    stack.push_back(element)
}
