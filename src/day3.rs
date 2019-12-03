
use std::collections::HashSet;
use std::collections::HashMap;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Wire> {

    let mut id: i64 = 0;
    
    input
    .lines()
    .map(|line| {
        id+=1;
        let mut wire_moves: Vec<WireMove> = Vec::new();
        
        for entry in line.split(","){
            let (first, last) = entry.split_at(1);
            let amount = last.parse::<i64>().unwrap();
            
            match first {
                "U" => {
                    
                    wire_moves.push(WireMove {dir: Direction::Up, amount: amount});
                },
                "D" => {
                    
                    wire_moves.push(WireMove {dir: Direction::Down, amount: amount});
                },
                "R" => {
                    
                    wire_moves.push(WireMove {dir: Direction::Right, amount: amount});
                },
                "L" => {
                    
                    wire_moves.push(WireMove {dir: Direction::Left, amount: amount});
                },
                _ => {}
            }
        };
        

        Wire {id: id, moves: wire_moves}
        
    })
    .collect()


}




#[derive(Debug)]
enum Direction {
        Up,
        Down,
        Left,
        Right
}
#[derive(Debug)]
pub struct WireMove {
        dir: Direction,
        amount: i64,
}

#[derive(Debug)]
pub struct Wire {
    id: i64,
    moves: Vec<WireMove>
}

#[derive(Debug)]
pub struct WireBoard {
    wires: Vec<Wire>,
    height: i64,
    width: i64,
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Point {
    x: i64,
    y: i64
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &Vec<Wire>) -> i64 {
    let mut memory: HashMap<Point, HashSet<i64>> = HashMap::new();

    
    for wire in input.iter() {
        let mut cur_x = 0;
        let mut cur_y = 0;
        for wire_move in wire.moves.iter() {
            match wire_move.dir {
                Direction::Up => {
                    for _i in 0..wire_move.amount {
                        cur_y += 1;
                        let p: Point = Point{x: cur_x, y: cur_y};
                        match memory.get_mut(&p) {
                            Some(set) => {
                                set.insert(wire.id);
                            },
                            None => {
                                let mut set = HashSet::new();
                                set.insert(wire.id);
                                memory.insert(p, set);
                            }
                        }
                    }
                    
                },
                Direction::Down => {
                    for _i in 0..wire_move.amount {
                        cur_y -= 1;
                        let p: Point = Point{x: cur_x, y: cur_y};
                        match memory.get_mut(&p) {
                            Some(set) => {
                                set.insert(wire.id);
                            },
                            None => {
                                let mut set = HashSet::new();
                                set.insert(wire.id);
                                memory.insert(p, set);
                            }
                        }
                    }
                },
                Direction::Right => {
                    for _i in 0..wire_move.amount {
                        cur_x += 1;
                        let p: Point = Point{x: cur_x, y: cur_y};
                        match memory.get_mut(&p) {
                            Some(set) => {
                                set.insert(wire.id);
                            },
                            None => {
                                let mut set = HashSet::new();
                                set.insert(wire.id);
                                memory.insert(p, set);
                            }
                        }
                    }
                },
                Direction::Left => {
                    for _i in 0..wire_move.amount {
                        cur_x -= 1;
                        let p: Point = Point{x: cur_x, y: cur_y};
                        match memory.get_mut(&p) {
                            Some(set) => {
                                set.insert(wire.id);
                            },
                            None => {
                                let mut set = HashSet::new();
                                set.insert(wire.id);
                                memory.insert(p, set);
                            }
                        }
                    }
                }
            }
            
        }
    }

    let mut min_dist: i64 = std::i64::MAX;
    for (k, v) in memory.iter() {
        
        if v.len() == input.len() {
            let dist = k.x.abs()+k.y.abs();
            if dist < min_dist {
                min_dist = dist;
            }
        }
    }
    min_dist
    
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &Vec<Wire>) -> i64 {
    let mut memory: HashMap<Point, HashMap<i64, i64>> = HashMap::new();

    
    for wire in input.iter() {
        let mut cur_x = 0;
        let mut cur_y = 0;
        let mut step_count = 0;
        for wire_move in wire.moves.iter() {
            match wire_move.dir {
                Direction::Up => {
                    for _i in 0..wire_move.amount {
                        cur_y += 1;
                        step_count+=1;
                        let p: Point = Point{x: cur_x, y: cur_y};
                        match memory.get_mut(&p) {
                            Some(map) => {
                                match map.get_mut(&wire.id) {
                                    Some(_entry) => {},
                                    None => {
                                        map.insert(wire.id, step_count);
                                    }
                                }
                            },
                            None => {
                                let mut map = HashMap::new();
                                map.insert(wire.id, step_count);
                                memory.insert(p, map);
                            }
                        }
                    }
                    
                },
                Direction::Down => {
                    for _i in 0..wire_move.amount {
                        cur_y -= 1;
                        step_count+=1;
                        let p: Point = Point{x: cur_x, y: cur_y};
                        match memory.get_mut(&p) {
                            Some(map) => {
                                match map.get_mut(&wire.id) {
                                    Some(_entry) => {},
                                    None => {
                                        map.insert(wire.id, step_count);
                                    }
                                }
                            },
                            None => {
                                let mut map = HashMap::new();
                                map.insert(wire.id, step_count);
                                memory.insert(p, map);
                            }
                        }
                    }
                },
                Direction::Right => {
                    for _i in 0..wire_move.amount {
                        cur_x += 1;
                        step_count+=1;
                        let p: Point = Point{x: cur_x, y: cur_y};
                        match memory.get_mut(&p) {
                            Some(map) => {
                                match map.get_mut(&wire.id) {
                                    Some(_entry) => {},
                                    None => {
                                        map.insert(wire.id, step_count);
                                    }
                                }
                            },
                            None => {
                                let mut map = HashMap::new();
                                map.insert(wire.id, step_count);
                                memory.insert(p, map);
                            }
                        }
                    }
                },
                Direction::Left => {
                    for _i in 0..wire_move.amount {
                        cur_x -= 1;
                        step_count+=1;
                        let p: Point = Point{x: cur_x, y: cur_y};
                        match memory.get_mut(&p) {
                            Some(map) => {
                                match map.get_mut(&wire.id) {
                                    Some(_entry) => {},
                                    None => {
                                        map.insert(wire.id, step_count);
                                    }
                                }
                            },
                            None => {
                                let mut map = HashMap::new();
                                map.insert(wire.id, step_count);
                                memory.insert(p, map);
                            }
                        }
                    }
                }
            }
            
        }
    }

    let mut min_signal: i64 = std::i64::MAX;
    for (_k, v) in memory.iter() {
        
        if v.len() == input.len() {
            let mut signal = 0;
            for (_k, v) in v.iter() {
                signal+=v;
            }
            
            if signal < min_signal {
                min_signal = signal;
            }
        }
    }
    min_signal
    
}