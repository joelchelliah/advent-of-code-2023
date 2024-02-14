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
    IfGreaterThan(String, u32, WorkflowName),
    IfLessThan(String, u32, WorkflowName),
    Else(WorkflowName),
}
use Rule::*;

#[derive(Clone)]
struct Workflow {
    name: WorkflowName,
    rules: Vec<Rule>,
}

struct Rating {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Rating {
    fn sum(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}

fn follow_rule(rating: &Rating, rule: &Rule) -> Option<WorkflowName> {
    match rule {
        IfGreaterThan(category, compare_value, next_workflow) => {
            if [
                category == "x" && rating.x > *compare_value,
                category == "m" && rating.m > *compare_value,
                category == "a" && rating.a > *compare_value,
                category == "s" && rating.s > *compare_value,
            ].iter().any(|x| *x) {
                Some(next_workflow.clone())
            } else { None }
        }
        IfLessThan(category, compare_value, next_workflow) => {
            if [
                category == "x" && rating.x < *compare_value,
                category == "m" && rating.m < *compare_value,
                category == "a" && rating.a < *compare_value,
                category == "s" && rating.s < *compare_value,
            ].iter().any(|x| *x) {
                Some(next_workflow.clone())
            } else { None }
        }
        Else(next_workflow) => { Some(next_workflow.clone()) }
    }
}

fn is_rating_accepted(rating: &Rating, name: WorkflowName, workflows: &[Workflow]) -> bool {
    if WorkflowName::Accept == name { return true }
    if WorkflowName::Reject == name { return false }

    let workflow = workflows.iter().find(|workflow| workflow.name == name).unwrap();

    for rule in &workflow.rules {
        if let Some(found_name) = follow_rule(rating, rule) {
            return is_rating_accepted(rating, found_name, workflows);
        }
    }
    true
}

pub fn solve() {
    let file = File::open("src/day19/workflows_and_ratings.txt").expect("💣");
    let reader = BufReader::new(file);

    let mut parsing_workflows = true;
    let mut workflows: Vec<Workflow> = Vec::new();
    let mut ratings: Vec<Rating> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Nuuuu! 💣");
        if line.trim().is_empty() {
            parsing_workflows = false;
            continue
        }

        if parsing_workflows {
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
                        let compare_value = rest[0].parse::<u32>().unwrap();
                        let next_workflow = WorkflowName::from_string(rest[1]);
                        IfLessThan(category, compare_value, next_workflow)
                    } else if rule_as_str.contains(">") {
                        let category = rule_as_str.chars().next().unwrap().to_string();
                        let rest: Vec<&str> = rule_as_str[2..].split(":").collect();
                        let compare_value = rest[0].parse::<u32>().unwrap();
                        let next_workflow = WorkflowName::from_string(rest[1]);
                        IfGreaterThan(category, compare_value, next_workflow)
                    } else {
                        Else(WorkflowName::from_string(rule_as_str))
                    }
                }).collect()
            });
        } else {
            let parts: Vec<&str> = line[1..line.len() - 1].split(",").collect();
            ratings.push(Rating {
                x: parts[0][2..].parse::<u32>().unwrap(),
                m: parts[1][2..].parse::<u32>().unwrap(),
                a: parts[2][2..].parse::<u32>().unwrap(),
                s: parts[3][2..].parse::<u32>().unwrap(),
            });
        }

    }

    let sum = ratings.iter().filter(|rating| {
        is_rating_accepted(rating, WorkflowName::Label("in".to_string()), &workflows)
    }).fold(0, |acc, rating| acc + rating.sum());

    println!("Sum: {}", sum);
}
