#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|l| {
            l.parse::<i64>().unwrap()
        }).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[i64]) -> i64 {
    input
        .iter()
        .map(|x| *x/3 -2 )
        .sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[i64]) -> i64 {
    input
        .iter()
        .map(|x| {
                let mut sum : i64;
                let mut tsum : i64;
                sum = *x/3 -2;
                tsum = sum;
                while tsum > 0 {
                        tsum = tsum/3 -2;
                        if tsum > 0 {
                                sum += tsum;
                        }
                }
                sum
        })
        .sum()
}