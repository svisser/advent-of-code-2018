#[macro_use(lazy_static)]
extern crate lazy_static;
extern crate regex;

use std::collections::HashMap;
use std::env;
use std::fs;
use std::process;

use regex::Regex;

type Square = (u32, u32);

#[derive(Debug)]
struct Claim {
    id: u64,
    left_offset: u32,
    top_offset: u32,
    width: u32,
    height: u32,
}

fn parse_to_claims(file_contents: String) -> Vec<Claim> {
    let mut claims: Vec<Claim> = Vec::new();
    lazy_static! {
        static ref CLAIM_LINE_REGEX: Regex = Regex::new(
            r"^#(?P<identifier>\d+) @ (?P<left_offset>\d+),(?P<top_offset>\d+): (?P<width>\d+)x(?P<height>\d+)$"
        ).unwrap();
    }
    for line in file_contents.lines() {
        match CLAIM_LINE_REGEX.captures(line) {
            Some(caps) => {
                let id: u64 = caps
                    .name("identifier")
                    .unwrap()
                    .as_str()
                    .parse::<u64>()
                    .unwrap();
                let left_offset: u32 = caps
                    .name("left_offset")
                    .unwrap()
                    .as_str()
                    .parse::<u32>()
                    .unwrap();
                let top_offset: u32 = caps
                    .name("top_offset")
                    .unwrap()
                    .as_str()
                    .parse::<u32>()
                    .unwrap();
                let width: u32 = caps.name("width").unwrap().as_str().parse::<u32>().unwrap();
                let height: u32 = caps
                    .name("height")
                    .unwrap()
                    .as_str()
                    .parse::<u32>()
                    .unwrap();
                let claim = Claim {
                    id,
                    left_offset,
                    top_offset,
                    width,
                    height,
                };
                claims.push(claim);
            }
            _ => (),
        };
    }
    claims
}

fn calculate_overlapping_squares(claims: &Vec<Claim>) -> u64 {
    let mut fabric_usage: HashMap<Square, u64> = HashMap::new();
    let mut overlapping_squares: u64 = 0;
    for claim in claims.iter() {
        for x in claim.left_offset..(claim.left_offset + claim.width) {
            for y in claim.top_offset..(claim.top_offset + claim.height) {
                let square: Square = (x, y);
                let count = fabric_usage.entry(square).or_insert(0);
                *count += 1;
            }
        }
    }
    for square_usage in fabric_usage.values() {
        if *square_usage > 1 {
            overlapping_squares += 1;
        }
    }
    overlapping_squares
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("One argument is expected: filepath");
        process::exit(1);
    }
    let filepath: &str = &args[1];
    let file_contents: String =
        fs::read_to_string(filepath).expect("Something went wrong when reading the file");
    let claims: Vec<Claim> = parse_to_claims(file_contents);
    let overlapping_squares: u64 = calculate_overlapping_squares(&claims);
    println!("Number of overlapping squares: {}", overlapping_squares);
}
