use std::io;
use std::str;

fn main() {
    let mut final_frequency: i64 = 0;
    let mut changes_applied: u64 = 0;
    let mut changes_skipped: u64 = 0;
    println!("Please provide a comma-separated sequence of frequency changes: ");
    let mut input: String = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let frequency_changes: str::Split<&str> = input.split(",");
            for change in frequency_changes {
                let trimmed_change: &str = change.trim();
                if trimmed_change.len() < 2 {
                    continue;
                }
                let operator: &str = &trimmed_change[..1];
                let number: &str = &trimmed_change[1..];
                let numeric_change: Result<i64, std::num::ParseIntError> = number.parse();
                match numeric_change {
                    Ok(n) => {
                        if operator == "+" {
                            final_frequency += n;
                            changes_applied += 1;
                        } else if operator == "-" {
                            final_frequency -= n;
                            changes_applied += 1;
                        } else {
                            changes_skipped += 1;
                        }
                    }
                    Err(_) => {
                        changes_skipped += 1;
                    }
                }
            }
            println!("Final frequency: {}", final_frequency);
            println!("Frequency changes applied: {}", changes_applied);
            println!("Frequency changes skipped: {}", changes_skipped);
        }
        Err(error) => println!("Unable to parse input: {}", error),
    };
}
