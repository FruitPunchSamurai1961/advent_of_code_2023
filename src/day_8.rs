use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    name: String,
    left: Option<Weak<RefCell<Node>>>,
    right: Option<Weak<RefCell<Node>>>,
}

impl Node {
    fn new(name: String) -> Node {
        Node {
            name,
            left: None,
            right: None,
        }
    }
}


pub fn solve(input: &str) {
    println!("Solving day 8 problems...");

    let sequence = input.lines().next().unwrap_or("").to_string();
    let nodes_mapping = get_node_mapping_from_input(input);

    let part_one_solution = get_min_steps_to_end(&sequence, &nodes_mapping);

    if part_one_solution != 0 {
        println!("The solution to the first gold star is: {}", part_one_solution);
    }

    let part_two_solution = get_min_steps_to_end_from_vector(&sequence, &nodes_mapping);

    if part_two_solution != 0 {
        println!("The solution to the second gold star is: {}", part_two_solution);
    }
}

fn get_min_steps_to_end(sequence: &str, nodes_mapping: &HashMap<String, Rc<RefCell<Node>>>) -> u64 {
    if let Some(start_node) = nodes_mapping.get("AAA") {
        let mut current_node = start_node.clone();
        let mut steps = 0;
        loop {
            for step in sequence.chars() {
                steps += 1;
                let next_node = match step {
                    'L' => current_node.borrow().left.clone(),
                    'R' => current_node.borrow().right.clone(),
                    _ => None,
                };
                if let Some(weak_node) = next_node {
                    if let Some(strong_node) = weak_node.upgrade() {
                        current_node = strong_node;
                    } else {
                        println!("Broken reference encountered.");
                        return 0;
                    }
                } else {
                    println!("Invalid step in sequence: {}", step);
                    return 0;
                }
            }
            if current_node.borrow().name == "ZZZ" {
                return steps;
            }
        }
    } else {
        println!("Starting node AAA not found.");
        return 0;
    }
}


fn get_node_mapping_from_input(input: &str) -> HashMap<String, Rc<RefCell<Node>>> {
    let mut nodes = HashMap::new();

    // First, create all nodes without connections
    for line in input.lines() {
        if line.contains('=') {
            let parts: Vec<&str> = line.split('=').collect();
            let name = parts[0].trim().to_string();
            nodes.entry(name.clone()).or_insert_with(|| Rc::new(RefCell::new(Node::new(name))));
        }
    }

    // Now, set up connections
    for line in input.lines() {
        if line.contains('=') {
            let parts: Vec<&str> = line.split('=').collect();
            let name = parts[0].trim();
            let children_part = parts[1].trim().trim_matches(|c| c == '(' || c == ')');
            let children: Vec<&str> = children_part.split(',').map(|s| s.trim()).collect();

            if let Some(node) = nodes.get(name) {
                let left_child = children.get(0).and_then(|&child| nodes.get(child)).cloned();
                let right_child = children.get(1).and_then(|&child| nodes.get(child)).cloned();

                if let Some(left) = left_child {
                    node.borrow_mut().left = Some(Rc::downgrade(&left));
                }
                if let Some(right) = right_child {
                    node.borrow_mut().right = Some(Rc::downgrade(&right));
                }
            }
        }
    }
    nodes
}


fn get_min_steps_to_end_from_vector(sequence: &str, node_mapping: &HashMap<String, Rc<RefCell<Node>>>) -> u64 {
    let initial_vec = get_initial_vec(node_mapping);
    let mut path_lengths = Vec::new();

    for node in initial_vec.iter() {
        let length = get_path_len(node, sequence);
        if length > 0 {
            path_lengths.push(length);
        }
    }

    lcm_of_list(&path_lengths)
}

fn get_initial_vec(node_mapping: &HashMap<String, Rc<RefCell<Node>>>) -> Vec<Rc<RefCell<Node>>> {
    let mut vec = Vec::new();
    for node in node_mapping.values() {
        let name = &node.borrow().name;
        if name.ends_with('A') {
            vec.push(node.clone());
        }
    }
    vec
}

fn get_path_len(start_node: &Rc<RefCell<Node>>, sequence: &str) -> u64 {
    let mut current_node = start_node.clone();
    let mut steps = 0;
    loop {
        for step in sequence.chars() {
            steps += 1;
            let next_node = match step {
                'L' => current_node.borrow().left.clone(),
                'R' => current_node.borrow().right.clone(),
                _ => None,
            };
            if let Some(weak_node) = next_node {
                if let Some(strong_node) = weak_node.upgrade() {
                    current_node = strong_node;
                } else {
                    println!("Broken reference encountered.");
                    return 0;
                }
            } else {
                println!("Invalid step in sequence: {}", step);
                return 0;
            }
        }
        if current_node.borrow().name.ends_with('Z') {
            return steps;
        }
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn lcm_of_list(numbers: &[u64]) -> u64 {
    numbers.iter().fold(1, |acc, &x| lcm(acc, x))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        let input = r#"

"#;
        let sequence = "LR";
        let nodes_mapping = crate::day_8::get_node_mapping_from_input(input);
        let result = crate::day_8::get_min_steps_to_end_from_vector(sequence, &nodes_mapping);
        assert_eq!(result, 6);
    }
}