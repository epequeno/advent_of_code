use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn part_one() {
    let f = File::open("input.txt").unwrap();
    let f = BufReader::new(f);
    let mut data = Vec::new();

    for line in f.lines() {
        data.push(line.unwrap());
    }
    data.sort();

    let mut group = Vec::new();
    let mut groups = Vec::new();
    for record in data {
        if group.len() == 0 && record.contains("Guard") {
            group.push(record);
        } else if group.len() > 1 && record.contains("Guard") {
            groups.push(group.clone());
            group = Vec::new();
            group.push(record);
        } else if group.len() == 1 && group[0].contains("Guard") && record.contains("Guard") {
            group = Vec::new();
            group.push(record);
        } else {
            group.push(record);
        }
    }
    groups.push(group);

    let mut guard_info: HashMap<String, Vec<String>> = HashMap::new();
    for g in groups {
        let guard_id = parse_guard_line(g.first().unwrap());
        let records = guard_info.entry(guard_id).or_insert(Vec::new());
        for r in g[1..].into_iter() {
            records.push(r.to_string());
        }
    }

    let mut guards = Vec::new();
    for (gid, records) in guard_info {
        let guard = Guard {
            id: gid.to_string(),
            minutes: mark_minutes(records),
        };
        guards.push(guard);
    }

    let mut most_sleep = 0;
    let mut guard_id = String::new();
    let mut guard_times = Vec::new();
    for g in guards {
        let mut minutes_sleep = 0;
        for i in g.minutes.clone() {
            minutes_sleep += i;
        }
        if minutes_sleep > most_sleep {
            most_sleep = minutes_sleep;
            guard_id = g.id;
            guard_times = g.minutes.clone();
        }
    }

    let mut max_min = 0;
    let mut idx = 0;
    let mut max_idx = 0;
    for min in guard_times {
        if min > max_min {
            max_min = min;
            max_idx = idx;
        }
        idx += 1;
    }
    let gid: u32 = guard_id.replace("#", "").parse().unwrap();
    println!("{}", gid * max_idx);
}

fn mark_minutes(records: Vec<String>) -> Vec<u16> {
    let mut ans = vec![0; 60];
    for i in (0..records.len()).step_by(2) {
        let sleep_start = parse_record(&records[i]);
        let sleep_end = parse_record(&records[i + 1]);
        for j in sleep_start..sleep_end {
            ans[j as usize] += 1;
        }
    }
    ans
}

#[derive(Debug)]
struct Guard {
    id: String,
    minutes: Vec<u16>,
}

fn parse_record(line: &str) -> u16 {
    let parts: Vec<&str> = line.split(" ").collect();
    let timestamp: Vec<&str> = parts[1].split(":").collect();

    timestamp[1].replace("]", "").parse().unwrap()
}

fn parse_guard_line(line: &str) -> String {
    let parts: Vec<&str> = line.split(" ").collect();
    parts[3].to_string()
}

// fn part_two() {}

fn main() {
    part_one();
}
