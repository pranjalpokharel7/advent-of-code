use crate::part_one;

fn concatenate_slices_omit(vec: &Vec<i32>, i: usize) -> Vec<i32> {
    let mut result = Vec::with_capacity(vec.len() - 1);
    result.extend_from_slice(&vec[..i]);
    result.extend_from_slice(&vec[i + 1..]);
    result
}

pub fn correct_and_calculate_safe_levels(lines: Vec<&str>) -> u32 {
    let mut safe_count: u32 = 0;

    for line in lines {
        let levels: Vec<i32> = line
            .split(' ')
            .map(|level| level.parse().expect("Error - Not a valid number: {level}"))
            .collect();

        let safety = part_one::check_level_safety(&levels);
        if safety == true {
            safe_count += 1;
            continue;
        }

        for index in 0..levels.len() {
            let modified_levels = concatenate_slices_omit(&levels, index);
            let modified_safe = part_one::check_level_safety(&modified_levels);
            if modified_safe == true {
                safe_count += 1;
                break;
            }
        }
    }

    safe_count
}