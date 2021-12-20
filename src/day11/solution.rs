use crate::common::common::read_lines;

const FLASH: i32 = -1;
const STEPS: u32 = 100;
const DIMENSION: i32 = 10;

pub fn get_solution(input_path: &str) -> String {
    let mut solution: String = "".to_owned();
    let mut octopuses: Vec<Vec<i32>> = vec![];

    if let Ok(lines) = read_lines(input_path) {
        for line_result in lines {
            if let Ok(line) = line_result {
                octopuses.push(line.trim().chars().map(|c| c.to_digit(10).expect("not a digit") as i32).collect::<Vec<i32>>());
            }
        }
    }

    let mut flash_sum = 0;
    // for i in 0..STEPS {
    let mut i = 1;
    loop {
        println!("Step {} ===============>", i);
        for r in 0..octopuses.len() {
            let row = &mut octopuses[r];
            for c in 0..row.len() {
                row[c] += 1;
            }
        }
        println!("Add 1 to All:");
        println!("{}", octopuses_to_string(&octopuses));

        for r in 0..octopuses.len() {
            let row = &mut octopuses[r];
            for c in 0..row.len() {
                flash_sum += flash(&mut octopuses, r, c);
            }
        }
        println!("Flash everything:");
        println!("{}", octopuses_to_string(&octopuses));

        if all_flashed(&octopuses) {
            solution = i.to_string();
            break;
        }

        reset_flashed(&mut octopuses);

        println!("Reset:");
        println!("{}", octopuses_to_string(&octopuses));
        println!();

        i += 1;
    }
    // solution = flash_sum.to_string();

    // solution = ...;

    return solution;
}

fn flash(octopuses: &mut Vec<Vec<i32>>, r: usize, c: usize) -> usize {
    let o = octopuses[r][c];
    if o > 9 {
        // flash
        octopuses[r][c] = FLASH;
        // Loop through all the eight values surrounding the oct
        let mut flash_sum = 1;
        for i in 0..3 {
            for j in 0..3 {
                if i != 1 || j != 1 {
                    let row: i32 = r as i32 + i as i32 - 1;
                    let col: i32 = c as i32 + j as i32 - 1;
                    if row >= 0 && col >= 0 && row < DIMENSION && col < DIMENSION {
                        if octopuses[row as usize][col as usize] != FLASH {
                            octopuses[row as usize][col as usize] += 1;
                            flash_sum += flash(octopuses, row as usize, col as usize);
                        }
                    }
                }
            }
        }
        return flash_sum;
    }

    return 0;
}

fn reset_flashed(octopuses: &mut Vec<Vec<i32>>) {
    for r in 0..octopuses.len() {
        let row = &mut octopuses[r];
        for c in 0..row.len() {
            if octopuses[r][c] == FLASH {
                octopuses[r][c] = 0;
            }
        }
    }
}

fn octopuses_to_string(octopuses: &Vec<Vec<i32>>) -> String {
    let mut str_repr: String = String::new();
    for r in 0..octopuses.len() {
        let row = &octopuses[r];
        for c in 0..row.len() {
            if octopuses[r][c] == FLASH {
                str_repr.push('*');
            } else if octopuses[r][c] > 9 {
                str_repr.push('^');
            } else {
                str_repr.push(char::from_digit(octopuses[r][c] as u32, 10).expect("was not a digit"));
            }
        }
        str_repr.push('\n');
    }
    return str_repr;
}

fn all_flashed(octopuses: &Vec<Vec<i32>>) -> bool {
    for r in 0..octopuses.len() {
        let row = &octopuses[r];
        for c in 0..row.len() {
            if octopuses[r][c] != FLASH {
                return false;
            }
        }
    }
    return true;
}