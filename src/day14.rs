use std::collections::HashMap;





#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<Reaction> {

    input
    .lines()
    .map(|line| {
        let mut parts = line.split(" => ");
        let inputs = parts.next().unwrap()
            .split(", ")
            .map(|s| parse_chemical(s))
            .collect::<HashMap<String, i64>>();
        let output = parse_chemical(parts.next().unwrap());
        Reaction::new(inputs, output)
    })
    .collect()
    

}

#[aoc(day14, part1)]
pub fn solve_part1(reactions: &Vec<Reaction>) -> i64{
    
    let fuel_cost = ore_per_fuel(reactions, 1);

    fuel_cost
}

#[aoc(day14, part2)]
pub fn solve_part2(reactions: &Vec<Reaction>) -> i64{

    let fuel_created = fuel_per_ore(reactions, 1_000_000_000_000);

    fuel_created
}


#[derive(Clone,Debug,PartialEq)]
pub struct Reaction {
    inputs: HashMap<String, i64>,
    output: (String, i64)
}

impl Reaction {
    pub fn new(inputs: HashMap<String, i64>, output: (String, i64)) -> Self {
        Self {
            inputs,
            output
        }
    }
}

fn parse_chemical(s: &str) -> (String, i64) {
    let mut parts = s.split(' ');
    let reactions_needed = parts.next().unwrap().parse::<i64>().unwrap();
    let chemical = parts.next().unwrap().to_string();
    (chemical, reactions_needed)
}



fn ore_per_fuel(reactions: &[Reaction], fuel_needed: i64) -> i64 {
    let mut needed: HashMap<String, i64>  = HashMap::new();
    needed.insert("FUEL".to_string(), fuel_needed);
    while !needed.is_empty() {
        let next = needed.iter().filter(|(chemical, needed)| &chemical[..] != "ORE" && **needed > 0).next();
        if next.is_none() {
            break;
        }
        let (chemical, amount_needed) = next.unwrap();
        let (chemical, amount_needed) = (chemical.clone(), *amount_needed);

        let reaction = reactions.iter().find(|reaction| reaction.output.0 == *chemical).unwrap();
        let produced_per_reaction = reaction.output.1;
        let reactions_needed = (amount_needed as f64 / produced_per_reaction as f64).ceil() as i64;
        let extra = reactions_needed * produced_per_reaction - amount_needed;
        assert!(extra >= 0);

        needed.insert(chemical.to_string(), -extra);

        for (input, needed_per_reaction) in &reaction.inputs {
            needed.insert(input.clone(), needed.get(input).map_or(0, |val| *val) + needed_per_reaction * reactions_needed);
        }
    }
    needed["ORE"]
}


fn fuel_per_ore(reactions: &[Reaction], target: i64) -> i64 {
    let (mut min, mut max) = (1, 2);
    while ore_per_fuel(reactions, max) < target {
        min = max;
        max *= 2;
    }
    while max - min >= 2 {
        let half = min + (max - min) / 2;
        if ore_per_fuel(reactions, half) < target {
            min = half;
        }
        else {
            max = half;
        }
    }
    min
}