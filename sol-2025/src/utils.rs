use std::fs;

pub fn read_from_file(path: &str) -> String {
    fs::read_to_string(path).expect(format!("Unable to read file: {}", path).as_str())
}
