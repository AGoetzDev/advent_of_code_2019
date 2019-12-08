
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use itertools::Itertools;

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input.split(",").map(|c| c.parse::<i64>().unwrap()).collect()
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &Vec<i64>) -> i64 {
    (0..=4)
    .permutations(5)
    .map(|phase_modes| {
        let result = run_simulation(input.clone(), phase_modes, true);
    
        match result {
                Ok(x) => return x,
                Err(x) => {
                    println!("{:?}", x);
                    return -1;
                }
            } 
            -1 
    })
    .max()
    .unwrap()
    
    
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &Vec<i64>) -> i64 {
    (5..=9)
    .permutations(5)
    .map(|phase_modes| {
        let result = run_simulation(input.clone(), phase_modes, true);
    
        match result {
                Ok(x) => return x,
                Err(x) => {
                    println!("{:?}", x);
                    return -1;
                }
            } 
            -1 
    })
    .max()
    .unwrap()
    
    
}





fn get_param(input: &Vec<i64>, mode: i64, num_param: usize, index: usize) -> i64{
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

enum SimulationMessage {
    Input(i64),
    HaltEvent(String),
}


fn run_simulation(program: Vec<i64>, phase_modes: Vec<i64>, force_start: bool) -> Result<i64, &'static str> {
    if phase_modes.len() == 0 {
        return Err("Need At least 1 Phase Mode!");
    }


    let mut thread_senders: Vec<(usize, Sender<SimulationMessage>)> = Vec::new();
    let mut thread_receivers: Vec<(usize, Receiver<SimulationMessage>)> = Vec::new();
    for i in 0..phase_modes.len() {
        let (sender, receiver) = mpsc::channel::<SimulationMessage>();
        thread_senders.push((i, sender));
        thread_receivers.push((i, receiver));
    }

    let first_sender = mpsc::Sender::clone(&thread_senders[0].1);
    let mut sim_receiver: Option<Receiver<SimulationMessage>> = None;
    let mut removed_senders = 0;
    let mut removed_receivers = 0;

    for (i, item) in phase_modes.iter().enumerate() {
        let phase_mode = *item;
        let prog_clone = program.clone();
        if i == phase_modes.len() -1 {
            
            let sender = thread_senders.remove(0);
            removed_senders+=1;
            let receiver = thread_receivers.remove(i-removed_receivers);
            removed_receivers +=1;

            let (sim_tx, sim_rx) = mpsc::channel::<SimulationMessage>();
            sim_receiver = Some(sim_rx);
            //println!("Amp {:?} started with sender {:?} and receiver {:?}", i, sender.0, receiver.0);
            thread::spawn(move || {
                
                let sender = sender.1;
                let receiver = receiver.1;
                let prog = prog_clone;
                let result = execute_program(prog, phase_mode, &sender, &receiver);
                match result {
                    Ok(r) => {
                        //println!("Result: {:?} {:?}", i, r);
                        sim_tx.send(SimulationMessage::Input(r));
                    },
                    Err(_s) => {
                        //println!("Halted: {:?}", i);
                        sender.send(SimulationMessage::HaltEvent("Halt".to_string()));
                        sim_tx.send(SimulationMessage::HaltEvent("Halt".to_string()));
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

            //println!("Amp {:?} started with sender {:?} and receiver {:?}", i, sender.0, receiver.0);
            thread::spawn(move || {
                let sender = sender.1;
                let receiver = receiver.1;
                let prog = prog_clone;
                let result = execute_program(prog, phase_mode, &sender, &receiver);
                match result {
                    Ok(r) => {
                        //println!("Result: {:?} {:?}", i, r);
                    },
                    Err(_s) => {
                        //println!("Halted: {:?}", i);
                        sender.send(SimulationMessage::HaltEvent("Halt".to_string()));
                    }
                    
                }
            });
        } 
    }
    
    if force_start {
        first_sender.send(SimulationMessage::Input(0));
    }
    
    match sim_receiver {
        Some(t) => {
            let final_output = t.recv();
            match final_output {
                Ok(s) => {
                    match s {
                        SimulationMessage::Input(t) => {
                            return Ok(t);
                        },
                        SimulationMessage::HaltEvent(_s) => {
                            return Err("Halted By Previous");
                        }
                    }
                },
                Err(_) => {
                    return Err("Error during receive ")
                }, 
            }
        }, 
        None => {
            return Err("Sim Channel was never initialized!")
        }
    }
    
    
}

fn execute_program(memory: Vec<i64>, phase_mode: i64, sender: &Sender<SimulationMessage>, receiver: &Receiver<SimulationMessage>) -> Result<i64, &'static str> {
            let mut input = memory;
            let mut index = 0;
            let mut last_output = 0;
            let mut phase_mode_set = false;

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
                        let target = get_param(&input, 1, 1, index);
                        match phase_mode_set {
                            true => {
                                let rec = receiver.recv();
                                
                                match rec {
                                    Ok(s) => {
                                        match s {
                                            SimulationMessage::Input(t) => {
                                                input[target as usize] = t;
                                            },
                                            SimulationMessage::HaltEvent(s) => {
                                                return Err("Halted By Previous");
                                            }
                                        }
                                    },
                                    Err(_) => {
                                        println!("Error receiving: {:?}", phase_mode);
                                        return Err("Error during receive ")
                                    },
                                } 
                            }, 
                            false => {
                                input[target as usize] = phase_mode;
                                phase_mode_set = true;
                            }
                        }                        

                        advance = 2;

                    },
                    4 => {
                        let param1 = get_param(&input,param_modes[0], 1, index);
                        
                        last_output = param1;
                        sender.send(SimulationMessage::Input(last_output));
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
            Ok(last_output)
}