use std::collections::HashSet;

fn sum_pairs(ints: &[i8], s: i8) -> Option<(i8, i8)> {
    let mut visited_numbers = HashSet::new();
    
    for num in ints {
        let second_num = s - num;
        if visited_numbers.contains(&second_num) {
            return Some((second_num, *num));
        }
        
        visited_numbers.insert(num);
    }
    
    None
}