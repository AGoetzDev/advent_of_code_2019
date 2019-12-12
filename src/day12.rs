use num::integer::lcm;
use regex::Regex;
use std::cmp::Ordering;
use std::str::FromStr;




pub enum Axis{
    X,
    Y,
    Z
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct CelestialBody {
    x: i64,
    y: i64, 
    z: i64,
    v_x: i64,
    v_y: i64,
    v_z: i64
}

impl FromStr for CelestialBody {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^<x=(-?\d+), y=(-?\d+), z=(-?\d+)>$").unwrap();
        if let Some(m) = re.captures(s) {
            Ok(CelestialBody {
                x:  m[1].parse().unwrap(),
                y:  m[2].parse().unwrap(),
                z:  m[3].parse().unwrap(),
                v_x: 0,
                v_y: 0,
                v_z: 0,
            })
        } else {
            Err("Error matching Regex".to_string())
        }
    }
}


impl CelestialBody {
    

    pub fn advance_step(&mut self) {
        self.x+=self.v_x;
        self.y+=self.v_y;
        self.z+=self.v_z;
    }

    pub fn advance_step_axis(&mut self, axis: Axis) {
        match axis {
            Axis::X => {
                self.x+=self.v_x;
            },
            Axis::Y => {
                self.y+=self.v_y;
            },
            Axis::Z => {
                self.z+=self.v_z;
            }
        }   
    }

    fn get_energy(&self) -> i64 {
        (self.x.abs()+self.y.abs()+self.z.abs()) * (self.v_x.abs()+self.v_y.abs()+self.v_z.abs())
    }
}

fn apply_gravity(bodies: &mut [CelestialBody], axis: Axis) {
    for i in 0..bodies.len() - 1 {
        for j in i + 1..bodies.len() {
            match axis {
                Axis::X => {
                    match bodies[i].x.cmp(&bodies[j].x) {
                        Ordering::Less => {
                            bodies.get_mut(i).unwrap().v_x += 1;
                            bodies.get_mut(j).unwrap().v_x -= 1;
                        }
                        Ordering::Equal => {}
                        Ordering::Greater => {
                            bodies.get_mut(i).unwrap().v_x -= 1;
                            bodies.get_mut(j).unwrap().v_x += 1;
                        }
                    }
                },
                Axis::Y => {
                    match bodies[i].y.cmp(&bodies[j].y) {
                        Ordering::Less => {
                            bodies.get_mut(i).unwrap().v_y += 1;
                            bodies.get_mut(j).unwrap().v_y -= 1;
                        }
                        Ordering::Equal => {}
                        Ordering::Greater => {
                            bodies.get_mut(i).unwrap().v_y -= 1;
                            bodies.get_mut(j).unwrap().v_y += 1;
                        }
                    }
                },
                Axis::Z => {
                    match bodies[i].z.cmp(&bodies[j].z) {
                        Ordering::Less => {
                            bodies.get_mut(i).unwrap().v_z += 1;
                            bodies.get_mut(j).unwrap().v_z -= 1;
                        }
                        Ordering::Equal => {}
                        Ordering::Greater => {
                            bodies.get_mut(i).unwrap().v_z -= 1;
                            bodies.get_mut(j).unwrap().v_z += 1;
                        }
                    }
                }
            }   
            
        }
    }
}


#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<CelestialBody> {
    
    input
        .lines()
        .map(|l| {
            l.parse::<CelestialBody>().unwrap() 
        }).collect()
}


#[aoc(day12, part1)]
pub fn solve_part1(input: &Vec<CelestialBody>) -> i64 {
    
    let mut original = input.clone();

    for _i in 0..1000{

        apply_gravity(&mut original, Axis::X);
        apply_gravity(&mut original, Axis::Y);
        apply_gravity(&mut original, Axis::Z);
        
        original.iter_mut().for_each(CelestialBody::advance_step); 

    }
    
    original.iter().map(CelestialBody::get_energy).sum()
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &Vec<CelestialBody>) -> i64 {
    
    let original = input.clone();
    let mut working_copy = input.clone();

    let x_cycle = (1..).find(|_| {
                        apply_gravity(&mut working_copy, Axis::X);
                        working_copy.iter_mut().for_each(|c| c.advance_step_axis(Axis::X));
                        return working_copy == original;
                    })
                    .unwrap();
    
    let y_cycle = (1..).find(|_| {
                        apply_gravity(&mut working_copy, Axis::Y);
                        working_copy.iter_mut().for_each(|c| c.advance_step_axis(Axis::Y));
                        return working_copy == original;
                    })
                    .unwrap();

    let z_cycle = (1..).find(|_| {
                        apply_gravity(&mut working_copy, Axis::Z);
                        working_copy.iter_mut().for_each(|c| c.advance_step_axis(Axis::Z));
                        return working_copy == original;
                    })
                    .unwrap();

    lcm(x_cycle,lcm(y_cycle, z_cycle))

}