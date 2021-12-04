use crate::common::common::read_lines;

pub fn get_solution(input_path: &str) -> String {
    let mut solution: String = "".to_owned();
    let mut first = true;
    let mut number_call_list: Vec<i32> = Vec::new();
    let mut index = 0;
    let mut board: String = String::new();
    let mut boards: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines(input_path) {
        for line_result in lines {
            if let Ok(line) = line_result {
               if first {
                   number_call_list = line.split(',').map(|n| n.parse::<i32>().unwrap())
                       .collect::<Vec<i32>>();
                   first = false;
               } else {
                   index += 1;
               }


                if line.eq("\n") {
                    // Save a new board
                    if !board.eq("") {
                        boards.push(board.clone());
                    }
                    board = String::new();
                } else {
                    board += &*(line + "\n");
                }
            }
        }
    }




    // solution = ;

    return solution;
}

fn get_