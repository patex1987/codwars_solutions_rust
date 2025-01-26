fn find(s: &str) -> u32 {
    let mut actual_slice_size: usize;
    let mut current_start_index: usize;
    let word_length = s.len();
    let mut is_valid_sequence: bool;
    let mut expected_next_number: u128;
    let mut sequence_start_number: u128 = s[0..1].parse().expect("not a number");
    println!("full number: {}", s);
    
    
    for initial_slice_size in 1..word_length+1 {
//         println!("actual slice: {}", initial_slice_size);
        is_valid_sequence = true;
        actual_slice_size = initial_slice_size;
        current_start_index = 0;
        
        expected_next_number = s[0..actual_slice_size].parse().expect("not a number");
        sequence_start_number = expected_next_number;
        
        while is_valid_sequence {
            if current_start_index >= word_length {
                println!("Number found: {}", sequence_start_number);
                return sequence_start_number.try_into().expect("number can't be converted into i32")
            }
            let current_finish_index = current_start_index + actual_slice_size;
            if current_finish_index > word_length {
                break;
            }
            let current_number: u128 = s[current_start_index..current_finish_index].parse().expect("not a number");

            if current_number != expected_next_number {
                println!("{} - {} current number: {}, expected next number: {}. Numbers don't match", 
                    current_start_index, current_finish_index, current_number, expected_next_number);
                break;
            }
            
            expected_next_number = current_number + 1;
            actual_slice_size = ((expected_next_number as f64).log10().floor() as usize) + 1;
            if actual_slice_size == word_length {
                println!("reached the end of the number");
                break;
            }
//             println!("{} - {} current number: {}, expected next number: {}. ---", 
//                     current_start_index, current_finish_index, current_number, expected_next_number);
            current_start_index = current_finish_index;
        }
        
        
    }
    
    sequence_start_number.try_into().expect("Number can't be converted into i32")
}