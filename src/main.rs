use std::fs;

fn main() {
    println!("Hello, world!");
    let contents = read_file("./input.txt".to_string());
    let vec_1 = parse_to_vec(&contents, 0);
    let vec_2 = parse_to_vec(&contents, 1);
    let differences = find_differences(vec_1, vec_2);
    println!("Differences: {:?}", differences);
    println!("Total: {}", calculate_total_distance(differences))
}

fn read_file(filename: String) -> String {
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

fn find_differences(mut vec_1: Vec<i32>, mut vec_2: Vec<i32>) -> Vec<i32> {
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
