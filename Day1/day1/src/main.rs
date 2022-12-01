use std::fs;

fn main() {
    let file_path = String::from("/home/justine/Sandbox/Python/AdventOfCode/Day1/day1/input.txt");
    let contents = fs::read_to_string(file_path)
        .expect("Could not read the file, bro");

    let splitted = contents.trim().split("\n\n");


    let mut collection = Vec::new();

    for s in splitted {
        let sub_splitted = s.split("\n");
        let mut current_elf: i32 = 0;
        for ss in sub_splitted {
            let food: i32 = ss.parse().unwrap();
            current_elf = current_elf + food;
        }
        collection.push(current_elf);
    }

    collection.sort();
    collection.reverse();

    let total: i32 = collection[0..=2].iter().sum();

    println!("{total}");




}
