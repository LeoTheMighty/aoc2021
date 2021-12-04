use crate::common::common::read_lines;

pub fn get_solution(input_path: &str) -> String {
  let mut first_time = true;

  let mut len: usize = 1;
  let mut n = 0;
  let mut gamma = "".to_owned();
  let mut epsilon = "".to_owned();
  while len > n {
    let mut vote = 0;

    if let Ok(lines) = read_lines(input_path) {
      for line_result in lines {
        if let Ok(line) = line_result {
          if first_time {
            len = line.len();
            println!("{} is len {}", line, len);
            first_time = false;
          }

          let c = line.chars().nth(n).unwrap();
          if c == '0' {
            vote -= 1;
          }
          else if c == '1' {
            vote += 1;
          }
        }
      }
    }

    if vote > 0 {
      gamma += "1";
      epsilon += "0";
    }
    else {
      gamma += "0";
      epsilon += "1";
    }

    n += 1;
  }


  let mut oxy = "".to_owned();
  let mut co2 = "".to_owned();
  for is_oxy in [true, false] {
    n = 0;
    first_time = true;
    let mut binary_numbers: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines(input_path) {
      for line_result in lines {
        if let Ok(line) = line_result {
          binary_numbers.push(line);
        }
      }
    }

    while len > n {
      // consider only position "n" for this loop

      let mut vote = 0;
      for number in &binary_numbers {
        let c = number.chars().nth(n).unwrap();
        if c == '0' {
          vote -= 1;
        } else if c == '1' {
          vote += 1;
        }
      }

      println!("{}", binary_numbers.len());
      println!("{}", 1);
      let mut i: i32 = (binary_numbers.len() - 1) as i32;
      let most_common: char = if vote >= 0 { '1' } else { '0' };
      let least_common: char = if vote >= 0 { '0' } else { '1' };
      let criteria = if is_oxy { most_common } else { least_common };
      if is_oxy { oxy += &*criteria.to_string() } else { co2 += &*criteria.to_string() }
      while i >= 0 {
        if binary_numbers.get(i as usize).unwrap().chars().nth(n).unwrap() != criteria {
          binary_numbers.remove(i as usize);
        }
        i -= 1;
      }

      n += 1;

      if binary_numbers.len() == 1 {
        let remaining_str = binary_numbers.get(0).unwrap();
        for i in n..remaining_str.len() {
          let c = remaining_str.chars().nth(i).unwrap().to_string();
          if is_oxy { oxy += &c } else { co2 += &c }
        }
        break;
      }
      if binary_numbers.len() == 0 {
        eprintln!("you fucked up");
      }
    }
  }

  println!("Oxy: {}, co2: {}", oxy, co2);
  return (binary_to_i32(oxy) * binary_to_i32(co2)).to_string();
  // return (binary_to_i32(gamma) * binary_to_i32(epsilon)).to_string();
}



fn binary_to_i32(binary: String) -> i32 {
  return isize::from_str_radix(&*binary, 2).unwrap() as i32;
}
