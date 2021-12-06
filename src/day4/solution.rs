use crate::common::common::read_lines;

pub fn get_solution(input_path: &str) -> String {
    let mut solution: String = "".to_owned();
    let mut first = true;
    let mut number_call_list: Vec<i32> = vec![];
    let mut index = 0;
    let mut board: Vec<Vec<i32>> = vec![];
    let mut boards: Vec<Vec<Vec<i32>>> = vec![];

    if let Ok(lines) = read_lines(input_path) {
        for line_result in lines {
            if let Ok(line) = line_result {
                // println!("{}", line);
                if first {
                   number_call_list = line.split(',').map(|n| n.parse::<i32>().unwrap())
                       .collect::<Vec<i32>>();
                   first = false;
                    continue;
               } else {
                   index += 1;
               }


                if line.eq("") {
                    // Save a new board
                    // println!("{}", board.len());
                    if board.len() != 0 {
                        boards.push(board);
                    }
                    board = vec![];
                } else {
                    let mut row: Vec<i32> = vec![];
                    // println!("splitting {} by spaces", line);
                    for n in line.split(" ") {
                        if !n.eq("") {
                            // println!("Parsing {} into int", n);
                            row.push(n.parse().unwrap());
                        }
                    }
                    board.push(row);
                }

            }
        }

        boards.push(board);
    }


    // for b in boards {
    //     print!("{}", print_board(&b));
    // }

    let boards_len = boards.len();
    let mut won = vec![false; boards_len];
    let mut num_won = 0;

    for number_called in number_call_list {
        // println!("called {}", number_called);
        let mut i = 0;
        for b in &mut boards {
            update_board(b, number_called);
            // println!("{}", print_board(b));

            if !won[i] && has_won(b)  {
                won[i] = true;

                let sum = sum_board(b);
                println!("Someone won! Board:\n{}\n sum = {}, number_called = {}", print_board(b), sum, number_called);

                num_won += 1;
                if num_won == boards_len {
                    return (sum * number_called).to_string();
                }

                // return (sum * number_called).to_string();
            }

            i += 1;
        }

    }



    // solution = ;

    return solution;
}



fn print_board(board: &Vec<Vec<i32>>) -> String {
    let mut printed: String = String::new();
    for row in board {
        for n in row {
            if n == &-1 {
                printed += " __ ";
            }
            else {
                if n > &9 {
                    printed += " ";
                } else {
                    printed += "  ";
                }
                printed.push_str(&n.to_string());
                printed += " ";
            }
        }
        printed += "\n";
    }
    printed += "\n";
    return printed;
}

fn sum_board(board: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for row in board {
        for n in row {
           if n != &-1 {
               sum += n;
           }
        }
    }
    return sum;
}

fn update_board(board: &mut Vec<Vec<i32>>, number_called: i32) {
    for row in board {
        for i in 0..row.len() {
            if row[i] == number_called {
                row[i] = -1;
            }
        }
    }
}

fn has_won(board: &Vec<Vec<i32>>) -> bool {
    let mut rows: Vec<i32> = vec![0, 0, 0, 0, 0];
    let mut columns: Vec<i32> = vec![0, 0, 0, 0, 0];
    // let mut d1: i32 = 0;
    // let mut d2: i32 = 0;
    for r in 0..board.len() {
        let row = board.get(r).unwrap();
        for c in 0..row.len() {
            let e = row.get(c).unwrap();
            if e == &-1 {
                rows[r] += 1;
                columns[c] += 1;
                // if r == c {
                //     d1 += 1;
                // }
                // if r + c == board.len() - 1 {
                //     d2 += 1;
                // }
                if rows[r] == 5 || columns[c] == 5 {// || d1 == 5 || d2 == 5 {
                    return true;
                }
            }
        }
    }
    return false;
}

fn get_empty_board() -> Vec<Vec<i32>> {
    return vec![];
}