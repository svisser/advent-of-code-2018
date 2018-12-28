use std::collections::hash_map::HashMap;
use std::io;

#[derive(Debug)]
struct IdentifierAnalysis {
    has_two: bool,
    has_three: bool,
}

fn process_box_id(box_id: &str) -> IdentifierAnalysis {
    let mut has_two: bool = false;
    let mut has_three: bool = false;
    let mut counts: HashMap<char, u32> = HashMap::new();
    for c in box_id.chars() {
        let count = counts.entry(c).or_insert(0);
        *count += 1;
    }
    for value in counts.values() {
        if *value == 2 {
            has_two = true;
        }
        if *value == 3 {
            has_three = true;
        }
    }
    IdentifierAnalysis { has_two, has_three }
}

fn calculate_checksum(analyses: Vec<IdentifierAnalysis>) -> u32 {
    let mut two_count: u16 = 0;
    let mut three_count: u16 = 0;
    for analysis in analyses {
        if analysis.has_two {
            two_count += 1;
        }
        if analysis.has_three {
            three_count += 1;
        }
    }
    u32::from(two_count * three_count)
}

// abcdef,bababc,abbcde,abcccd,aabcdd,abcdee,ababab

fn main() {
    println!("Please provide comma-separated list of box IDs: ");
    let mut input: String = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let analyses: Vec<IdentifierAnalysis> = input
                .split(',')
                .map(|box_id| process_box_id(box_id.trim()))
                .collect();
            println!("Checksum: {}", calculate_checksum(analyses));
        }
        Err(error) => println!("Unable to parse input: {}", error),
    };
}
