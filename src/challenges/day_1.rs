use std::collections::HashMap;

pub fn solve(input: String) {
    let vec_1 = parse_to_vec(&input, 0);
    let vec_2 = parse_to_vec(&input, 1);
    let (differences, total) = solve_part_1(&mut vec_1.clone(), &mut vec_2.clone());
    println!("Differences: {:?}", differences);
    println!("Total: {}", total);

    let (similarities, total) = solve_part_2(&mut vec_1.clone(), &mut vec_2.clone());
    println!("Similarities: {:?}", similarities);
    println!("Total: {}", total);
}

fn solve_part_1(vec_1: &mut Vec<i32>, vec_2: &mut Vec<i32>) -> (Vec<i32>, i32) {
    let differences = find_differences(vec_1, vec_2);
    let total = calculate_total_distance(&differences);

    (differences, total)
}

fn solve_part_2(vec_1: &mut Vec<i32>, vec_2: &mut Vec<i32>) -> (Vec<i32>, i32) {
    let map_of_location_id = map_from_vec(vec_2);
    let similarities = calculate_similarity(&vec_1, &map_of_location_id);
    let total = calculate_total_distance(&similarities);

    (similarities, total)
}

fn parse_to_vec(contents: &str, column: usize) -> Vec<i32> {
    let lines = contents.split("\n");
    let mut numbers = Vec::new();

    for line in lines {
        if line.is_empty() {
            continue;
        }
        let num: Vec<&str> = line.split_whitespace().collect();
        let num: i32 = num[column].parse().unwrap();

        numbers.push(num);
    }
    numbers
}

fn find_differences(vec_1: &mut Vec<i32>, vec_2: &mut Vec<i32>) -> Vec<i32> {
    let mut differences = Vec::new();
    vec_1.sort();
    vec_2.sort();

    for index in 0..vec_1.len() {
        if vec_1[index] >= vec_2[index] {
            differences.push(vec_1[index] - vec_2[index]);
        } else {
            differences.push(vec_2[index] - vec_1[index]);
        }
    }
    differences
}

fn calculate_total_distance(differences: &Vec<i32>) -> i32 {
    let mut total_difference = 0;
    for difference in differences {
        total_difference = total_difference + difference;
    }

    total_difference
}

fn map_from_vec(values: &Vec<i32>) -> HashMap<i32, i32> {
    let mut occurances: HashMap<i32, i32> = HashMap::new();
    for value in values {
        if occurances.contains_key(&value) {
            let count: &i32 = occurances.get(&value).expect("Should be a valid value");
            let count: i32 = count.clone() + 1;
            occurances.insert(value.clone(), count);
        } else {
            occurances.insert(value.clone(), 1);
        }
    }
    occurances
}

fn calculate_similarity(values: &Vec<i32>, occurances: &HashMap<i32, i32>) -> Vec<i32> {
    let mut similarities = Vec::new();
    for value in values {
        if occurances.contains_key(&value) {
            let similarity = value * occurances.get(&value).expect("Should contain a value");
            similarities.push(similarity);
        }
    }
    similarities
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "3   4
                4   3
                2   5
                1   3
                3   9
                3   3";

    #[test]
    fn part_1_test() {
        let mut vec_1 = parse_to_vec(&INPUT, 0);
        let mut vec_2 = parse_to_vec(&INPUT, 1);
        let (differences, total) = solve_part_1(&mut vec_1, &mut vec_2);
        let expected_diff = vec![2, 1, 0, 1, 2, 5];
        assert_eq!(expected_diff, differences);
        assert_eq!(11, total);
    }

    #[test]
    fn part_2_test() {
        let mut vec_1 = parse_to_vec(&INPUT, 0);
        let mut vec_2 = parse_to_vec(&INPUT, 1);
        let (differences, total) = solve_part_2(&mut vec_1, &mut vec_2);
        let expected_similarity = vec![9, 4, 9, 9];
        assert_eq!(expected_similarity, differences);
        assert_eq!(31, total);
    }
}
