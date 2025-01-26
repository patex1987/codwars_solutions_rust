use std::collections::HashMap;
use std::collections::hash_map::Entry;

fn order_weight(s: &str) -> String {
    if s.is_empty() {
        return String::from("");
    }
    
    let words: Vec<&str> = s.split(" ").collect();
    let mut weights: HashMap<i64, Vec<String>> = HashMap::new();
    let mut sums: Vec<i64> = Vec::new();
    println!("--------New unit--------");
    println!("{:?}", words);
    
    for str_num in words {
        let my_int = str_num.parse::<i64>().expect("Couldnt parse the number");
//         println!("{}", my_int);
        let digits = get_digits(my_int);
        let digit_sum = digits.sum::<i64>();
//         println!("sum of digits: {}", digit_sum);
        
        insert_ordered_into_sums(&mut sums, digit_sum);
        insert_into_weights(&mut weights, digit_sum, str_num);
//         println!("--------Round--------");
//         println!("{:?}", sums);
//         println!("{:?}", weights);
    }
    
    let result: String = sums.iter()
        .map(|sum_number| weights.get(sum_number).expect("Missing key").join(" "))
        .collect::<Vec<String>>()
        .join(" ");
//     println!("{:?}", result);
    
    result
}

fn insert_ordered_into_sums(sum_vec: &mut Vec<i64>, value: i64) {
    let position_to_insert = sum_vec.binary_search(&value);
//     println!("{:?}", position_to_insert);
    match position_to_insert {
        Ok(_) => {}
        Err(pos) => sum_vec.insert(pos, value)
    }
//     println!("{:?}", sum_vec);
}

fn insert_into_weights(weights: &mut HashMap<i64, Vec<String>> , digit_sum: i64, str_num: &str){
    let num_text = str_num.to_string();
    
    let entry: Entry<i64, Vec<String>> = weights.entry(digit_sum);
    match entry {
        Entry::Vacant(e) => { 
            e.insert(vec![num_text]); 
        },
        Entry::Occupied(mut e) => { 
            let existing_numbers = e.get_mut();
            let position_to_insert = existing_numbers.binary_search(&num_text);
            match position_to_insert {
                Ok(pos) => existing_numbers.insert(pos, num_text),
                Err(pos) => existing_numbers.insert(pos, num_text)
            }
        }
    }
}

fn get_digits(mut num: i64) -> impl Iterator<Item = i64>{
    let mut divisor = 1;
    while num >= divisor * 10 {
        divisor *= 10;
    }

    std::iter::from_fn(move || {
        if divisor == 0 {
            None
        } else {
            let v = num / divisor;
            num %= divisor;
            divisor /= 10;
            Some(v)
        }
    })
}