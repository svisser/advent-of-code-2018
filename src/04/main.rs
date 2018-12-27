#[macro_use(lazy_static)]
extern crate lazy_static;
extern crate regex;

use std::collections::hash_map::HashMap;
use std::env;
use std::fs;
use std::process;

use regex::Regex;

type Minute = u8;
type GuardIdentifier = u32;
type GuardSleepingPeriods = Vec<GuardSleepPeriod>;
type GuardSleepingSummary = HashMap<GuardIdentifier, GuardSleepingPeriods>;

static LOG_LINE_FALLS_ASLEEP: &'static str = "falls asleep";
static LOG_LINE_WAKES_UP: &'static str = "wakes up";

#[derive(Debug)]
struct GuardSleepPeriod {
    start_minute: Minute,
    end_minute: Minute,
}

impl GuardSleepPeriod {
    fn duration(&self) -> u8 {
        self.end_minute - self.start_minute
    }
}

#[derive(Debug)]
struct GuardBehaviour {
    guard_identifier: GuardIdentifier,
    minutes_asleep: u32,
    most_frequent_minute_asleep: Minute,
}

impl GuardBehaviour {
    fn solution(&self) -> u32 {
        self.guard_identifier * (self.most_frequent_minute_asleep as u32)
    }
}

fn parse_to_guard_sleeps(file_contents: String) -> GuardSleepingSummary {
    // Sorting input lines has not been implemented
    lazy_static! {
        static ref GUARD_LOG_LINE_REGEX: Regex =
            Regex::new(r"^\[(?P<year>\d{4})\-(?P<month>\d{2})\-(?P<day>\d{2}) (?P<hour>\d{2}):(?P<minute>\d{2})\] (?P<log_line>.*)$")
                .unwrap();
        static ref GUARD_BEGINS_REGEX: Regex = Regex::new(
            r"Guard #(?P<guard_identifier>\d+) begins shift"
        ).unwrap();
    }
    let mut guard_sleeps: HashMap<GuardIdentifier, GuardSleepingPeriods> = HashMap::new();
    let mut current_guard_id: GuardIdentifier = 0;
    let mut current_guard_asleep_minute: u8 = 0;
    for line in file_contents.lines() {
        match GUARD_LOG_LINE_REGEX.captures(line) {
            Some(caps) => {
                let log_line = caps.name("log_line").unwrap().as_str();
                match GUARD_BEGINS_REGEX.captures(log_line) {
                    Some(begins_caps) => {
                        current_guard_id = begins_caps
                            .name("guard_identifier")
                            .unwrap()
                            .as_str()
                            .parse::<GuardIdentifier>()
                            .unwrap();
                    }
                    _ => (),
                }
                let log_line_minute = caps.name("minute").unwrap().as_str().parse::<u8>().unwrap();
                if log_line == LOG_LINE_FALLS_ASLEEP {
                    current_guard_asleep_minute = log_line_minute;
                } else if log_line == LOG_LINE_WAKES_UP {
                    let e: &mut Vec<GuardSleepPeriod> = guard_sleeps
                        .entry(current_guard_id)
                        .or_insert_with(|| Vec::new());
                    let period = GuardSleepPeriod {
                        start_minute: current_guard_asleep_minute,
                        end_minute: log_line_minute,
                    };
                    e.push(period);
                }
            }
            _ => (),
        };
    }
    guard_sleeps
}

fn calc_minutes_asleep(sleeps: &GuardSleepingPeriods) -> u32 {
    sleeps.iter().map(|s| s.duration() as u32).sum()
}

fn calc_most_frequent_minute(sleeps: &GuardSleepingPeriods) -> Minute {
    let mut minutes_in_hour: HashMap<Minute, u32> = HashMap::new();
    for sleep in sleeps.iter() {
        let mut k = sleep.start_minute;
        while k < sleep.end_minute {
            let minute_count = minutes_in_hour.entry(k).or_insert(0);
            *minute_count += 1;
            k += 1;
        }
    }
    let mut selected_minute: Minute = 0;
    let mut max_minute_count: u32 = u32::min_value();
    for (minute, minute_count) in minutes_in_hour.iter() {
        if *minute_count > max_minute_count {
            max_minute_count = *minute_count;
            selected_minute = *minute;
        }
    }
    selected_minute
}

fn convert_sleeps_to_behaviours(guard_sleeps: GuardSleepingSummary) -> Vec<GuardBehaviour> {
    let mut guard_behaviours: Vec<GuardBehaviour> = Vec::new();
    for (guard_identifier, sleeps_per_guard) in guard_sleeps.iter() {
        let minutes_asleep: u32 = calc_minutes_asleep(&sleeps_per_guard);
        let most_frequent_minute_asleep: Minute = calc_most_frequent_minute(&sleeps_per_guard);
        let guard_behaviour = GuardBehaviour {
            guard_identifier: *guard_identifier,
            minutes_asleep: minutes_asleep,
            most_frequent_minute_asleep: most_frequent_minute_asleep,
        };
        guard_behaviours.push(guard_behaviour);
    }
    guard_behaviours
}

fn calculate_solution(guard_behaviours: Vec<GuardBehaviour>) -> u32 {
    guard_behaviours
        .iter()
        .max_by_key(|behaviour| behaviour.minutes_asleep)
        .map(|behaviour| behaviour.solution())
        .unwrap_or(0)
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
    let guard_sleeps = parse_to_guard_sleeps(file_contents);
    let guard_behaviours = convert_sleeps_to_behaviours(guard_sleeps);
    let solution: u32 = calculate_solution(guard_behaviours);
    println!("Solution: {}", solution);
}
