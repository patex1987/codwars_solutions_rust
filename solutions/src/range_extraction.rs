use num::abs;

mod solution {
    
    pub fn range_extraction(a: &[i32]) -> String {
        println!("{:?}", a);
        let mut diff = 0;
        let mut solution_vec: Vec<String> = vec![];
        let mut current_lower_bound = a[0];
        let mut current_consecutive_length = 1;
        let mut current_upper_bound = a[0];
        let max_len = a.len();
        for i in 1..max_len {
            diff = num::abs(a[i] - a[i-1]);
            if diff != 1 {
                let range: String = match current_consecutive_length {
                    1 => format!("{}", current_lower_bound),
                    2 => format!("{},{}", current_lower_bound, current_upper_bound),
                    _ => format!("{}-{}", current_lower_bound, current_upper_bound),
                };
                current_lower_bound = a[i];
                current_upper_bound = a[i];
                current_consecutive_length = 1;
                solution_vec.push(range);
            }
            else if diff == 1 {
                current_consecutive_length += 1;
                current_upper_bound = a[i];
            }
            
            if i == max_len -1 {
                let range: String = match current_consecutive_length {
                    1 => format!("{}", current_lower_bound),
                    2 => format!("{},{}", current_lower_bound, current_upper_bound),
                    _ => format!("{}-{}", current_lower_bound, current_upper_bound),
                };
                solution_vec.push(range);
            }

            
        }
//         println!("{:?}", solution_vec.join(","));
        format!("{}", solution_vec.join(","))
    }
}