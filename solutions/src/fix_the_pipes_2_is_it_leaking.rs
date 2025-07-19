use core::str;
use std::{
    collections::{HashMap, HashSet, VecDeque},
};

fn main() {
    let test_case: [&str; 3] = [
        "╋━━┓", 
        "┃..┃", 
        "┛┛.┣"
    ];
    for line in test_case {
        println!("{}", line);
    }
    let is_leaking = check_pipe(&test_case);
    println!("Is leaking: {}", is_leaking);
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum DIRECTIONS {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum NeighborType {
    EMPTY,
    PIPE,
    WATER,
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct PipeNeighborItem {
    direction: DIRECTIONS,
    neighbor_type: NeighborType,
    valid_connection: bool,
}

impl PipeNeighborItem {
    fn new(direction: DIRECTIONS, neighbor_type: NeighborType, valid_connection: bool) -> Self {
        Self {
            direction,
            neighbor_type,
            valid_connection,
        }
    }
}

fn check_pipe(pipe_map: &[&str]) -> bool {
    let direction_coordinate_change_mapping: HashMap<DIRECTIONS, (i32, i32)> = HashMap::from([
        (DIRECTIONS::UP, (0, -1)),
        (DIRECTIONS::DOWN, (0, 1)),
        (DIRECTIONS::LEFT, (-1, 0)),
        (DIRECTIONS::RIGHT, (1, 0)),
    ]);

    let pipe_direction_mapping: HashMap<char, HashSet<DIRECTIONS>> = HashMap::from([
        ('┃', HashSet::from([DIRECTIONS::UP, DIRECTIONS::DOWN])),
        ('━', HashSet::from([DIRECTIONS::LEFT, DIRECTIONS::RIGHT])),
        ('┓', HashSet::from([DIRECTIONS::LEFT, DIRECTIONS::DOWN])),
        ('┛', HashSet::from([DIRECTIONS::LEFT, DIRECTIONS::UP])),
        ('┗', HashSet::from([DIRECTIONS::UP, DIRECTIONS::RIGHT])),
        ('┏', HashSet::from([DIRECTIONS::RIGHT, DIRECTIONS::DOWN])),
        (
            '┣',
            HashSet::from([DIRECTIONS::UP, DIRECTIONS::RIGHT, DIRECTIONS::DOWN]),
        ),
        (
            '┫',
            HashSet::from([DIRECTIONS::LEFT, DIRECTIONS::UP, DIRECTIONS::DOWN]),
        ),
        (
            '┻',
            HashSet::from([DIRECTIONS::LEFT, DIRECTIONS::UP, DIRECTIONS::RIGHT]),
        ),
        (
            '┳',
            HashSet::from([DIRECTIONS::LEFT, DIRECTIONS::RIGHT, DIRECTIONS::DOWN]),
        ),
        (
            '┫',
            HashSet::from([DIRECTIONS::LEFT, DIRECTIONS::UP, DIRECTIONS::DOWN]),
        ),
        (
            '╋',
            HashSet::from([
                DIRECTIONS::LEFT,
                DIRECTIONS::UP,
                DIRECTIONS::RIGHT,
                DIRECTIONS::DOWN,
            ]),
        ),
    ]);

    let neighbor_facing_direction_mapping: HashMap<DIRECTIONS, DIRECTIONS> = HashMap::from([
        (DIRECTIONS::UP, DIRECTIONS::DOWN),
        (DIRECTIONS::DOWN, DIRECTIONS::UP),
        (DIRECTIONS::LEFT, DIRECTIONS::RIGHT),
        (DIRECTIONS::RIGHT, DIRECTIONS::LEFT),
    ]);

    let mut empty_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut pipe_positions: HashMap<(i32, i32), char> = HashMap::new();

    let mut neighbor_relations: HashMap<(i32, i32), HashMap<DIRECTIONS, PipeNeighborItem>> =
        HashMap::new();
    let mut potential_water_sources: HashSet<(i32, i32)> = HashSet::new();
    let mut nodes_connected_with_empty_positions: HashSet<(i32, i32)> = HashSet::new();

    let height = pipe_map.len();
    let width = pipe_map[0].chars().count();

    println!("H:{}, W: {}", height, width);

    for (row_nr, row) in pipe_map.iter().enumerate() {
        for (col_nr, character) in row.chars().into_iter().enumerate() {
            let current_coordinate = (col_nr as i32, row_nr as i32);

            if character == '.' {
                empty_positions.insert(current_coordinate);
            } else if character != '.' {
                pipe_positions.insert(current_coordinate, character);

                let directions = pipe_direction_mapping.get(&character);

                let directions = match directions {
                    Some(directions) => directions,
                    None => {
                        println!("Invalid character: {}", character);
                        panic!("There is an invalid character")
                    }
                };

                for direction in directions.iter() {
                    let is_neighbor_water =
                        is_water_source(direction, col_nr, row_nr, height, width);
                    if is_neighbor_water {
                        potential_water_sources.insert(current_coordinate);
                    }

                    let neighbors = neighbor_relations
                        .entry(current_coordinate)
                        .or_insert(HashMap::new());
                    if is_neighbor_water {
                        let neighbor_item =
                            PipeNeighborItem::new(direction.clone(), NeighborType::WATER, true);
                        neighbors.insert(direction.clone(), neighbor_item);
                        continue;
                    }

                    let valid_neighbor = is_neighbor_a_valid_connection(
                        direction,
                        current_coordinate,
                        pipe_map,
                        &direction_coordinate_change_mapping,
                        &neighbor_facing_direction_mapping,
                        &pipe_direction_mapping,
                    );

                    if valid_neighbor == false {
                        nodes_connected_with_empty_positions.insert(current_coordinate);
                    }

                    let neighbor_item = PipeNeighborItem::new(
                        direction.clone(),
                        if valid_neighbor {
                            NeighborType::PIPE
                        } else {
                            NeighborType::EMPTY
                        },
                        valid_neighbor,
                    );
                    neighbors.insert(direction.clone(), neighbor_item);
                }
            }
        }
    }

    // debug pretty printing

    // // println!("{:?}", neighbor_relations);
    // for (coordinate, coordinate_state) in &neighbor_relations {
    //     println!("Neighbors for coordinate {:?}", coordinate);
    //     for (direction, neighbor) in coordinate_state {
    //         println!("\t{:?} -> {:?}", direction, neighbor);
    //     }
    // }
    // // println!("water sources: {:?}", potential_water_sources);

    let is_leaking = assess_by_walking_from_empty_nodes(
        &nodes_connected_with_empty_positions,
        &neighbor_relations,
    );

    is_leaking
}

fn assess_by_walking_from_empty_nodes(
    nodes_connected_to_empty_positions: &HashSet<(i32, i32)>,
    neighbor_relations: &HashMap<(i32, i32), HashMap<DIRECTIONS, PipeNeighborItem>>,
) -> bool {
    let mut seen_pipe_connected_to_empty: HashSet<(i32, i32)> = HashSet::new();

    let mut valid_nodes_connected_to_empty: HashSet<(i32, i32)> =
        nodes_connected_to_empty_positions
            .difference(&seen_pipe_connected_to_empty)
            .cloned()
            .collect();

    while !valid_nodes_connected_to_empty.is_empty() {
        let cloned_valid_nodes_connected_to_empty: Vec<(i32, i32)> =
            valid_nodes_connected_to_empty.iter().cloned().collect();

        for pipe_node_connected_to_empty in cloned_valid_nodes_connected_to_empty {
            seen_pipe_connected_to_empty.insert(pipe_node_connected_to_empty);
            let walk_result =
                walk_from_empty_node(&pipe_node_connected_to_empty, &neighbor_relations);
            if walk_result.is_leaking == true {
                return true;
            }

            seen_pipe_connected_to_empty.extend(walk_result.empty_nodes_visited);

            valid_nodes_connected_to_empty = nodes_connected_to_empty_positions
                .difference(&seen_pipe_connected_to_empty)
                .cloned()
                .collect();
        }
    }

    false
}

struct EmtyNodeWalkResult {
    is_leaking: bool,
    empty_nodes_visited: HashSet<(i32, i32)>,
}

impl EmtyNodeWalkResult {
    fn new(is_leaking: bool, empty_nodes_visited: HashSet<(i32, i32)>) -> Self {
        Self {
            is_leaking,
            empty_nodes_visited,
        }
    }
}

fn walk_from_empty_node(
    pipe_node_connected_to_empty: &(i32, i32),
    neighbor_relations: &HashMap<(i32, i32), HashMap<DIRECTIONS, PipeNeighborItem>>,
) -> EmtyNodeWalkResult {
    let mut queue: VecDeque<((i32, i32), Vec<(i32, i32)>)> = VecDeque::new();
    let mut seen_nodes: HashSet<(i32, i32)> = HashSet::new();
    let mut empty_nodes_visited: HashSet<(i32, i32)> = HashSet::new();
    let mut is_leaking = false;

    queue.push_back((
        *pipe_node_connected_to_empty,
        vec![*pipe_node_connected_to_empty],
    ));

    while !queue.is_empty() {
        let (current_node, path) = queue.pop_front().expect("there should be a node");

        if seen_nodes.contains(&current_node) {
            continue;
        }

        seen_nodes.insert(current_node);

        let neighbors = neighbor_relations
            .get(&current_node)
            .expect("there should be neighbors");

        for (direction, neighbor) in neighbors.iter() {
            if neighbor.neighbor_type == NeighborType::EMPTY {
                if !path.contains(&current_node) {
                    empty_nodes_visited.insert(current_node);
                }
                continue;
            }

            if neighbor.neighbor_type == NeighborType::WATER && neighbor.valid_connection == true {
                is_leaking = true;
                return EmtyNodeWalkResult::new(is_leaking, empty_nodes_visited);
            }

            if neighbor.valid_connection {
                let coordinate_change = match direction {
                    DIRECTIONS::UP => (0, -1),
                    DIRECTIONS::DOWN => (0, 1),
                    DIRECTIONS::LEFT => (-1, 0),
                    DIRECTIONS::RIGHT => (1, 0),
                };

                let neighbor_coordinate = (
                    current_node.0 + coordinate_change.0,
                    current_node.1 + coordinate_change.1,
                );

                if !path.contains(&neighbor_coordinate) {
                    let mut new_path = path.clone();
                    new_path.push(neighbor_coordinate);
                    queue.push_back((neighbor_coordinate, new_path));
                }
            }
        }
    }

    EmtyNodeWalkResult::new(is_leaking, empty_nodes_visited)
}

fn is_neighbor_a_valid_connection(
    direction: &DIRECTIONS,
    current_coordinate: (i32, i32),
    pipe_map: &[&str],
    direction_coordinate_change_mapping: &HashMap<DIRECTIONS, (i32, i32)>,
    neighbor_facing_direction_mapping: &HashMap<DIRECTIONS, DIRECTIONS>,
    pipe_direction_mapping: &HashMap<char, HashSet<DIRECTIONS>>,
) -> bool {
    let coordinate_change = direction_coordinate_change_mapping
        .get(direction)
        .expect("coordinate change should be avail");
    let neighbor_x = current_coordinate.0 + coordinate_change.0;
    let neighbor_y = current_coordinate.1 + coordinate_change.1;

    let neighbor_character = pipe_map[neighbor_y as usize]
        .chars()
        .nth(neighbor_x as usize)
        .expect("all these neighbors should be valid");

    if neighbor_character == '.' {
        return false;
    }

    let expected_opposite_direction = neighbor_facing_direction_mapping
        .get(direction)
        .expect("there should be a mapping for this direction");

    let neighbors_available_directions = pipe_direction_mapping
        .get(&neighbor_character)
        .expect("This character should be available in the pipe mapping");

    let is_available = neighbors_available_directions.contains(expected_opposite_direction);
    is_available
}

fn is_water_source(
    direction: &DIRECTIONS,
    x: usize,
    y: usize,
    height: usize,
    width: usize,
) -> bool {
    let last_row = height - 1;
    let last_col = width - 1;

    let res = match (direction, x, y) {
        // top left
        (DIRECTIONS::UP, 0, 0) => true,
        (DIRECTIONS::LEFT, 0, 0) => true,
        // bottom left
        (DIRECTIONS::LEFT, 0, y) if y == last_row => true,
        (DIRECTIONS::DOWN, 0, y) if y == last_row => true,
        // top right
        (DIRECTIONS::UP, x, 0) if x == last_col => true,
        (DIRECTIONS::RIGHT, x, 0) if x == last_col => true,
        // bottom right
        (DIRECTIONS::RIGHT, x, y) if x == last_col && y == last_row => true,
        (DIRECTIONS::DOWN, x, y) if x == last_col && y == last_row => true,
        // top row
        (DIRECTIONS::UP, _, 0) => true,
        // bottom row
        (DIRECTIONS::DOWN, _, y) if y == last_row => true,
        // left column
        (DIRECTIONS::LEFT, 0, _) => true,
        // right column
        (DIRECTIONS::RIGHT, x, _) if x == last_col => true,
        _ => false,
    };

    res
}

// fn is_neighbor_out_of_field(
//     direction: &DIRECTIONS,
//     x: usize,
//     y: usize,
//     height: usize,
//     width: usize,
// ) -> bool {
//     let last_row = height - 1;
//     let last_col = width - 1;

//     let res = match (direction, x, y) {
//         // top row
//         (DIRECTIONS::UP, _, 0) => true,
//         // bottom row
//         (DIRECTIONS::DOWN, _, y) if y == last_row => true,
//         // left column
//         (DIRECTIONS::LEFT, 0, _) => true,
//         // right column
//         (DIRECTIONS::RIGHT, x, _) if x == last_col => true,
//         _ => false,
//     };

//     res
// }
