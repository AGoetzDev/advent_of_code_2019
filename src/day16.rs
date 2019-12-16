static BASE_PATTERN: &'static [i64] = &[0, 1, 0, -1];

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input.to_string().chars().map(|d| d.to_digit(10).unwrap() as i64).collect()
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &Vec<i64>) -> String {
    let mut input = input.clone();
    for i in 0..100 {
        for j in 0..input.len(){
            let o = calc_output(&input, j, 0);
            input[j] = o;
        }
    }

    let mut result_str = "".to_string();
    for i in 0..8 {
        result_str.push_str(&input[i].to_string());
    }

    result_str

}

#[aoc(day16, part2)]
pub fn solve_part2(input: &Vec<i64>) -> String {
    let mut offset_str = "".to_string();
    for i in 0..7 {
        offset_str.push_str(&input[i].to_string());
    }
    let offset = offset_str.parse::<i64>().unwrap();
    
   

    let mut input_10k = input.clone();
    for i in 1..10000{
        let mut add = input.clone();
        input_10k.append(&mut add);
    }
    
    let mut temp = Vec::new();
    for i in 0..input_10k.len(){
        temp.push(0);
    }
    for i in 0..100 {
        let mut acc = 0;
        for j in (offset as usize..input_10k.len()).rev(){
            acc = (acc+input_10k[j]).abs() %10;
            temp[j] = acc;
        }
        let temp_2 = temp;
        temp = input_10k;
        input_10k = temp_2;
        
    }
    
    let mut result_str = "".to_string();
    for i in offset as usize..offset as usize +8 {
        result_str.push_str(&input_10k[i].to_string());
    }

    result_str
    

}

fn calc_output(input: &Vec<i64>, out_pos: usize, start:usize) -> i64{
    let mut out = 0;
    for i in start..input.len(){
        let x = input[i];
        out += x*BASE_PATTERN[((i+1)/(out_pos+1))%4];
    }
    out.abs()%10
}

