use gdlauncher::{crumbling_list, read_first_k};
use std::fs::File;
use std::io::BufReader;

fn main() {
    let file = File::open("challenge_input.txt").expect("File not found");
    // In case file size is known, use:
    // let reader = BufReader::with_capacity(file_size, file);
    // Otherwise use:
    // let reader = BufReader::new(file);
    let reader = BufReader::with_capacity(200_000, file);
    let (cursor, mut values, mut values_sorted) = read_first_k(reader, 100);
    let crumbling = crumbling_list(cursor, &mut values, &mut values_sorted);
    match crumbling {
        Some(list) => println!("{:#?}", list.iter().collect::<Vec<_>>()),
        None => println!("Everything is safe!"),
    };
}
