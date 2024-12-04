pub fn solve(input: String) -> () {
    solve_part_1(&input);
    solve_part_2(&input);
}

fn solve_part_1(input: &String) {
    let mut total = 0;
    let instructions = decorrupt(&input);
    for instruction in instructions {
        if instruction.command.eq("mul") {
            let var1: i32 = instruction.params[0];
            let var2: i32 = instruction.params[1];
            let result = var1 * var2;
            total = total + result;
        }
    }
    println!("Part one ---");
    println!("Result {}", total);
}

fn solve_part_2(input: &String) {
    let mut total = 0;
    let mut do_op = true;
    let instructions = decorrupt(&input);
    for instruction in instructions {
        match instruction.command.as_str() {
            "don't" => {
                println!("Told to stop!");
                do_op = false
            }

            "do" => do_op = true,
            "mul" => {
                if do_op {
                    let var1: i32 = instruction.params[0];
                    let var2: i32 = instruction.params[1];
                    let result = var1 * var2;

                    total = total + result;
                }
            }
            &_ => (),
        }
    }
    println!("Part two ---");
    println!("Result {}", total);
}

struct Instruction {
    params: Vec<i32>,
    command: String,
}

fn decorrupt(input: &String) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    let max_length = "mul(123,456)".len();

    for (index, char) in input.chars().enumerate() {
        if char == 'm' {
            let start = index;
            let mut stop = index + max_length;

            if stop > input.len() {
                stop = input.len() - 1;
            }

            let instruction = parse_instruction(&input[start..stop]);
            match instruction {
                None => (),
                Some(value) => instructions.push(value),
            }
        }

        if char == 'd' {
            let start = index;
            let mut stop = index + "don't()".len();

            if stop > input.len() {
                stop = input.len() - 1;
            }
            let instruction = parse_conditional(&input[start..stop]);
            match instruction {
                None => (),
                Some(value) => instructions.push(value),
            }
        }
    }

    instructions
}

fn parse_instruction(input: &str) -> Option<Instruction> {
    let mut var1 = String::new();
    let mut var2 = String::new();
    let mut comma_hit = false;
    let mut end_hit = false;
    let mut expected: Vec<char> = "mul(".chars().collect();

    for char in input.chars() {
        if expected.len() > 0 {
            let expected_char = expected.remove(0);
            if char != expected_char {
                return None;
            } else {
                continue;
            }
        }
        if char == ',' {
            comma_hit = true;
            continue;
        }

        if char == ')' {
            end_hit = true;
            break;
        }
        if !comma_hit {
            if var1.len() >= 3 {
                return None;
            }
            var1.push(char);
        } else {
            if var2.len() >= 3 {
                return None;
            }
            var2.push(char);
        }
    }
    if !comma_hit || !end_hit {
        return None;
    }

    let var1_result: Result<i32, _> = var1.parse();
    let var2_result: Result<i32, _> = var2.parse();
    let var1: i32;
    match var1_result {
        Err(_) => return None,
        Ok(val) => var1 = val,
    }

    let var2: i32;
    match var2_result {
        Err(_) => return None,
        Ok(val) => var2 = val,
    }

    Some(Instruction {
        params: vec![var1, var2],
        command: "mul".to_string(),
    })
}

fn parse_conditional(input: &str) -> Option<Instruction> {
    if input.contains("do()") {
        return Some(Instruction {
            params: Vec::new(),
            command: "do".to_string(),
        });
    }
    if input.contains("don't()") {
        return Some(Instruction {
            params: Vec::new(),
            command: "don't".to_string(),
        });
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_method_parse_for_mu_3_char_intl() {
        let input = "mul(123,456)";
        let result: Option<Instruction> = parse_instruction(&input);
        assert!(result.is_some());
        let result: Instruction = result.unwrap();
        let params: Vec<i32> = result.params;
        assert_eq!(&123, params.get(0).unwrap());
        assert_eq!(&456, params.get(1).unwrap());
    }

    #[test]
    fn check_method_parse_for_mul_1_char_int() {
        let input = "mul(1,2)";
        let result: Option<Instruction> = parse_instruction(&input);
        assert!(result.is_some());
        let result: Instruction = result.unwrap();
        let params: Vec<i32> = result.params;
        assert_eq!(&1, params.get(0).unwrap());
        assert_eq!(&2, params.get(1).unwrap());
    }

    #[test]
    fn check_method_parse_for_mul_malformed() {
        let input = "mul1,2)";
        let result: Option<Instruction> = parse_instruction(&input);
        assert!(result.is_none());
    }

    #[test]
    fn check_method_parse_for_malformed() {
        let input = "mll(1,2)";
        let result: Option<Instruction> = parse_instruction(&input);
        assert!(result.is_none());
    }

    #[test]
    fn check_method_parse_no_end() {
        let input = "mul(1,2";
        let result: Option<Instruction> = parse_instruction(&input);
        assert!(result.is_none());
    }

    #[test]
    fn check_method_4char_int_should_not_parse() {
        let input = "mul(1234,5)";
        let result: Option<Instruction> = parse_instruction(&input);
        assert!(result.is_none());
    }

    #[test]
    fn check_decorrupt() {
        let input =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string();
        let result = decorrupt(&input);
        assert_eq!(4, result.len());
    }

    #[test]
    fn check_parse_dont() {
        let input = "don't()";
        let result: Option<Instruction> = parse_conditional(&input);
        assert!(result.is_some());
        assert_eq!("don't", result.unwrap().command);
    }

    #[test]
    fn check_parse_do() {
        let input = "do()";
        let result: Option<Instruction> = parse_conditional(&input);
        assert!(result.is_some());
        assert_eq!("do", result.unwrap().command);
    }

    #[test]
    fn check_parse_dont_with_noise() {
        let input = "don't()fee2";
        let result: Option<Instruction> = parse_conditional(&input);
        assert!(result.is_some());
        assert_eq!("don't", result.unwrap().command);
    }
}
