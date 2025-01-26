#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    West,
    South,
}

fn dir_reduc(arr: &[Direction]) -> Vec<Direction> {
    let mut result: Vec<Direction> = arr.to_vec();
    println!("Original array: {:?}", arr);

    loop {
        let values = where_is_right_away(&result);

        match values {
            Some(values) => {
                let (x,y) = values;
                result.remove(y);
                result.remove(x);
                println!("{}{}",x,y);
            } 
            None => break,
        } 
    }
    result
}

fn where_is_right_away(directions: &Vec<Direction>) -> Option<(usize, usize)> {
    if directions.len() == 0 {
        return None;
    }
    for position in 0..directions.len()-1 {
        let left = directions[position];
        let right = directions[position+1];
        let is_pairing = is_pairing_direction(&left, &right);
        if is_pairing {
            return Some((position, position+1));
        }
    }
    
    return None;
}

fn is_pairing_direction(current_elem: &Direction, next_elem: &Direction) -> bool {
    let result = match (current_elem, next_elem) {
        (Direction::North, Direction::South) => true,
        (Direction::South, Direction::North) => true,
        (Direction::East, Direction::West) => true,
        (Direction::West, Direction::East) => true,
        _ => false,
    };
    result
}