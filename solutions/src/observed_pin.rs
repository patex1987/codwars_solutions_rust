use itertools::Itertools;
use std::{collections::HashMap, vec};



fn get_pins(observed: &str) -> Vec<String> {
    
    let mut number_neighbors: HashMap<String, Vec<String>> = HashMap::new();
    number_neighbors.insert("1".to_string(), vec!["1".to_string(), "4".to_string(),"2".to_string()]);
    number_neighbors.insert("2".to_string(), vec!["2".to_string(), "1".to_string(),"5".to_string(),"3".to_string()]);
    number_neighbors.insert("3".to_string(), vec!["3".to_string(), "2".to_string(),"6".to_string()]);
    number_neighbors.insert("4".to_string(), vec!["4".to_string(), "1".to_string(),"5".to_string(),"7".to_string()]);
    number_neighbors.insert("5".to_string(), vec!["5".to_string(), "2".to_string(),"4".to_string(),"8".to_string(),"6".to_string()]);
    number_neighbors.insert("6".to_string(), vec!["6".to_string(), "3".to_string(),"5".to_string(),"9".to_string()]);
    number_neighbors.insert("7".to_string(), vec!["7".to_string(), "4".to_string(),"8".to_string()]);
    number_neighbors.insert("8".to_string(), vec!["8".to_string(), "5".to_string(),"7".to_string(),"0".to_string(),"9".to_string()]);
    number_neighbors.insert("9".to_string(), vec!["9".to_string(), "6".to_string(),"8".to_string()]);
    number_neighbors.insert("0".to_string(), vec!["0".to_string(), "8".to_string()]);
    
    let mut all_potential_sequences: Vec<Vec<String>> = vec![];

    for char_as_str in observed.chars().map(|c| c.to_string()) {
        let potential_values = number_neighbors.get(&char_as_str).unwrap().clone();
        all_potential_sequences.push(potential_values);
    }


    let result: Vec<String> = all_potential_sequences
        .into_iter()
        .map(|x| x.into_iter())
        .multi_cartesian_product()
        .map(|x| x.into_iter().collect::<String>())
        .collect();

    println!("{:?}", result);
    result
}