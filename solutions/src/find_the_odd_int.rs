use std::collections::HashMap;

fn find_odd(arr: &[i32]) -> i32 {
    let mut result: i32=0;
    let number_counts: HashMap<i32, i32> = 
        arr
            .iter()
            .fold(HashMap::new(), |mut map, c| {
                *map.entry(*c).or_insert(0) += 1;
                map
            });
    println!("{:?}", number_counts);
    for (key, value) in &number_counts {
        if value % 2 == 1 {
            result = *key;
        }
    }
    
    result
}