use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let contents = read_file(path);
    let mut vec_1 = parse_to_vec(&contents, 0);
    let mut vec_2 = parse_to_vec(&contents, 1);
    let differences = find_differences(&mut vec_1, &mut vec_2);
    println!("Differences: {:?}", differences);
    println!("Total: {}", calculate_total_distance(differences));

    let map_of_location_id = map_from_vec(vec_2);
    let similarities = calculate_similarity(&vec_1, &map_of_location_id);

    println!("Similarities: {:?}", similarities);
    println!("Total: {}", calculate_total_distance(similarities));
}

fn read_file(filename: &str) -> String {
    let contents =
        fs::read_to_string(filename).expect("Should have been able to read the input file");
    contents
}

fn parse_to_vec(contents: &str, column: usize) -> Vec<i32> {
    let lines = contents.split("\n");

    let mut numbers = Vec::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let nums: Vec<&str> = line.split_whitespace().collect();
        let num: i32 = nums[column].parse().unwrap();

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

fn calculate_total_distance(differences: Vec<i32>) -> i32 {
    let mut total_difference = 0;
    for difference in differences {
        total_difference = total_difference + difference;
    }

    total_difference
}

fn map_from_vec(values: Vec<i32>) -> HashMap<i32, i32> {
    let mut occurances: HashMap<i32, i32> = HashMap::new();
    for value in values {
        if occurances.contains_key(&value) {
            let count: &i32 = occurances.get(&value).expect("Should be a valid value");
            let new_count: i32 = count + 1;
            occurances.insert(value, new_count);
        } else {
            occurances.insert(value, 1);
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
