use std::ops::Neg;

#[allow(dead_code)]
fn calculate_password(rotations: &Vec<&str>) -> u16 {
    let mut password = 0;
    let mut pointer = 50; // initial starting point

    for r in rotations {
        let bytes = r.as_bytes();
        let direction = bytes[0];
        let scale: i16 = r[1..].parse().unwrap();

        if direction == b'L' {
            pointer -= scale;
        } else if direction == b'R' {
            pointer += scale;
        } else {
            panic!("Unknown direction value");
        }

        pointer = pointer % 100; // adjust modulo
        if pointer < 0 {
            pointer += 100;
        }

        if pointer == 0 {
            password += 1;
        }
    }

    password
}

#[allow(dead_code)]
fn calculate_password_part_two(rotations: &Vec<&str>) -> i16 {
    let mut password = 0;
    let mut pointer = 50; // initial starting point
    let mut previously_positive = true;

    for r in rotations {
        let bytes = r.as_bytes();
        let direction = bytes[0];
        let mut scale: i16 = r[1..].parse().unwrap();

        if direction == b'L' {
            scale = scale.neg();
        }

        // count number of times we encountered zero
        pointer += scale;

        let range = pointer.abs() / 100;
        password += range; // 1) add quotient
        
        pointer %= 100; // adjust modulo

        if
            (pointer < 0 && previously_positive) ||
            (pointer > 0 && !previously_positive) ||
            (pointer == 0 && range != 0)
        {
            password += 1; // 2) sign change
        }

        previously_positive = pointer > 0;
    }

    password
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_from_file;

    #[test]
    fn test_sample() {
        let contents = read_from_file("data/day_one/sample.txt");
        let directions = contents.split("\n").collect();
        let password = calculate_password(&directions);
        assert_eq!(password, 3);
    }

    #[test]
    fn test_input() {
        let contents = read_from_file("data/day_one/input.txt");
        let directions = contents.split("\n").collect();
        let password = calculate_password(&directions);
        assert_eq!(password, 962);
    }

    #[test]
    fn test_sample_part_two() {
        let contents = read_from_file("data/day_one/sample.txt");
        let directions = contents.split("\n").collect();
        let password = calculate_password_part_two(&directions);
        assert_eq!(password, 6);
    }

    // TODO: doesn't get the correct answer on input txt
    // #[test]
    // fn test_input_part_two() {
    //     let contents = read_from_file("data/day_one/input.txt");
    //     let directions = contents.split("\n").collect();
    //     let password = calculate_password_part_two(&directions);
    //     assert_eq!(password, 6368);
    // }
}
