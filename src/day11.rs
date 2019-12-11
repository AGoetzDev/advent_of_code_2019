use std::collections::HashMap;
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::ops::{Sub, Add};


#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input.split(",").map(|c| c.parse::<i64>().unwrap()).collect()
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &Vec<i64>) -> i64 {

    let mut initial_inputs: Vec<Vec<i64>> = Vec::new();
    let mut input_1: Vec<i64> = Vec::new();
    input_1.push(0);
    initial_inputs.push(input_1);
    let result = run_simulation(input.clone(), &initial_inputs, false);
    
    match result {
        Ok(x) => return x,
        Err(x) => {
            println!("{:?}", x);
            return -1;
        }
    } 
    -1 

}

#[aoc(day11, part2)]
pub fn solve_part2(input: &Vec<i64>) -> i64 {

    let mut initial_inputs: Vec<Vec<i64>> = Vec::new();
    let mut input_1: Vec<i64> = Vec::new();
    input_1.push(1);
    initial_inputs.push(input_1);
    let result = run_simulation(input.clone(), &initial_inputs, true);
    
    match result {
        Ok(x) => return x,
        Err(x) => {
            println!("{:?}", x);
            return -1;
        }
    } 
    -1 

}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct Point(pub i64, pub i64);

impl Sub for Point {
    type Output = Self;
    fn sub(self, r: Self) -> Self {
        Point(self.0 - r.0, self.1 - r.1)
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, r: Self) -> Self {
        Point(self.0 + r.0, self.1 + r.1)
    }
}

enum SimulationMessage {
    Input(i64),
    HaltEvent(Result<i64, &'static str>),
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
enum RobotOutputMode {
    Paint,
    Rotate,
}
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
enum RobotRotation {
    Up,
    Down,
    Right,
    Left
}


fn run_simulation(program: Vec<i64>, initial_inputs: &Vec<Vec<i64>>, print: bool) -> Result<i64, &'static str> {
    if initial_inputs.len() == 0 {
        return Err("Need At least 1 input!");
    }

    let mut initial_inputs = initial_inputs.clone();
    

    let mut thread_senders: Vec<(usize, Sender<SimulationMessage>)> = Vec::new();
    let mut thread_receivers: Vec<(usize, Receiver<SimulationMessage>)> = Vec::new();
    for i in 0..initial_inputs.len() {
        let (sender, receiver) = mpsc::channel::<SimulationMessage>();
        thread_senders.push((i, sender));
        thread_receivers.push((i, receiver));
    }
    let first_sender = mpsc::Sender::clone(&thread_senders[0].1);
    let mut sim_receiver: Option<Receiver<SimulationMessage>> = None;
    let mut removed_senders = 0;
    let mut removed_receivers = 0;

    if initial_inputs.len() > 1 {
        for (i, item) in initial_inputs.iter().enumerate() {
            let initial_input = item.clone();
            let mem_clone = program.clone();
            if i == initial_inputs.len() -1 {
                
                let sender = thread_senders.remove(0);
                removed_senders+=1;
                let receiver = thread_receivers.remove(i-removed_receivers);
                removed_receivers +=1;
    
                let (sim_tx, sim_rx) = mpsc::channel::<SimulationMessage>();
                sim_receiver = Some(sim_rx);
                thread::spawn(move || {
                    
                    let sender = sender.1;
                    let receiver = receiver.1;
                    let prog = mem_clone;
                    let mut computer = IntCodeComputer::new(i, prog, initial_input, sender, receiver);
                    let result = computer.execute_program();
                    match result {
                        Ok(r) => {
                            sim_tx.send(SimulationMessage::HaltEvent(Ok(r)));
                        },
                        Err(s) => {
                            computer.sender.send(SimulationMessage::HaltEvent(Err(s)));
                            sim_tx.send(SimulationMessage::HaltEvent(Err(s)));
                        }
                        
                    }
                });
            } else {
                let sender_id = i+1-removed_senders;
                let receiver_id = i-removed_receivers;
                
                let sender = thread_senders.remove(sender_id);
                removed_senders+=1;
                let receiver = thread_receivers.remove(receiver_id);
                removed_receivers +=1;
    
                
                thread::spawn(move || {
                    let sender = sender.1;
                    let receiver = receiver.1;
                    let prog = mem_clone;
                    let mut computer = IntCodeComputer::new(i, prog, initial_input, sender, receiver);
                    let result = computer.execute_program();
                    match result {
                        Ok(_r) => {
                            
                        },
                        Err(s) => {
                            computer.sender.send(SimulationMessage::HaltEvent(Err(s)));
                        }
                        
                    }
                });
            } 
        }
    } else {
        let initial_input = initial_inputs.remove(0);
        let mem_clone = program.clone();
        let receiver = thread_receivers.remove(0).1;
        
        let (sim_tx, sim_rx) = mpsc::channel::<SimulationMessage>();
        
        sim_receiver = Some(sim_rx);
        
        thread::spawn(move || {
                    
            let sender = sim_tx;
            let receiver = receiver;
            let prog = mem_clone;
            let mut computer = IntCodeComputer::new(1, prog, initial_input, sender, receiver);
            let result = computer.execute_program();
            match result {
                Ok(r) => {
                    computer.sender.send(SimulationMessage::HaltEvent(Ok(r)));
                },
                Err(s) => {
                    computer.sender.send(SimulationMessage::HaltEvent(Err(s))); 
                }
                        
            }
        });
    }

    
    let mut current_position = Point{0:0, 1:0};
    let mut current_rotation = RobotRotation::Up;
    let mut hull:HashMap<Point, (i64, i64)> = HashMap::new();
    let mut mode = RobotOutputMode::Paint;
    let mut painted_count = 0;
    
    match sim_receiver {
        Some(t) => {
            loop {
                let output = t.recv();
                match output {
                    Ok(s) => {
                        match s {
                            SimulationMessage::Input(t) => {
                                match mode {
                                    RobotOutputMode::Paint => {
                                        let entry = hull.entry(current_position).or_insert((0, 0));
                                        entry.0 = t;
                                        entry.1 = entry.1+1;
                                        if entry.1 == 1 {
                                            painted_count+=1;
                                        }
                                        mode = RobotOutputMode::Rotate;
                                    },
                                    RobotOutputMode::Rotate => {
                                        match t {
                                            0 => {
                                                match current_rotation {
                                                    RobotRotation::Up => {
                                                        current_position.0 -=1;
                                                        current_rotation = RobotRotation::Left;
                                                    },
                                                    RobotRotation::Down => {
                                                        current_position.0 +=1;
                                                        current_rotation = RobotRotation::Right;
                                                    },
                                                    RobotRotation::Left => {
                                                        current_position.1 -=1;
                                                        current_rotation = RobotRotation::Down;
                                                    },
                                                    RobotRotation::Right => {
                                                        current_position.1 +=1;
                                                        current_rotation = RobotRotation::Up;
                                                    }
                                                }
                                            },
                                            1 => {
                                                match current_rotation {
                                                    RobotRotation::Up => {
                                                        current_position.0 +=1;
                                                        current_rotation = RobotRotation::Right;
                                                    },
                                                    RobotRotation::Down => {
                                                        current_position.0 -=1;
                                                        current_rotation = RobotRotation::Left;
                                                    },
                                                    RobotRotation::Left => {
                                                        current_position.1 +=1;
                                                        current_rotation = RobotRotation::Up;
                                                    },
                                                    RobotRotation::Right => {
                                                        current_position.1 -=1;
                                                        current_rotation = RobotRotation::Down;
                                                    }
                                                }
                                            },
                                            _ => panic!("Invalid Rotation!")
                                        }
                                        mode = RobotOutputMode::Paint;
                                        let entry = hull.entry(current_position).or_insert((0, 0));
                                        first_sender.send(SimulationMessage::Input(entry.0));
                                    }
                                }
                                //println!("Received: {:?}", t);
                            },
                            SimulationMessage::HaltEvent(e) => {
                                match e {
                                    Ok(_r) => {
                                        
                                        if print {
                                            let max_x = hull.iter().max_by_key(|(k, _v)| k.0).unwrap().0;
                                            let min_x = hull.iter().min_by_key(|(k, _v)| k.0).unwrap().0;
                                            let max_y = hull.iter().max_by_key(|(k, _v)| k.1).unwrap().0;
                                            let min_y = hull.iter().min_by_key(|(k, _v)| k.1).unwrap().0;
                                            for j in (min_y.1..=max_y.1).rev(){
                                                let mut line = "".to_string();
                                                for i in min_x.0..=max_x.0{
                                                    let point = Point{0: i, 1: j};
                                                    let entry = hull.get(&point);
                                                    match entry {
                                                        Some(t) => {
                                                            match t.0 {
                                                                0 => {
                                                                    line.push_str(".");
                                                                },
                                                                1 => {
                                                                    line.push_str("#");
                                                                }
                                                                _ => {}
                                                            }
                                                        },
                                                        None => {
                                                            line.push_str(".")
                                                        }
                                                    }
                                                    
                                                }
                                                println!("{:?}", line);
                                            }
                                        }
                                        
                                        return Ok(painted_count)
                                    }
                                    Err(s) => {
                                        return Err(s);
                                    }
                                }
                            }
                        }
                    },
                    Err(_) => {
                        return Err("Error during receive ")
                    }, 
                }
            }
            
        }, 
        None => {
            return Err("Sim Channel was never initialized!")
        }
    }
    
    
}

struct IntCodeComputer {
    id: usize,
    memory: HashMap<usize, i64>,
    ip: usize,
    base: usize,
    inputs: Vec<i64>,
    sender: Sender<SimulationMessage>,
    receiver: Receiver<SimulationMessage>
}

impl IntCodeComputer {
    fn new(id: usize, memory: Vec<i64>, inputs: Vec<i64>, sender: Sender<SimulationMessage>, receiver: Receiver<SimulationMessage>) -> IntCodeComputer {
        let mut res = IntCodeComputer { id: id, memory: HashMap::new(), ip: 0, base: 0, inputs: inputs, sender: sender, receiver: receiver };
        for i in 0..memory.len() {
            res.memory.insert(i, memory[i]);
        }

        res
    }

    fn get_param(&mut self, mode: i64, num_param: usize, index: usize, base: usize) -> i64{
        let val = *self.memory.entry(index+num_param).or_insert(0);
        match mode {
            0 => {
                return *self.memory.entry(val as usize).or_insert(0)
            },
            1 => {
                return val
            },
            2 => {
                return *self.memory.entry(val as usize + base).or_insert(0)
            },
            _ =>  {
                panic!("Invalid mode");
                
            }
        }
    }
    
    fn set_param(&mut self, mode: i64, val: i64, pos: usize, base: usize){
        match mode {
            0 => {
                let res_pos = *self.memory.entry(pos).or_insert(0) as usize;
                self.memory.insert(res_pos, val);
            },
            2 => {
                let res_pos = base + *self.memory.entry(pos).or_insert(0) as usize;
                self.memory.insert(res_pos, val);
            },
            _ => panic!("Invalid mode"),
        }
    }

    fn execute_program(&mut self) -> Result<i64, &'static str> {

        let mut last_output = 0;

        loop {
            let instruction = self.memory.get(&self.ip).unwrap();
            let opcode = instruction % 100;
            let opstr =  instruction.to_string();
           
            let mut param_modes: Vec<i64>  = Vec::new();
            if opstr.len() > 2 {
                param_modes = opstr[..opstr.len()-2].chars().map(|d| d.to_digit(10).unwrap() as i64).collect();
            }
            param_modes.reverse();
            while param_modes.len() < 4 {
                param_modes.push(0);
            }
            
            let advance;
            match opcode {
                
                1 => {
                    let param1 = self.get_param(param_modes[0], 1, self.ip, self.base);
                    let param2 =  self.get_param(param_modes[1], 2, self.ip, self.base);

                    self.set_param(param_modes[2], param1 + param2, self.ip+3, self.base);
                    advance = 4;
                },
                2 => {
                    let param1 = self.get_param(param_modes[0], 1, self.ip, self.base);
                    let param2 = self.get_param(param_modes[1], 2, self.ip, self.base);
                    self.set_param(param_modes[2], param1 * param2, self.ip+3, self.base);

                    advance = 4;
                },
                3 => {
                    
                    match self.inputs.len() {
                        0 => {
                            let rec = self.receiver.recv();
                            
                            match rec {
                                Ok(s) => {
                                    match s {
                                        SimulationMessage::Input(input) => {
                                            self.set_param(param_modes[0], input, self.ip+1, self.base);
                                        },
                                        SimulationMessage::HaltEvent(s) => {
                                            return Err("Halted By Previous");
                                        },
                                    }
                                },
                                Err(_) => {
                                    println!("Error receiving: {:?}", self.id);
                                    return Err("Error during receive ")
                                },
                            } 
                        }, 
                        _ => {
                            let input = self.inputs.remove(0);
                            self.set_param(param_modes[0], input, self.ip+1, self.base);
                        }
                    }                        

                    advance = 2;

                },
                4 => {
                    let param1 = self.get_param(param_modes[0], 1, self.ip, self.base);
                    last_output = param1;
                    self.sender.send(SimulationMessage::Input(last_output));
                    advance = 2;

                }
                5 => {
                    let param1 = self.get_param(param_modes[0], 1, self.ip, self.base);
                    let param2 = self.get_param(param_modes[1], 2, self.ip, self.base);
                    
                    if param1 != 0 {
                        self.ip = param2 as usize;
                        advance = 0;
                    } else {
                        advance = 3;
                    }
                    
                    
                },
                6 => {
                    let param1 = self.get_param(param_modes[0], 1, self.ip, self.base);
                    let param2 = self.get_param(param_modes[1], 2, self.ip, self.base);
                    
                    if param1 == 0 {
                        self.ip = param2 as usize;
                        advance = 0;
                    } else {
                        advance = 3;
                    }
                    
                    
                },
                7 => {
                    let param1 = self.get_param(param_modes[0], 1, self.ip, self.base);
                    let param2 = self.get_param(param_modes[1], 2, self.ip, self.base);
                    
                    
                    if param1 < param2 {
                        self.set_param(param_modes[2], 1, self.ip+3, self.base);
                    } else {
                        self.set_param(param_modes[2], 0, self.ip+3, self.base);
                    }
                    advance = 4;
                    
                    
                },
                8 => {
                    let param1 = self.get_param(param_modes[0], 1, self.ip, self.base);
                    let param2 = self.get_param(param_modes[1], 2, self.ip, self.base);
                    
                    
                    if param1 == param2 {
                        self.set_param(param_modes[2], 1, self.ip+3, self.base);
                    } else {
                        self.set_param(param_modes[2], 0, self.ip+3, self.base);
                    }
                    advance = 4;
                    
                    
                },
                9 => { 
                    let param1 = self.get_param(param_modes[0], 1, self.ip, self.base);
                    self.base = self.base + param1 as usize;
                    advance = 2;
                }
                99 => break,
                _ => return Err("Invalid OpCode")
            }
            
            self.ip +=advance;
        }
        Ok(last_output)
    }
}