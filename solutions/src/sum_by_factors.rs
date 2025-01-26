use std::{collections::HashMap, vec};

fn sum_of_divided(l: Vec<i64>) -> Vec<(i64, i64)> {
    println!("input is: {:?}", l);
    
    if l.len() == 0 {
        return vec![];
    }
    let max_val: i64 = l    
        .iter()
        .map(|&x| x.abs())
        .max()
        .unwrap();

    let prime_factors = sieve(max_val);
    // println!("Prime factors of {:?} are {:?}", max_val, prime_factors);

    let prime_factor_sum_map = get_prime_factor_sum_pairs(l, prime_factors);
    // println!("Prime factor sum pairs are {:?}", prime_factor_sum_map);

    let mut prime_factor_sum_pairs: Vec<(i64, i64)> = prime_factor_sum_map.into_iter().collect();
    prime_factor_sum_pairs.sort_by(|a, b| a.0.cmp(&b.0));
    return prime_factor_sum_pairs;
}

fn sieve(n: i64) -> Vec<i64> {
    let mut bool_mask = vec![true; n as usize + 1];
    bool_mask[0] = false;
    bool_mask[1] = false;

    for i in (4..n + 1).step_by(2) {
        bool_mask[i as usize] = false;
    }

    let mut i = 3;
    while i * i <= n + 1 {
        if !bool_mask[i as usize] {
            i += 2;
            continue;
        }

        for j in (i * i..n + 1).step_by((2 * i) as usize) {
            bool_mask[j as usize] = false;
        }
        i += 2;
    }

    let mut primes: Vec<i64> = Vec::new();
    for i in 0..n + 1 {
        if bool_mask[i as usize] {
            primes.push(i);
        }
    }
    primes

    // functional approach
    // let result: Vec<i64> = bool_mask
    //     .iter()
    //     .enumerate()
    //     .filter(|&(_i, &val)| val)
    //     .map(|(i, _val)| i as i64)
    //     .collect();
}

fn get_prime_factor_sum_pairs(numbers: Vec<i64>, prime_factors: Vec<i64>) -> HashMap<i64, i64> {
    // TODO: create a set too for faster presence checks
    let mut prime_number_mapping: HashMap<i64, i64> = HashMap::new();
    for num in numbers {
        let limit = (num.abs() / 2) as i64 + 1;
        if prime_factors.contains(&num.abs()) {
            *prime_number_mapping.entry(num.abs()).or_insert(0) += num;
            continue;
        }
        for prime_num in &prime_factors {
//             println!(
//                 "checking {} against prime: {}, limit: {}",
//                 num, prime_num, limit
//             );
            if *prime_num > limit {
                break;
            }
            if num % prime_num != 0 {
                continue;
            }
//             println!("[REACHED] checking {} against prime: {}", num, prime_num);
            *prime_number_mapping.entry(*prime_num).or_insert(0) += num;
        }
    }
    return prime_number_mapping;
}