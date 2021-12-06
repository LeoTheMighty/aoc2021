use crate::common::common::read_lines;

pub fn get_solution(input_path: &str) -> String {
    let mut ocean_floor: Vec<Vec<i32>> = vec![];

    if let Ok(lines) = read_lines(input_path) {
        for line_result in lines {
            if let Ok(line) = line_result {
                let split = line.split(" ").collect::<Vec<&str>>();
                let start = split[0].split(",").collect::<Vec<&str>>();
                let end = split[2].split(",").collect::<Vec<&str>>();

                let start_x = start[0].parse().unwrap();
                let start_y = start[1].parse().unwrap();
                let end_x = end[0].parse().unwrap();
                let end_y = end[1].parse().unwrap();



                expand_ocean_floor(
                    &mut ocean_floor,
                    std::cmp::max(start_x, end_x) + 1,
                    std::cmp::max(start_y, end_y) + 1,
                );

                draw_line(&mut ocean_floor, start_x, start_y, end_x, end_y);

                // println!("{},{} -> {},{}", start_x, start_y, end_x, end_y);
                // println!("{}", print_ocean_floor(&ocean_floor));
            }
        }
    }

    let p = print_ocean_floor(&ocean_floor);
    println!("{}", p);

    return count_highs(&ocean_floor).to_string();
    // solution = ...;

    // return solution;
}

fn draw_line(ocean_floor: &mut Vec<Vec<i32>>, start_x: usize, start_y: usize, end_x: usize, end_y: usize) {
    let x_difference = i32::abs(end_x as i32 - start_x as i32);
    let y_difference = i32::abs(end_y as i32 - start_y as i32);

    if x_difference > 0 && y_difference > 0 {
        // diagonal line
        assert_eq!(x_difference, y_difference);

        let x_inc = (end_x as i32 - start_x as i32) / i32::abs(end_x as i32 - start_x as i32);
        let y_inc = (end_y as i32 - start_y as i32) / i32::abs(end_y as i32 - start_y as i32);

        for i in 0..x_difference+1 {
            ocean_floor[(start_y as i32 + (i * y_inc)) as usize][(start_x as i32 + (i * x_inc)) as usize] += 1;
        }
    } else if x_difference > 0 {
        let x = std::cmp::min(start_x, end_x);
        for i in 0..x_difference+1 {
            ocean_floor[end_y][x + i as usize] += 1;
        }
    } else if y_difference > 0 {
        let y = std::cmp::min(start_y, end_y);
        for i in 0..y_difference+1 {
            ocean_floor[y + i as usize][end_x] += 1;
        }
    }
}

fn print_ocean_floor(ocean_floor: &Vec<Vec<i32>>) -> String {
    let mut printed: String = String::new();
    for row in ocean_floor {
        for n in row {
            printed += &*n.to_string();
        }
        printed += "\n";
    }
    return printed;
}


fn count_highs(ocean_floor: &Vec<Vec<i32>>) -> i32 {
    let mut highs = 0;
    for row in ocean_floor {
        for n in row {
            if n >= &2 {
                highs += 1;
            }
        }
    }
    return highs;
}

fn expand_ocean_floor(ocean_floor: &mut Vec<Vec<i32>>, mut width: usize, mut height: usize) {
    let ocean_floor_height: usize = ocean_floor.len();
    let ocean_floor_width: usize = if ocean_floor_height == 0 { 0 } else { ocean_floor[0].len() };

    width = std::cmp::max(ocean_floor_width, width);
    height = std::cmp::max(ocean_floor_height, height);

    if ocean_floor_width != 0 && width > ocean_floor_width {
        for i in 0..ocean_floor_height {
            ocean_floor[i].append(&mut vec![0; width - ocean_floor_width]);
        }
        // for row in ocean_floor {
        //     row.append(&mut vec![0; width - ocean_floor_width]);
        // }
    }

    if height > ocean_floor_height {
        ocean_floor.append(&mut vec![vec![0; width]; height - ocean_floor_height]);
    }
}
