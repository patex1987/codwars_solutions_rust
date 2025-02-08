use std::{
    char,
    collections::{BTreeMap, BTreeSet, HashMap},
};

fn main() {
    // println!("Hello, world!");
    let text_1 = "yes we are ";
    let text_2 = "yyes we are rrrrr";
    let x = mix(text_1, text_2);
    println!("{}", x);
}

fn mix(s1: &str, s2: &str) -> String {
    let mut position = 0;
    let mut word_1_finished = false;
    let mut word_2_finished = false;
    let mut ordered_counts_to_chars: BTreeMap<i32, BTreeSet<char>> = BTreeMap::new();

    let mut char_count_mapping: HashMap<char, (i32, i32)> = HashMap::new();

    while !(word_1_finished && word_2_finished) {
        let char_1 = s1.chars().nth(position);
        let char_2 = s2.chars().nth(position);

        if char_1.is_none() && char_2.is_none() {
            break;
        }

        if char_1.is_none() {
            word_1_finished = true;
        }

        if char_2.is_none() {
            word_2_finished = true;
        }

        let mut previous_char_1_counts: Option<(i32, i32)> = None;
        let mut updated_char_1_counts: Option<(i32, i32)> = None;
        if is_character_valid(&char_1) {
            let real_char_1 = char_1.unwrap();
            previous_char_1_counts =
                Some(get_char_both_counts(&real_char_1, &mut char_count_mapping));
            update_char_mapping(&real_char_1, WordType::First, &mut char_count_mapping);
            updated_char_1_counts =
                Some(get_char_both_counts(&real_char_1, &mut char_count_mapping));
            // println!(
            //     "char determination [1][position: {:?}] - char: {:?}, previous_counts: {:?}, updated_counts: {:?}",
            //     position, real_char_1, previous_char_1_counts, updated_char_1_counts
            // );
        }

        let mut previous_char_2_counts: Option<(i32, i32)> = None;
        let mut updated_char_2_counts: Option<(i32, i32)> = None;
        if is_character_valid(&char_2) {
            let real_char_2 = char_2.unwrap();
            previous_char_2_counts =
                Some(get_char_both_counts(&real_char_2, &mut char_count_mapping));
            update_char_mapping(&real_char_2, WordType::Second, &mut char_count_mapping);
            updated_char_2_counts =
                Some(get_char_both_counts(&real_char_2, &mut char_count_mapping));
            // println!(
            //     "char determination [2][position:{:?}] - char: {:?}, previous_counts: {:?}, updated_counts: {:?}",
            //     position, real_char_2, previous_char_2_counts, updated_char_2_counts
            // );
        }

        let mut equal_chars = false;
        if is_character_valid(&char_1) && is_character_valid(&char_2) {
            let real_char_1 = char_1.unwrap();
            let real_char_2 = char_2.unwrap();

            if real_char_1 == real_char_2 {
                equal_chars = true;
                // println!("characters equals on both sides - position: {:?}, character: {:?}", position, real_char_1);
                let (count_to_remove_from, count_to_add_to) =
                    get_equal_char_remove_add_count_positions(
                        previous_char_1_counts,
                        updated_char_1_counts,
                        previous_char_2_counts,
                        updated_char_2_counts,
                    );

                remove_char_from_counts(
                    &mut ordered_counts_to_chars,
                    count_to_remove_from,
                    &real_char_1,
                );
                add_char_to_counts(&mut ordered_counts_to_chars, count_to_add_to, real_char_1);
            }
        }

        if !equal_chars && is_character_valid(&char_1) {
            let real_char_1 = char_1.unwrap();
            let (count_to_remove_from, count_to_add_to) =
                get_single_char_remove_add_count_positions(
                    previous_char_1_counts,
                    updated_char_1_counts,
                );

            if count_to_remove_from.is_some() {
                remove_char_from_counts(
                    &mut ordered_counts_to_chars,
                    count_to_remove_from,
                    &real_char_1,
                );
            }

            if count_to_add_to.is_some() {
                add_char_to_counts(&mut ordered_counts_to_chars, count_to_add_to, real_char_1);
            }
        }

        if !equal_chars && is_character_valid(&char_2) {
            let real_char_2 = char_2.unwrap();
            let (count_to_remove_from, count_to_add_to) =
                get_single_char_remove_add_count_positions(
                    previous_char_2_counts,
                    updated_char_2_counts,
                );

            if count_to_remove_from.is_some() {
                remove_char_from_counts(
                    &mut ordered_counts_to_chars,
                    count_to_remove_from,
                    &real_char_2,
                );
            }

            if count_to_add_to.is_some() {
                add_char_to_counts(&mut ordered_counts_to_chars, count_to_add_to, real_char_2);
            }
        }

        position += 1;
    }

    println!("{:?}", char_count_mapping);
    println!("{:?}", ordered_counts_to_chars);

    // TODO: pretty printing here, and return the pretty printed string (check the task for how to do the pretty printing properly)
    String::from("result")
}

fn get_single_char_remove_add_count_positions(
    previous_char_counts: Option<(i32, i32)>,
    updated_char_counts: Option<(i32, i32)>,
) -> (Option<i32>, Option<i32>) {
    let count_to_remove_from = match previous_char_counts {
        Some((char_left_count, char_right_count)) => {
            let count_to_remove_from = if char_left_count > char_right_count {
                char_left_count
            } else {
                char_right_count
            };
            Some(count_to_remove_from)
        }
        None => None,
    };
    let count_to_add_to = match updated_char_counts {
        Some((char_left_count, char_right_count)) => {
            let count_to_add_to = if char_left_count > char_right_count {
                char_left_count
            } else {
                char_right_count
            };
            Some(count_to_add_to)
        }
        None => None,
    };
    (count_to_remove_from, count_to_add_to)
}

fn get_equal_char_remove_add_count_positions(
    previous_char_1_counts: Option<(i32, i32)>,
    updated_char_1_counts: Option<(i32, i32)>,
    previous_char_2_counts: Option<(i32, i32)>,
    updated_char_2_counts: Option<(i32, i32)>,
) -> (Option<i32>, Option<i32>) {
    let count_to_remove_from = match (previous_char_1_counts, previous_char_2_counts) {
        (Some(prev_char_1_counts), Some(_prev_char_2_counts)) => {
            let (char_1_left_count, char_1_right_count) = prev_char_1_counts;

            let count_to_remove_from = if char_1_left_count > char_1_right_count {
                char_1_left_count
            } else {
                char_1_right_count
            };
            Some(count_to_remove_from)
        }
        (Some(previous_char_1_counts), None) => {
            let (char_1_left_count, char_1_right_count) = previous_char_1_counts;

            let count_to_remove_from = if char_1_left_count > char_1_right_count {
                char_1_left_count
            } else {
                char_1_right_count
            };
            Some(count_to_remove_from)
        }
        (None, Some(_previous_char_2_counts)) => None, // we don't care. would be better to panic
        (None, None) => None,                          // we don't care. would be better to panic
    };

    let count_to_add_to = match (updated_char_1_counts, updated_char_2_counts) {
        (Some(_updated_char_1_counts), Some(updated_char_2_counts)) => {
            let (char_2_left_count, char_2_right_count) = updated_char_2_counts;
            let count_to_add_to = if char_2_left_count > char_2_right_count {
                char_2_left_count
            } else {
                char_2_right_count
            };
            Some(count_to_add_to)
        }
        (Some(updated_char_1_counts), None) => {
            let (char_1_left_count, char_1_right_count) = updated_char_1_counts;
            let count_to_add_to = if char_1_left_count > char_1_right_count {
                char_1_left_count
            } else {
                char_1_right_count
            };
            Some(count_to_add_to)
        } // we don't care. would be better to panic
        (None, Some(updated_char_2_counts)) => {
            let (char_2_left_count, char_2_right_count) = updated_char_2_counts;
            let count_to_add_to = if char_2_left_count > char_2_right_count {
                char_2_left_count
            } else {
                char_2_right_count
            };
            Some(count_to_add_to)
        }
        (None, None) => {
            panic!("at least one of the counts must be a number!")
        }
    };
    (count_to_remove_from, count_to_add_to)
}

fn add_char_to_counts(
    ordered_counts_to_chars: &mut BTreeMap<i32, BTreeSet<char>>,
    count_to_add_to: Option<i32>,
    char_to_add: char,
) {
    if count_to_add_to.is_none() {
        return;
    }

    let count_to_add_to = count_to_add_to.unwrap();
    // println!(
    //     "adding {:?}, position: {:?} - {:?}",
    //     char_to_add,
    //     count_to_add_to,
    //     ordered_counts_to_chars.clone()
    // );
    let mut add_set = ordered_counts_to_chars
        .entry(count_to_add_to)
        .or_default()
        .clone();
    add_set.insert(char_to_add);
    ordered_counts_to_chars.insert(count_to_add_to, add_set);

    // println!(
    //     "added {:?}, position: {:?} - {:?}",
    //     char_to_add,
    //     count_to_add_to,
    //     ordered_counts_to_chars.clone()
    // );
}

fn remove_char_from_counts(
    ordered_counts_to_chars: &mut BTreeMap<i32, BTreeSet<char>>,
    count_to_remove_from: Option<i32>,
    character_to_remove: &char,
) {
    if count_to_remove_from.is_none() {
        return;
    }

    let count_to_remove_from = count_to_remove_from.unwrap();
    let mut remove_set = ordered_counts_to_chars
        .entry(count_to_remove_from)
        .or_default()
        .clone();

    // let before_copy = remove_set.clone();
    // println!(
    //     "removing char: {:?}, position: {:?}, before set: {:?} - {:?}",
    //     character_to_remove,
    //     count_to_remove_from,
    //     before_copy,
    //     ordered_counts_to_chars.clone()
    // );

    remove_set.remove(character_to_remove);
    // let after_copy = remove_set.clone();
    ordered_counts_to_chars.insert(count_to_remove_from, remove_set);

    // println!(
    //     "removed {:?}, position: {:?}, after set: {:?} - {:?}",
    //     character_to_remove,
    //     count_to_remove_from,
    //     after_copy,
    //     ordered_counts_to_chars.clone()
    // );
}

fn is_character_valid(character: &Option<char>) -> bool {
    if character.is_none() {
        return false;
    }
    let character = character.unwrap();
    character.is_ascii_lowercase()
}

fn update_char_mapping(
    character: &char,
    word_type: WordType,
    char_count_mapping: &mut HashMap<char, (i32, i32)>,
) -> i32 {
    let character_counts = char_count_mapping.entry(*character).or_insert((0, 0));

    // let updated_value = match word_type {
    match word_type {
        WordType::First => {
            let updated_values = (character_counts.0 + 1, character_counts.1);
            char_count_mapping.insert(*character, updated_values);
            updated_values.0
        }
        WordType::Second => {
            let updated_values = (character_counts.0, character_counts.1 + 1);
            char_count_mapping.insert(*character, updated_values);
            updated_values.1
        }
    }

    // println!("Updated char count mapping - char {:?}, count mapping: {:?}", character, char_count_mapping);
    // updated_value
}

// fn get_char_count(
//     character: &char,
//     word_type: WordType,
//     char_count_mapping: &mut HashMap<char, (i32, i32)>,
// ) -> i32 {
//     let character_counts = char_count_mapping.entry(*character).or_insert((0, 0));

//     match word_type {
//         WordType::First => {
//             character_counts.0
//         }
//         WordType::Second => {
//             character_counts.1
//         }
//     }
// }

fn get_char_both_counts(
    character: &char,
    char_count_mapping: &mut HashMap<char, (i32, i32)>,
) -> (i32, i32) {
    let character_counts = char_count_mapping.entry(*character).or_insert((0, 0));
    *character_counts
}

enum WordType {
    First,
    Second,
}

// fn random_stuff(mut a: char) {
//     let x = 'x';
//     a = 'b';
// }
