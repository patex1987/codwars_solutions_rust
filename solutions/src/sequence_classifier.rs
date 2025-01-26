use preloaded::Order;

fn sequence_classifier(arr: &[i32]) -> Order {
//     println!("input array {:?}", arr);
    let i: usize = arr.len();
    let mut current_result: Order = Order::Constant;
    for n in 1..i {
//         println!("{} - {}", arr[n], arr[n-1]);
        let diff = arr[n] - arr[n-1];
        
        if n == 1 {
            if diff > 0 {
                current_result = Order::Increasing;
            } else if diff < 0 {
                current_result = Order::Decreasing;
            } else if diff == 0 {
                current_result = Order::Constant;
            }
            continue;
        }

        
        if diff > 0 && current_result == Order::Increasing {
            current_result = Order::Increasing;
        } else if diff == 0 && current_result == Order::Increasing {
            current_result = Order::NotDecreasing;
        } else if diff < 0 && current_result == Order::Increasing {
            return Order::Unordered;
            
        } else if diff > 0 && current_result == Order::Decreasing {
            return Order::Unordered;
        } else if diff < 0 && current_result == Order::Decreasing {
               current_result = Order::Decreasing;         
        } else if diff == 0 && current_result == Order::Decreasing {
            current_result = Order::NotIncreasing;
            
        } else if diff > 0 && current_result == Order::NotIncreasing {
            return Order::Unordered;
        } else if diff == 0 && current_result == Order::NotIncreasing {
            current_result = Order::NotIncreasing;
        } else if diff < 0 && current_result == Order::NotIncreasing {
            return Order::NotIncreasing;
            
            
        } else if diff > 0 && current_result == Order::NotDecreasing {
            current_result = Order::NotDecreasing;
        } else if diff == 0 && current_result == Order::NotDecreasing {
            current_result = Order::NotDecreasing;
        } else if diff < 0 && current_result == Order::NotDecreasing {
            return Order::Unordered;
            
        } else if diff > 0 && current_result == Order::Constant {
            current_result = Order::NotDecreasing;
        } else if diff < 0 && current_result == Order::Constant {
            return Order::NotIncreasing;    
            
        } else {
            current_result = Order::Constant;
        }
            
        
    }
    
    return current_result;
}