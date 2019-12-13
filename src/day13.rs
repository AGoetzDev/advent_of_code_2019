use std::collections::HashMap;
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::ops::{Sub, Add};
use std::cmp::Ordering;
use std::io::{stdin,stdout,Write};


#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input.split(",").map(|c| c.parse::<i64>().unwrap()).collect()
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &Vec<i64>) -> i64 {

    let mut initial_inputs: Vec<Vec<i64>> = Vec::new();
    let mut input_1: Vec<i64> = Vec::new();
    initial_inputs.push(input_1);
    let result = run_simulation(input.clone(), &initial_inputs, false, false);
    
    match result {
        Ok(x) => return x.0,
        Err(x) => {
            println!("{:?}", x);
            return -1;
        }
    } 
    -1 

}

#[aoc(day13, part2)]
pub fn solve_part2(input: &Vec<i64>) -> i64 {
    let mut input = input.clone();
    input[0] = 2;
    let mut initial_inputs: Vec<Vec<i64>> = Vec::new();
    let mut input_1: Vec<i64> = Vec::new();
    initial_inputs.push(input_1);
    let result = run_simulation(input, &initial_inputs, false, false);
    
    match result {
        Ok(x) => return x.1,
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
    InputRequest,
    HaltEvent(Result<i64, &'static str>),
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
enum OutputMode {
    X,
    Y,
    Id
}



fn print_grid(grid: &HashMap<Point, i64>){
    let max_x = grid.iter().max_by_key(|(k, _v)| k.0).unwrap().0;
    let min_x = grid.iter().min_by_key(|(k, _v)| k.0).unwrap().0;
    let max_y = grid.iter().max_by_key(|(k, _v)| k.1).unwrap().0;
    let min_y = grid.iter().min_by_key(|(k, _v)| k.1).unwrap().0;
    
    for j in min_y.1..=max_y.1{
        let mut line = "".to_string();
        for i in min_x.0..=max_x.0{
            let point = Point{0: i, 1: j};
            let entry = grid.get(&point);
            match entry {
                    Some(t) => {
                        line.push_str(&t.to_string())
                    },
                    None => {
                        line.push_str(" ")
                    }
            }
                                                        
        }
        println!("{:?}", line);
    }
}

fn get_input_from_console() -> i64 {
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
    s.parse::<i64>().unwrap()
}

fn run_simulation(program: Vec<i64>, initial_inputs: &Vec<Vec<i64>>, print: bool, manual: bool) -> Result<(i64, i64), &'static str> {
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
                    let mut computer = IntCodeComputer::new(i, prog, initial_input, sender, receiver, false);
                    let result = computer.execute_program();
                    match result {
                        Ok(r) => {
                            sim_tx.send(SimulationMessage::HaltEvent(Ok(r))).unwrap_or_else(|_e| println!("Failed to send Successful Halt in {:?}", i));
                        },
                        Err(s) => {
                            computer.sender.send(SimulationMessage::HaltEvent(Err(s))).unwrap_or_else(|_e| println!("Failed to send Err Halt1 in {:?}", i));
                            sim_tx.send(SimulationMessage::HaltEvent(Err(s))).unwrap_or_else(|_e| println!("Failed to send Err Halt2 in {:?}", i));
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
                    let mut computer = IntCodeComputer::new(i, prog, initial_input, sender, receiver, false);
                    let result = computer.execute_program();
                    match result {
                        Ok(_r) => {
                            
                        },
                        Err(s) => {
                            computer.sender.send(SimulationMessage::HaltEvent(Err(s))).unwrap_or_else(|_e| println!("Failed to send Err Halt in {:?}", i));
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
            let mut computer = IntCodeComputer::new(1, prog, initial_input, sender, receiver, true);
            let result = computer.execute_program();
            match result {
                Ok(r) => {
                    computer.sender.send(SimulationMessage::HaltEvent(Ok(r))).unwrap_or_else(|_e| println!("Failed to send Successful Halt in {:?}", 0));
                },
                Err(s) => {
                    computer.sender.send(SimulationMessage::HaltEvent(Err(s))).unwrap_or_else(|_e| println!("Failed to send Err Halt in {:?}", 0)); 
                }
                        
            }
        });
    }

    
    
    let mut grid:HashMap<Point, i64> = HashMap::new();
    let mut mode = OutputMode::X;

    let mut cur_x = 0;
    let mut cur_y = 0;
    let mut cur_score = 0;
    let mut ball_x = 0;
    let mut paddle_x = 0;
    match sim_receiver {
        Some(t) => {
            loop {
                let output = t.recv();
                match output {
                    Ok(s) => {
                        match s {
                            SimulationMessage::Input(t) => {
                                match mode {
                                    OutputMode::X => {
                                        cur_x = t;
                                        
                                        mode = OutputMode::Y;
                                    },
                                    OutputMode::Y => {
                                        cur_y = t;
                                        
                                        mode = OutputMode::Id;
                                    },
                                    OutputMode::Id => {
                                        if cur_x == -1 && cur_y == 0 {
                                            cur_score = t;
                                        } else {
                                            if t == 4 {
                                                ball_x = cur_x;
                                            }
                                            if t == 3 {
                                                paddle_x = cur_x;
                                            }
                                            let entry = grid.entry(Point{0:cur_x, 1:cur_y}).or_insert(t);
                                            *entry = t;
                                        }
                                        
                                        mode = OutputMode::X;
                                    }
                                }
                            },
                            SimulationMessage::HaltEvent(e) => {
                                match e {
                                    Ok(_r) => {
                                        let mut count = 0;
                                        for (_key, value) in grid {
                                            if value == 2 {
                                                count+=1;
                                            }
                                        }
                                        return Ok((count, cur_score))
                                    }
                                    Err(s) => {
                                        return Err(s);
                                    }
                                }
                            },
                            SimulationMessage::InputRequest => {
                                //draw board
                                if print {
                                    print_grid(&grid);
                                }
                                
                                if manual {
                                    let i = get_input_from_console();
                                    first_sender.send(SimulationMessage::Input(i)).unwrap_or_else(|_e| println!("Failed to send input to first sender"));
                                } else {
                                    match ball_x.cmp(&paddle_x) {
                                        Ordering::Less => {
                                            first_sender.send(SimulationMessage::Input(-1)).unwrap_or_else(|_e| println!("Failed to send input to first sender"));
                                        },
                                        Ordering::Equal => {
                                            first_sender.send(SimulationMessage::Input(0)).unwrap_or_else(|_e| println!("Failed to send input to first sender"));
                                        },
                                        Ordering::Greater => {
                                            first_sender.send(SimulationMessage::Input(1)).unwrap_or_else(|_e| println!("Failed to send input to first sender"));
                                        }
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
    receiver: Receiver<SimulationMessage>,
    request_input: bool,
}

impl IntCodeComputer {
    fn new(id: usize, memory: Vec<i64>, inputs: Vec<i64>, sender: Sender<SimulationMessage>, receiver: Receiver<SimulationMessage>, request_input: bool) -> IntCodeComputer {
        let mut res = IntCodeComputer { id: id, memory: HashMap::new(), ip: 0, base: 0, inputs: inputs, sender: sender, receiver: receiver, request_input: request_input };
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
                            if self.request_input {
                                self.sender.send(SimulationMessage::InputRequest).unwrap_or_else(|_e| println!("Request for input failed  in{:?}", self.id)); 
                            }
                            let rec = self.receiver.recv();
                            
                            match rec {
                                Ok(s) => {
                                    match s {
                                        SimulationMessage::Input(input) => {
                                            self.set_param(param_modes[0], input, self.ip+1, self.base);
                                        },
                                        SimulationMessage::HaltEvent(_s) => {
                                            return Err("Halted By Previous");
                                        },
                                        SimulationMessage::InputRequest => {
                                            return Err("Invalid Input Request");
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
                    self.sender.send(SimulationMessage::Input(last_output)).unwrap_or_else(|_e| println!("Failed to send output in {:?}", self.id));
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