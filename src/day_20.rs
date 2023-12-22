use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};

struct Signal<'a> {
    from: &'a str,
    to: &'a str,
    pulse: bool,
}

type Queue<'a> = VecDeque<Signal<'a>>;

enum Module {
    FlipFlop {
        on: RefCell<bool>,
        out: Vec<String>,
    },
    Conjunction {
        par: RefCell<HashMap<String, bool>>,
        out: Vec<String>,
    },
    Broadcaster {
        out: Vec<String>,
    },
}

impl Module {
    fn parse(s: &str, parent: &mut HashMap<String, Vec<String>>) -> (String, Module) {
        let mut parts = s.split(" -> ");
        let lhs = parts.next().unwrap();
        let rhs: Vec<_> = parts.next().unwrap().split(", ").collect();
        let on = RefCell::new(false);
        let par = RefCell::new(HashMap::new());
        let out: Vec<_> = rhs.iter().map(|&s| String::from(s)).collect();
        let (name, module) = match lhs.as_bytes()[0] {
            b'%' => (String::from(&lhs[1..]), Module::FlipFlop { on, out }),
            b'&' => (String::from(&lhs[1..]), Module::Conjunction { par, out }),
            b'b' => (String::from(lhs), Module::Broadcaster { out }),
            _ => panic!("Invalid input: '{}'", s),
        };
        for dest in rhs {
            parent
                .entry(dest.to_string())
                .or_default()
                .push(name.clone());
        }
        (name, module)
    }

    fn send<'a>(&'a self, signal: Signal<'a>, queue: &mut Queue<'a>) {
        match &self {
            Module::FlipFlop { on, out, .. } => {
                if !signal.pulse {
                    let new_state = !*on.borrow();
                    *on.borrow_mut() = new_state;
                    for dest in out {
                        queue.push_back(Signal {
                            from: signal.to,
                            to: dest,
                            pulse: new_state,
                        });
                    }
                }
            }
            Module::Conjunction { par, out } => {
                *par.borrow_mut().get_mut(signal.from).unwrap() = signal.pulse;
                let pulse = !par.borrow().values().all(|&pulse| pulse);
                for dest in out {
                    queue.push_back(Signal {
                        from: signal.to,
                        to: dest,
                        pulse: pulse,
                    });
                }
            }
            Module::Broadcaster { out } => {
                for dest in out {
                    queue.push_back(Signal {
                        from: signal.to,
                        to: dest,
                        pulse: signal.pulse,
                    });
                }
            }
        };
    }
}

struct Config(HashMap<String, Module>);

impl Config {
    fn read(path: &str) -> Config {
        let mut parent: HashMap<String, Vec<String>> = HashMap::new();
        let config = HashMap::from_iter(
            io::BufReader::new(File::open(path).unwrap())
                .lines()
                .filter_map(Result::ok)
                .map(|line| Module::parse(&line, &mut parent)),
        );
        for (module_name, module) in config.iter() {
            match module {
                Module::Conjunction { par, .. } => {
                    for parent_name in &parent[module_name] {
                        par.borrow_mut().insert(parent_name.clone(), false);
                    }
                }
                _ => (),
            }
        }
        Config(config)
    }

    fn reset(&self) {
        for module in self.0.values() {
            match module {
                Module::FlipFlop { on, .. } => {
                    *on.borrow_mut() = false;
                }
                Module::Conjunction { par, .. } => {
                    for (_, value) in par.borrow_mut().iter_mut() {
                        *value = false
                    }
                }
                _ => (),
            }
        }
    }

    fn run(&self) -> (u64, u64) {
        let mut queue = Queue::new();
        queue.push_back(Signal {
            from: "button",
            to: "broadcaster",
            pulse: false,
        });
        let (mut low, mut high) = (0, 0);
        while let Some(signal) = queue.pop_front() {
            if signal.pulse {
                high += 1;
            } else {
                low += 1;
            }
            if let Some(module) = self.0.get(signal.to) {
                module.send(signal, &mut queue);
            }
        }
        (low, high)
    }

    fn find_parent_conjunction(&self, target: &str) -> &Module {
        self.0
            .iter()
            .find(|(_, module)| match module {
                Module::Conjunction { out, .. } if out.iter().any(|name| name == target) => true,
                _ => false,
            })
            .map(|(_, module)| module)
            .unwrap_or_else(|| panic!("Conjunction module not found"))
    }

    fn run_until_trigger(&self, trigger_name: &str) -> bool {
        let mut queue = Queue::new();
        queue.push_back(Signal {
            from: "button",
            to: "broadcaster",
            pulse: false,
        });
        let mut found = false;
        while let Some(signal) = queue.pop_front() {
            if signal.to == trigger_name && !signal.pulse {
                found = true;
            }
            if let Some(module) = self.0.get(signal.to) {
                module.send(signal, &mut queue);
            }
        }
        found
    }

    fn get_trigger_step(&self, trigger_name: &str) -> u64 {
        let mut step = 1;
        while !self.run_until_trigger(trigger_name) {
            step += 1;
        }
        step
    }
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

pub fn part1(path: &str) {
    let config = Config::read(path);
    let sum = (0..1000)
        .map(|_| config.run())
        .fold((0u64, 0u64), |(acc1, acc2), (x, y)| (acc1 + x, acc2 + y));
    println!("{}", sum.0 * sum.1);
}

pub fn part2(path: &str) {
    let config = Config::read(path);
    let rx_conjunction = config.find_parent_conjunction("rx");
    let rx_conjunction_triggers: Vec<_> = if let Module::Conjunction { par, .. } = rx_conjunction {
        par.borrow().keys().map(|s| s.to_string()).collect()
    } else {
        panic!("Invalid module type");
    };
    let mut mul = 1;
    for trigger_name in rx_conjunction_triggers.iter() {
        let step = config.get_trigger_step(trigger_name);
        config.reset();
        mul = mul * step / gcd(mul, step);
    }
    println!("{mul}");
}
