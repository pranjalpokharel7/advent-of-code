use std::fs;

mod part_one;
mod part_two;


fn main() {
    let file_path = "data/input.txt";
    let contents = fs::read_to_string(file_path).expect("Unable to read file");

    let lines: Vec<&str> = contents.split('\n').collect();
    let safe_count = part_two::correct_and_calculate_safe_levels(lines);
    println!("{:?}", safe_count);
}
