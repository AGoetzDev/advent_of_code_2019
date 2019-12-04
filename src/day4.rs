#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> (i64, i64) {
    let split: Vec<String> = input.split("-").map(|s| s.to_string()).collect();
    (split[0].parse::<i64>().unwrap(), split[1].parse::<i64>().unwrap())
    
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &(i64, i64)) -> i64 {
    let mut count = 0;
    for i in input.0..input.1+1{
        let digits: Vec<i64> = i.to_string().chars().map(|d| d.to_digit(10).unwrap() as i64).collect();
        let mut stack: Vec<i64> = Vec::new();
        let mut seen_double = false;
        let mut correct = true;
        for j in digits {
            if stack.len() > 0{
                if j != stack[stack.len()-1] {
                    if j < stack[stack.len()-1]  {
                        correct = false;
                        break;
                    }
                    if stack.len() >= 2 {
                        seen_double = true
                    }
                    stack.clear();
                }
            }
            stack.push(j)
        }
        if stack.len() >= 2 {
            seen_double = true
        }
        if correct && seen_double {
            count+=1;
        }

    }
    count
    
    
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &(i64, i64)) -> i64 {
    let mut count = 0;
    for i in input.0..input.1+1{
        let digits: Vec<i64> = i.to_string().chars().map(|d| d.to_digit(10).unwrap() as i64).collect();
        let mut stack: Vec<i64> = Vec::new();
        let mut seen_double = false;
        let mut correct = true;
        for j in digits {
            if stack.len() > 0{
                if j != stack[stack.len()-1] {
                    if j < stack[stack.len()-1]  {
                        correct = false;
                        break;
                    }
                    if stack.len() == 2 {
                        seen_double = true
                    }
                    stack.clear();
                }
            }
            stack.push(j)
        }
        if stack.len() == 2 {
            seen_double = true
        }
        if correct && seen_double {
            count+=1;
        }

    }
    count
    
    
}

