use crate::common::common::read_lines;

pub fn get_solution(input_path: &str) -> String {
    let mut solution: String = "".to_owned();

    let mut heights: Vec<Vec<i32>> = Vec::new();
    let mut hh: usize = 0;
    let mut hw: usize = 0;

    if let Ok(lines) = read_lines(input_path) {
        for line_result in lines {
            if let Ok(line) = line_result {
                hh += 1;
                hw = line.len();
                heights.push(line.chars().map(|c| c.to_string().parse().expect("number not in input")).collect());
            }
        }
    }


    let mut sum = 0;
    let mut lowest : Vec<(usize, usize)> = Vec::new();
    for (r, row) in heights.iter().enumerate() {
        for (c, height) in row.iter().enumerate() {
            if is_lowest(&heights, hh, hw, r, c, height) {
                sum += (height + 1);
                lowest.push((r, c));
            }
        }
    }

    let mut sizes: Vec<usize> = Vec::new();
    for (r, c) in lowest {
        println!("BASIN ({}, {})===========:", r, c);
        let mut copy = copy_heights(&heights);
        assert_eq!(copy.len(), heights.len());
        let size = size_of_basin(&mut copy, hh, hw, r, c);
        sizes.push(size);
        print_heights(&copy);
        println!("SIZE^ = {}", size);
        // sum2 *= size;
    }

    let mut sum2 = 1;
    let mut i = 3;
    sizes.sort();
    sizes.reverse();
    println!("{:?}", sizes);
    for s in sizes {
        println!("FINAL: size = {}", s);
        sum2 *= s;

        i -= 1;
        if i <= 0 {
            break;
        }
    }

    solution = sum2.to_string();

    // solution = ...;

    return solution;
}

fn size_of_basin(heights: &mut Vec<Vec<i32>>, hh: usize, hw: usize, r: usize, c: usize) -> usize {
    let height = &heights[r][c];
    if height == &-1 || !is_lowest(heights, hh, hw, r, c, height) {
        return 0;
    }
    println!("HOLE: ({}, {}) = {}", r, c, height);
    let mut size = 1;
    heights[r][c] = -1;

    if r > 0 {
        size += size_of_basin(heights, hh, hw, r - 1, c);
    }
    if c > 0 {
        size += size_of_basin(heights, hh, hw, r, c - 1);
    }
    if r < (hh - 1) {
        size += size_of_basin(heights, hh, hw, r + 1, c);
    }
    if c < (hw - 1) {
        size += size_of_basin(heights, hh, hw, r, c + 1);
    }

    return size;
}

fn print_heights(heights: &Vec<Vec<i32>>) {
    for row in heights {
        for n in row {
            if n == &-1 {
                print!("x");
            }
            else {
                print!("{}", n);
            }
        }
        println!();
    }
}

fn copy_heights(heights: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut heights_copy: Vec<Vec<i32>> = Vec::new();
    for row in heights {
        let mut row_copy: Vec<i32> = Vec::new();
        for n in row {
            row_copy.push(*n);
        }
        heights_copy.push(row_copy);
    }
    return heights_copy;
}

fn is_lowest(heights: &Vec<Vec<i32>>, hh: usize, hw: usize, r: usize, c: usize, height: &i32) -> bool {
    // println!("TEST HEIGHT: {} ({}, {})!!", height, r, c);
    if height == &9 {
        return false;
    }
    for i in 0..3 {
        for j in 0..3 {
            if (i == 1 || j == 1) && !(i == 1 && j == 1) {
                let y: i32 = r as i32 + i as i32 - 1;
                let x: i32 = c as i32 + j as i32 - 1;
                if x >= 0 && y >= 0 && (x as usize) < hw && (y as usize) < hh {
                    let h = &heights[y as usize][x as usize];
                    // println!("test: {} ({}, {})", h, y, x);
                    if h != &-1 && height > h {
                        return false;
                    }
                }

            }
        }
    }
    // println!("PASSED LOWEST TEST!!!");
    return true;
}
