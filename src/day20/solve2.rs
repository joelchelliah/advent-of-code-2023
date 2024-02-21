use core::panic;
use std::io::{BufRead, BufReader};
use std::fs::File;

#[derive(Clone)]
struct FlipFlopModule {
    name: String,
    is_on: bool,
    output_modules: Vec<String>,
}

#[derive(Clone)]
struct ConjunctionModule {
    name: String,
    inputs: Vec<(String, Pulse)>,
    output_modules: Vec<String>,
}

impl ConjunctionModule {
    fn grow_input(&mut self, name: String) {
        self.inputs.push((name, Low));
    }

    fn update_input(&mut self, from_name: String, pulse: Pulse) {
        let i = self.inputs.iter().position(|(name, _)| name == &from_name).unwrap();

        self.inputs[i] = (from_name, pulse);
    }
}

#[derive(Clone, PartialEq)]
enum Pulse {
    High,
    Low,
}

use Pulse::*;

#[derive(Clone)]
enum Module {
    FlipFlop(FlipFlopModule),
    Conjunction(ConjunctionModule),
    BroadCaster(Vec<String>),
}

impl Module {
    fn get_name(&self) -> String {
        match self {
            FlipFlop(flip_flop_module) => flip_flop_module.name.clone(),
            Conjunction(conjunction_module) => conjunction_module.name.clone(),
            BroadCaster(_) => "broadcaster".to_string(),
        }
    }

    fn get_output_modules(&self) -> Vec<String> {
        match self {
            FlipFlop(flip_flop_module) => flip_flop_module.output_modules.clone(),
            Conjunction(conjunction_module) => conjunction_module.output_modules.clone(),
            BroadCaster(output_modules) => output_modules.clone(),
        }
    }
}

use Module::*;

fn find_broadcaster(modules: &Vec<Module>) -> &Module {
    modules.iter().find(|module| if let BroadCaster(_) = module { true } else { false }).unwrap()
}

fn find_flip_flop<'a>(modules: &'a Vec<Module>, name: &'a str) -> Option<&'a Module> {
    modules.iter().find(|module| if let FlipFlop(f) = module { f.name == name } else { false })
}

fn find_module_i(modules: Vec<Module>, name: &str, module_type: &str) -> Option<usize> {
    let mut i = 0;

    for module in modules {
        match module_type {
            "%" => {
                if let FlipFlop(m) = module {
                    if m.name == name { return Some(i); }
                }
            }
            "&" => {
                if let Conjunction(m) = module {
                    if m.name == name { return Some(i); }
                }
            }
            _ => { return None; }
        }
        i += 1;
    }
    None
}

fn find_conjunction<'a>(modules: &'a Vec<Module>, name: &'a str) -> Option<&'a Module> {
    modules.iter().find(|module| if let Conjunction(c) = module { c.name == name } else { false })
}

fn generate_queue_items(module: &Module, pulse: &Pulse) -> Vec<(String, Pulse, String)> {
    let from_name = module.get_name();

    module.get_output_modules()
        .iter()
        .map(|to_name| (from_name.clone(), pulse.clone(), to_name.clone()))
        .collect::<Vec<(String, Pulse, String)>>()
}

fn update_queue_with_pulse(queue: Vec<(String, Pulse)>, pulse: Pulse) -> Vec<(String, Pulse)> {
    queue.iter().map(|(name, _)| (name.clone(), pulse.clone())).collect::<Vec<(String, Pulse)>>()
}

fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(numbers: Vec<u64>) -> u64 {
    let a = numbers[0];

    if numbers.len() == 1 { return a; }

    let b = lcm(numbers[1..].to_vec());

    return a / gcd(a, b) * b;
}

fn push_button(mut modules: Vec<Module>, module_queue: Vec<(String, Pulse, String)>) -> u64 {
    let mut ln = 0;
    let mut db = 0;
    let mut vq = 0;
    let mut tf = 0;
    let mut count_pushes = 0;
    let mut module_queue = module_queue;

    loop {
        if ln > 0 && db > 0 && vq > 0 && tf > 0 {
            return lcm(vec![ln, db, vq, tf]);
        } else if module_queue.len() == 0 {
            module_queue = vec![("button".to_string(), Low, "broadcaster".to_string())];
            count_pushes += 1;
            continue;
        }

        let (from_name, pulse, to_name) = &module_queue.clone()[0];
        module_queue = module_queue[1..].to_vec();

        match to_name.as_str() {
            "broadcaster" => {
                let broadcaster = find_broadcaster(&modules);
                let queue_items = generate_queue_items(broadcaster, &pulse);

                module_queue.extend(queue_items);
            },
            name => {
                if let Some(index) = find_module_i(modules.clone(), name, "%") {
                    let flip_flop = modules[index].clone();

                    if let FlipFlop(ref mut module) = modules[index] {
                        let new_pulse;

                        if *pulse == High {
                            continue;
                        } else if module.is_on {
                            module.is_on = false;
                            new_pulse = Low;
                        } else {
                            module.is_on = true;
                            new_pulse = High;
                        };

                        let queue_items = generate_queue_items(&flip_flop, &new_pulse);
                        module_queue.extend(queue_items);
                    }
                } else if let Some(index) = find_module_i(modules.clone(), name, "&") {
                    let conjunction = modules[index].clone();
                    let source = from_name.as_str();

                    if pulse == &High {
                        if source == "ln" && ln == 0 {
                            ln = count_pushes;
                        } else if source == "db" && db == 0 {
                            db = count_pushes;
                        } else if source == "vq" && vq == 0 {
                            vq = count_pushes;
                        } else if source == "tf" && tf == 0 {
                            tf = count_pushes;
                        }
                    }

                    if let Conjunction(ref mut module) = modules[index] {
                        module.update_input(from_name.to_string(), pulse.clone());
                        let new_pulse;

                        if module.inputs.iter().all(|(_, input_pulse)| *input_pulse == High) {
                            new_pulse = Low;
                        } else {
                            new_pulse = High;
                        };

                        let queue_items = generate_queue_items(&conjunction, &new_pulse);
                        module_queue.extend(queue_items);
                    }
                } else {
                    continue;
                }
            }
        }
    }
}

pub fn solve() {
    let file = File::open("src/day20/module_configuration.txt").expect("💣");
    let reader = BufReader::new(file);

    let mut modules: Vec<Module> = reader.lines()
        .map(|line| {
            let line = line.unwrap();
            let parts: Vec<&str> = line.split(" -> ").collect();
            let output_modules = parts[1].trim().split(",").map(|s| s.trim().to_string()).collect();
            let module = parts[0];

            if module == "broadcaster" {
                BroadCaster(output_modules)
            } else if module.starts_with("%") {
                FlipFlop(FlipFlopModule { name: module[1..].to_string(), is_on: false, output_modules })
            } else if module.starts_with("&") {
                Conjunction(ConjunctionModule { name: module[1..].to_string(), inputs: Vec::new(), output_modules })
            } else {
                panic!("Nuuuuuuuuuuuuu! 💣")
            }
        })
        .collect();

    for i in 0..modules.len() {
        let name = modules[i].get_name();
        let output_modules = modules[i].get_output_modules();

        for j in 0..modules.len() {
            if let Conjunction(conjunction_module) = &mut modules[j] {
                if output_modules.iter().any(|output_module| conjunction_module.name == *output_module) {
                    conjunction_module.grow_input(name.clone());
                }
            }
        }
    }

    let count = push_button(modules, Vec::new());

    println!("Sum: {}", count);
}
