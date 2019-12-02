#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input.split(",").map(|c| c.parse::<i64>().unwrap()).collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Vec<i64>) -> i64 {
    let result = execute_input(input, 12, 2);
    
    match result {
        Ok(x) => return x,
        Err(x) => {
            println!("{:?}", x);
            return -1;
        }
    }
    
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &Vec<i64>) -> i64 {
    
    for noun in 0..99 {
        for verb in 0..99 {
            let result = execute_input(input, noun, verb);
            match result {
                Ok(x) => match x {
                    19690720 => return 100 * noun + verb,
                    _ => {}
                },
                Err(_x) => {}    
            }
        }
    }
    
    println!("{:?}", "No solution could be found!");
    -1
}

pub fn execute_input(base_input: &Vec<i64>, noun: i64, verb: i64) -> Result<i64, &'static str> {
            let mut input = base_input.clone();
            let len = base_input.len();
            let mut index = 0;
            input[1] = noun;
            input[2] = verb;
            
            while index < input.len() {
                let opcode = input[index];
                
                if opcode == 99 {
                    break;
                }

                let param1_p = input[index+1];
                let param2_p = input[index+2];
                let target_p = input[index+3];

                if param1_p as usize >= len || param2_p as usize >= len || target_p as usize >= len {
                    return Err("Invalid parameter pointers!");
                }

                let operand_1 = input[param1_p as usize];
                let operand_2 = input[param2_p as usize];
                

                match opcode {
                    99 => break,
                    1 => {
                        let result = operand_1 + operand_2;
                        input[target_p as usize] = result;
                    },
                    2 => {
                        let result = operand_1 * operand_2;
                        input[target_p as usize] = result;
                    },
                    _ => return Err("Invalid OpCode")
                }
                
                index +=4;
            }
            Ok(input[0])
}

