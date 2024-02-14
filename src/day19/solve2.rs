use std::io::{BufRead, BufReader};
use std::fs::File;

#[derive(Clone, PartialEq)]
enum WorkflowName {
    Accept,
    Reject,
    Label(String),
}

impl WorkflowName {
    fn from_string(name: &str) -> WorkflowName {
        match name {
            "A" => WorkflowName::Accept,
            "R" => WorkflowName::Reject,
            _ => WorkflowName::Label(name.to_string()),
        }
    }
}

#[derive(Clone)]
enum Rule {
    IfGreaterThan(String, u64, WorkflowName),
    IfLessThan(String, u64, WorkflowName),
    Else(WorkflowName),
}
use Rule::*;

#[derive(Clone)]
struct Workflow {
    name: WorkflowName,
    rules: Vec<Rule>,
}

#[derive(Clone)]
struct RatingBounds {
    x: (u64, u64),
    m: (u64, u64),
    a: (u64, u64),
    s: (u64, u64),
}

impl RatingBounds {
    fn sum(&self) -> u64 {
        (self.x.1 - self.x.0 + 1) * (self.m.1 - self.m.0 + 1) * (self.a.1 - self.a.0 + 1) * (self.s.1 - self.s.0 + 1)
    }

    fn update(&self, category: &str, op: &str, value: u64) -> RatingBounds {
        RatingBounds {
            x:  if category == "x" && op == ">" {(value + 1, self.x.1)}
                else if category == "x" && op == "<" {(self.x.0, value - 1)}
                else { self.x },
            m:  if category == "m" && op == ">" {(value + 1, self.m.1)}
                else if category == "m" && op == "<" {(self.m.0, value - 1)}
                else { self.m },
            a:  if category == "a" && op == ">" {(value + 1, self.a.1)}
                else if category == "a" && op == "<" {(self.a.0, value - 1)}
                else { self.a },
            s:  if category == "s" && op == ">" {(value + 1, self.s.1) }
                else if category == "s" && op == "<" {(self.s.0, value - 1)}
                else { self.s },
        }
    }

    fn inverse(&self, category: &str, op: &str, value: u64) -> RatingBounds {
        RatingBounds {
            x:  if category == "x" && op == ">" {(self.x.0, value)}
                else if category == "x" && op == "<" {(value, self.x.1)}
                else { self.x },
            m:  if category == "m" && op == ">" {(self.m.0, value)}
                else if category == "m" && op == "<" {(value, self.m.1)}
                else { self.m },
            a:  if category == "a" && op == ">" {(self.a.0, value)}
                else if category == "a" && op == "<" {(value, self.a.1)}
                else { self.a },
            s:  if category == "s" && op == ">" {(self.s.0, value)}
                else if category == "s" && op == "<" {(value, self.s.1)}
                else { self.s },
        }
    }
}

fn count_ratings(workflows: &[Workflow], name: WorkflowName, rating_bounds: RatingBounds) -> u64 {
    if WorkflowName::Accept == name { return rating_bounds.sum() }
    if WorkflowName::Reject == name { return 0 }

    let workflow = workflows.iter().find(|workflow| workflow.name == name).unwrap();

    let start_state = (0, rating_bounds);
    let result = workflow.rules.iter().fold(start_state, |acc, rule| {
        let (sum, bounds) = acc;

        match rule {
            IfGreaterThan(category, compare_value, next_workflow) => {
                let new_bounds = bounds.update(category, ">", *compare_value);
                let new_sum = sum + count_ratings(workflows, next_workflow.clone(), new_bounds);

                (new_sum, bounds.inverse(category, ">", *compare_value))
            }
            IfLessThan(category, compare_value, next_workflow) => {
                let new_bounds = bounds.update(category, "<", *compare_value);
                let new_sum = sum + count_ratings(workflows, next_workflow.clone(), new_bounds);

                (new_sum, bounds.inverse(category, "<", *compare_value))
            }
            Else(next_workflow) => {
                (sum + count_ratings(workflows, next_workflow.clone(), bounds.clone()), bounds)
            }
        }
    });
    result.to_owned().0
}


pub fn solve() {
    let file = File::open("src/day19/workflows_and_ratings.txt").expect("💣");
    let reader = BufReader::new(file);

    let mut workflows: Vec<Workflow> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Nuuuu! 💣");
        if line.trim().is_empty() { break; }

        let parts: Vec<&str> = line.split("{").collect();
        let name = parts[0].trim();
        let rules_as_str = parts[1][..parts[1].len() - 1].split(",").collect::<Vec<&str>>();

        workflows.push(Workflow {
            name: WorkflowName::Label(name.to_string()),
            rules: rules_as_str.iter().map(|rule| {
                let rule_as_str = rule.trim();

                if rule_as_str.contains("<") {
                    let category = rule_as_str.chars().next().unwrap().to_string();
                    let rest: Vec<&str> = rule_as_str[2..].split(":").collect();
                    let compare_value = rest[0].parse::<u64>().unwrap();
                    let next_workflow = WorkflowName::from_string(rest[1]);
                    IfLessThan(category, compare_value, next_workflow)
                } else if rule_as_str.contains(">") {
                    let category = rule_as_str.chars().next().unwrap().to_string();
                    let rest: Vec<&str> = rule_as_str[2..].split(":").collect();
                    let compare_value = rest[0].parse::<u64>().unwrap();
                    let next_workflow = WorkflowName::from_string(rest[1]);
                    IfGreaterThan(category, compare_value, next_workflow)
                } else {
                    Else(WorkflowName::from_string(rule_as_str))
                }
            }).collect()
        });
    }

    let start_state = RatingBounds { x: (1, 4000), m: (1, 4000), a: (1, 4000), s: (1, 4000) };
    let count = count_ratings(&workflows, WorkflowName::Label("in".to_string()), start_state);

    println!("Sum: {}", count);
}
