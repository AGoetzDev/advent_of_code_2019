use std::{usize};
#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect()

}

#[aoc(day8, part1)]
pub fn solve_part1(input: &Vec<u32>) -> usize {
    let mut least_zeroes = usize::MAX;
    let mut result = 0;

    for layer in input.chunks(6 * 25) {
        let zeroes = layer.iter().filter(|i| **i == 0).count();

        if zeroes < least_zeroes {
            let ones = layer.iter().filter(|i| **i == 1).count();
            let twos = layer.iter().filter(|i| **i == 2).count();

            least_zeroes = zeroes;
            result = ones * twos;
        }
    }   
    result
    
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &Vec<u32>) -> usize {
    
    let layers = input.chunks(25 * 6).rev().collect::<Vec<_>>();
    let mut image = layers[0].to_owned();

    for i in 1..(25 * 6) {
        for layer in &layers {
            if layer[i] != 2 {
                image[i] = layer[i];
            }
        }
    }

    for row in image.chunks(25) {
        println!(
            "{}",
            row.iter()
                .map(|n| if *n == 0 { " " } else { "o" })
                .collect::<Vec<_>>()
                .join("")
        );
    }
    0
}