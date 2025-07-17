use std::{
    char,
    collections::{HashMap, HashSet},
};

fn main() {
    let input_data: Vec<[char; 3]> = vec![
        ['t', 'u', 'p'],
        ['h', 'a', 'p'],
        ['a', 't', 's'],
        ['w', 'h', 's'],
        ['w', 'h', 'i'],
        ['t', 's', 'u'],
        ['t', 'i', 's'],
    ];

    let result_text = recover_secret(input_data) ;
    //  result_text
}

fn recover_secret(triplets: Vec<[char; 3]>) -> String {
        let lookup_table = get_lookup_table(triplets);
    // println!("{:?}", lookup_table);

    let start_letter = lookup_table
        .iter()
        .find(|(_, node)| node.before.is_empty())
        .expect("this must work")
        .0
        .clone();
    let desired_length = lookup_table.keys().len();

    let result =
        dfs_until_solution(&lookup_table, start_letter, desired_length).expect("this must work");

    let result_text: String = result.iter().collect();
    println!("{:?}", result_text);
    result_text
}

#[derive(Debug)]
struct Node {
    letter: char,
    before: HashSet<char>,
    after: HashSet<char>,
}

impl Node {
    fn new(letter: char) -> Self {
        Node {
            letter,
            before: HashSet::new(),
            after: HashSet::new(),
        }
    }
}

fn get_lookup_table(input_data: Vec<[char; 3]>) -> HashMap<char, Node> {
    let mut lookup_table: HashMap<char, Node> = HashMap::new();

    for row in input_data {
        for (index, character) in row.iter().enumerate() {
            if index == 0 {
                let node = lookup_table
                    .entry(*character)
                    .or_insert(Node::new(*character));
                node.after.insert(row[index + 1]);
                node.after.insert(row[index + 2]);
            } else if index == 1 {
                let node = lookup_table
                    .entry(*character)
                    .or_insert(Node::new(*character));
                node.before.insert(row[index - 1]);
                node.after.insert(row[index + 1]);
            } else if index == 2 {
                let node = lookup_table
                    .entry(*character)
                    .or_insert(Node::new(*character));
                node.before.insert(row[index - 1]);
                node.before.insert(row[index - 2]);
            }
        }
    }

    lookup_table
}

fn dfs_until_solution(
    lookup: &HashMap<char, Node>,
    start_letter: char,
    desired_length: usize,
) -> Option<Vec<char>> {
    let mut stack: Vec<(char, Vec<char>)> = vec![(start_letter, vec![start_letter])];

    // let x = stack.pop()

    while let Some((node, path)) = stack.pop() {
        if path.len() == desired_length {
            // println!("{:?}", path);
            return Some(path);
        }

        let children = &lookup[&node].after;
        for child in children {
            if !path.contains(child) {
                let mut new_path = path.clone();
                new_path.push(*child);
                stack.push((*child, new_path));
            }
        }
    }
    return None;
}

