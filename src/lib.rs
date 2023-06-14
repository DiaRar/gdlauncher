use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
// Everything could be wrapped in this to hide the *magic*
// pub struct Wrapper {
//     cursor: Lines<BufReader<File>>,
//     values: VecDeque<u128>,
//     values_sorted: BTreeMap<u128, u8>,
// }
#[derive(Debug)]
pub struct Number {
    _line: usize,
    value: u128,
}
#[derive(PartialEq)]
enum MineSafety {
    Safe,
    WillCrumble,
}
// Adding a new function in case of future testing, maybe will add a WrapperBuilder,
// but I should probably find a better name for the struct.
pub fn read_first_k(
    reader: BufReader<File>,
    number_of_elements: u32,
) -> (Lines<BufReader<File>>, VecDeque<u128>, BTreeMap<u128, u8>) {
    let mut values_sorted = BTreeMap::new();
    let mut values = VecDeque::with_capacity(100);
    let mut cursor = reader.lines();
    let mut index = 0;
    // Parsing the first 100 lines and appending them to both the sorted and unsorted versions
    for line in cursor.by_ref() {
        match line {
            Ok(line) => {
                let value = line.parse::<u128>().expect("Not a number!");

                let frequency = values_sorted.get(&value).unwrap_or(&0);
                values_sorted.insert(value, frequency + 1);

                values.push_back(value);
                index += 1;
            }
            Err(e) => {
                println!("Line Error: {}", e)
            }
        }
        if index == number_of_elements {
            break;
        }
    }

    (cursor, values, values_sorted)
}

fn crumble_check(number: &Number, values_sorted: &BTreeMap<u128, u8>) -> MineSafety {
    let mut iter = values_sorted.iter();
    let mut rev_iter = values_sorted.into_iter().rev();
    let mut low_element = iter.next().unwrap();
    let mut high_element = rev_iter.next().unwrap();
    while low_element.0 + high_element.0 != number.value {
        if low_element == high_element && low_element.1 == &(1 as u8) {
            return MineSafety::WillCrumble;
        }
        // Getting next high_element
        if low_element.0 + high_element.0 > number.value {
            high_element = match rev_iter.next() {
                Some(val) => val,
                None => {
                    return MineSafety::WillCrumble;
                }
            };
        }
        // Getting next low_element
        if low_element.0 + high_element.0 < number.value {
            low_element = match iter.next() {
                Some(val) => val,
                None => return MineSafety::WillCrumble,
            };
        }
        if low_element.0 > high_element.0 {
            return MineSafety::WillCrumble;
        }
    }
    MineSafety::Safe
}

// Making a crumbling list to see what numbers are at fault and what their lines are!
// The reason we use a BTreeMap<u128, u8> is to keep a sorted frequency list.
// In the u8 value we have the number of element aparitions.

pub fn crumbling_list(
    cursor: Lines<BufReader<File>>,
    values: &mut VecDeque<u128>,
    values_sorted: &mut BTreeMap<u128, u8>,
) -> Option<Vec<Number>> {
    let mut crumbling_list: Vec<Number> = Vec::new();
    let len = values.len();
    for (i, line) in cursor.enumerate() {
        let value = match line {
            Ok(line) => line.parse::<u128>().expect("Not a number!"),
            Err(_) => 0,
        };
        let number = Number {
            _line: i + len + 1,
            value,
        };
        // Initially, I thought that if a value would crumble, I would not add it to the previous 100 list.
        // After debugging I saw that one number I hadn't added because it crumbled,
        // made it crumble the whole list afterwards.
        if crumble_check(&number, &values_sorted) == MineSafety::WillCrumble {
            crumbling_list.push(number);
        }
        let removed = match values.pop_front() {
            Some(val) => val,
            None => panic!("Deque is empty"),
        };
        values.push_back(value);
        let removed_frequency = values_sorted.get(&removed)?;
        if *removed_frequency == 1 {
            values_sorted.remove(&removed);
        } else {
            values_sorted.insert(removed, removed_frequency - 1);
        }
        let frequency = values_sorted.get(&value).unwrap_or(&0);
        values_sorted.insert(value, frequency + 1);
    }
    match crumbling_list.is_empty() {
        true => None,
        false => Some(crumbling_list),
    }
}
