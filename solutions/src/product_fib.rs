fn product_fib(prod: u64) -> (u64, u64, bool) {
    let number_found = false;
    let mut n_2 = 0;
    let mut n_1 = 1;
    let mut current_num = n_2 + n_1;
    let mut current_product: u64;
    while !number_found {
        current_product = current_num * n_1;
//         println!("new iteration: {} -> {} = {}*{}", prod, current_product, previous_num, current_num);
        if current_product == prod {
            return (n_1, current_num , true);
        } else if current_product > prod {
            return (n_1, current_num , false); 
        }
        println!("fib: {} + {} = {}", n_2, n_1, n_2+n_1);
        n_2 = n_1;
        n_1 = current_num;
        current_num = n_2+n_1;
        
        
        
    }
    
    (n_1, current_num, true)
}