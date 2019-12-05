use std::io::{stdin,stdout,Write};

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input.split(",").map(|c| c.parse::<i64>().unwrap()).collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &Vec<i64>) -> i64 {
    let result = execute_input(input);
    
    match result {
        Ok(x) => return x,
        Err(x) => {
            println!("{:?}", x);
            return -1;
        }
    }
    
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Vec<i64>) -> i64 {
    let result = execute_input(input);
    
    match result {
        Ok(x) => return x,
        Err(x) => {
            println!("{:?}", x);
            return -1;
        }
    }
    
}



pub fn get_param(input: &Vec<i64>, mode: i64, num_param: usize, index: usize) -> i64{
    
    
    match mode {
        0 => {

            return input[input[index+num_param] as usize]
        },
        1 => {
            return input[index+num_param]
        },
        _ =>  {
            panic!("Invalid mode");
            
        }
    }
}

pub fn execute_input(base_input: &Vec<i64>) -> Result<i64, &'static str> {
            let mut input = base_input.clone();
            
            let mut index = 0;
            
            while index < input.len() {
                let instruction = input[index];
                
                let opstr =  instruction.to_string();
                let opcode: i64 ;
                if opstr.len() >= 2 {
                    opcode = opstr[opstr.len()-2..].parse::<i64>().unwrap();
                } else {
                    opcode = opstr.parse::<i64>().unwrap(); 
                }
                let mut param_modes: Vec<i64>  = Vec::new();
                if opstr.len() > 2 {
                    param_modes = opstr[..opstr.len()-2].chars().map(|d| d.to_digit(10).unwrap() as i64).collect();
                }
                param_modes.reverse();
                while param_modes.len() < 4 {
                    param_modes.push(0);
                }
                
                if opcode == 99 {
                    break;
                }
                
                let advance;
                match opcode {
                    99 => break,
                    1 => {
                        let param1 = get_param(&input, param_modes[0], 1, index);
                        
                        let param2 =  get_param(&input, param_modes[1], 2, index);
                        
                        let target = get_param(&input, 1, 3, index);
                        
                        
                        let result = param1 + param2;
                        input[target as usize] = result;
                        advance = 4;
                    },
                    2 => {
                        let param1 = get_param(&input, param_modes[0], 1, index);
                        
                        let param2 = get_param(&input, param_modes[1], 2, index);
                        
                        let target = get_param(&input, 1, 3, index);
                       
                        
                        let result = param1 * param2;
                        input[target as usize] = result;

                        advance = 4;
                    },
                    3 => {
                        let mut s=String::new();
                        print!("Please enter a number: ");
                        let _=stdout().flush();
                        stdin().read_line(&mut s).expect("Did not enter a correct string");
                        if let Some('\n')=s.chars().next_back() {
                            s.pop();
                        }
                        if let Some('\r')=s.chars().next_back() {
                            s.pop();
                        }
                        let i = s.parse::<i64>().unwrap();
                        let target = get_param(&input, 1, 1, index);
                        
                        
                        input[target as usize] = i;

                        advance = 2;

                    },
                    4 => {
                        let param1 = get_param(&input,param_modes[0], 1, index);
                        
                        
                        println!("#####Output: {:?} ######", param1);
                        advance = 2;

                    }
                    5 => {
                        let param1 = get_param(&input, param_modes[0], 1, index);
                        
                        let param2 = get_param(&input, param_modes[1], 2, index);
                        
                        if param1 != 0 {
                            index = param2 as usize;
                            advance = 0;
                        } else {
                            advance = 3;
                        }
                        
                        
                    },
                    6 => {
                        let param1 = get_param(&input, param_modes[0], 1, index);
                        
                        let param2 = get_param(&input, param_modes[1], 2, index);
                        
                        if param1 == 0 {
                            index = param2 as usize;
                            advance = 0;
                        } else {
                            advance = 3;
                        }
                        
                        
                    },
                    7 => {
                        let param1 = get_param(&input, param_modes[0], 1, index);
                        
                        let param2 = get_param(&input, param_modes[1], 2, index);
                        
                        let target = get_param(&input, 1, 3, index);
                        
                        if param1 < param2 {
                            input[target as usize] = 1;
                        } else {
                            input[target as usize] = 0;
                        }
                        advance = 4;
                        
                        
                    },
                    8 => {
                        let param1 = get_param(&input, param_modes[0], 1, index);
                        
                        let param2 = get_param(&input, param_modes[1], 2, index);
                        
                        let target = get_param(&input, 1, 3, index);
                        
                        if param1 == param2 {
                            input[target as usize] = 1;
                        } else {
                            input[target as usize] = 0;
                        }
                        advance = 4;
                        
                        
                    },
                    _ => return Err("Invalid OpCode")
                }
                
                index +=advance;
            }
            Ok(input[0])
}