use anyhow::{anyhow, Context, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

const INPUT_FILE: &str = "./input/dec-07-part-01/input.txt";

type BagMap = HashMap<String, Vec<(usize, String)>>;

fn main() -> Result<()> {
    let bags = read_file_to_bag_map(INPUT_FILE).context("Could not parse file")?;

    let mut sum = 0;
    let mut queue = bags.get("shiny gold").map_or_else(VecDeque::new, |items| {
        items
            .iter()
            .map(|(count, item)| (*count, item.as_str()))
            .collect()
    });

    while let Some((count, item)) = queue.pop_front() {
        sum += count;

        if let Some(items) = bags.get(item) {
            for (subcount, subitem) in items {
                queue.push_back((count * subcount, subitem));
            }
        }
    }

    println!("Answer: {:?}", sum);

    Ok(())
}

lazy_static! {
    static ref RE_CONTAINING_BAG: Regex =
        Regex::new(r"(?P<color>[\w ]+) bags contain (.*)").unwrap();
    static ref RE_CONTAINED_BAG: Regex =
        Regex::new(r"(?P<amount>\d+) (?P<color>[\w ]+) bag").unwrap();
}

fn read_file_to_bag_map<P: AsRef<Path> + Display + Clone>(file_path: P) -> Result<BagMap> {
    let input = File::open(file_path.clone())
        .with_context(|| format!("Could not open file: {}", file_path))?;
    let input_reader = BufReader::new(input);

    let mut bags: BagMap = HashMap::new();

    for line in input_reader.lines() {
        let line = line.with_context(|| format!("Could not read line from file: {}", file_path))?;
        let line = line.trim();

        if let Some(containing_bag_caps) = RE_CONTAINING_BAG.captures(line) {
            if let Some(containing_bag_color) = containing_bag_caps.name("color") {
                if !line.contains("contain no other bags") {
                    bags.insert(
                        containing_bag_color.as_str().to_string(),
                        RE_CONTAINED_BAG
                            .captures_iter(line)
                            .filter_map(|caps| {
                                Some((
                                    caps.name("amount")?.as_str().parse().ok()?,
                                    caps.name("color")?.as_str().to_string(),
                                ))
                            })
                            .collect(),
                    );
                }
            }
        }
    }

    Ok(bags)
}
