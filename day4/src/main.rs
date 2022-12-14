use std::fs;
use std::path;

const TEST_DATA: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

fn read_data() -> String {
    let path = path::Path::new("input.txt");
    return fs::read_to_string(path).unwrap();
}

fn parse_data(data: &str) -> Vec<Vec<u32>> {
    data
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                return None;
            }
            return Some(line
                .split(&[',', '-'][..])
                .map(|st| st.parse::<u32>().unwrap())
                .collect::<Vec<u32>>());
        })
        .collect::<Vec<Vec<u32>>>()
}

fn analyse_data_part1(pairs: Vec<Vec<u32>>) -> u32 {
    let mut number_contain_set = 0;
    for pair in pairs.iter() {
        if (pair[0] >= pair[2] && pair[1] <= pair[3]) 
            || (pair[2] >= pair[0] && pair[3] <= pair[1]) {
            number_contain_set += 1;
        }
    }
    return number_contain_set;
}

fn analyse_data_part2(pairs: Vec<Vec<u32>>) -> u32 {
    let mut number_overlap_set = 0;
    for pair in pairs.iter() {
        if pair[0] <= pair[3] && pair[2] <= pair[1] {
                number_overlap_set += 1;
        }
    }
    return number_overlap_set;
}

fn test_function() {
    let test_elf_pairs = parse_data(TEST_DATA);
    let test_contains = analyse_data_part1(test_elf_pairs.clone());
    let test_overlaps = analyse_data_part2(test_elf_pairs);
    println!("test data results 1: {}", test_contains);
    println!("test data results 2: {}", test_overlaps);
}

fn main() {
    test_function();

    let data = read_data();

    let elf_pairs = parse_data(&data);
    let contains = analyse_data_part1(elf_pairs.clone());
    let overlaps = analyse_data_part2(elf_pairs.clone());
    println!("data results 1: {}", contains);
    println!("data results 2: {}", overlaps);
}
