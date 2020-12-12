use anyhow::{anyhow, Context, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

const INPUT_FILE: &str = "./input/dec-07-part-01/input.txt";

lazy_static! {
    static ref RE_CONTAINING_BAG: Regex = Regex::new(r"(?P<color>[\w ]+) bags contain").unwrap();
    static ref RE_CONTAINED_BAG: Regex =
        Regex::new(r"(?P<amount>\d+) (?P<color>[\w ]+) bag").unwrap();
}

type BagMap = HashMap<String, Vec<(usize, String)>>;

fn main() -> Result<()> {
    let bags = read_file_to_bag_map(INPUT_FILE).context("Could not parse file")?;

    let ultimate_parent_bags = get_ultimate_parent_bags(&bags, "shiny gold")?;

    println!(
        "The following {} bag(s) eventually contain a \"shiny gold\" bag:\n\t{}",
        ultimate_parent_bags.len(),
        ultimate_parent_bags
            .iter()
            .cloned()
            .collect::<Vec<_>>()
            .join(", "),
    );

    Ok(())
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
                bags.entry(containing_bag_color.as_str().to_string())
                    .or_insert(vec![]);

                if !line.contains("contain no other bags") {
                    // this bag can contain other bags!
                    // loop through them, add them to the hashmap if not already there.
                    // whether they're there or not, add the containing bag to the list of containing
                    // bags.
                    for contained_bag_caps in RE_CONTAINED_BAG.captures_iter(line) {
                        let containing_bag_entry = (
                            contained_bag_caps["amount"].parse::<usize>()?,
                            containing_bag_color.as_str().to_string(),
                        );

                        bags.entry(contained_bag_caps["color"].to_string())
                            .and_modify(|bs| bs.push(containing_bag_entry.clone()))
                            .or_insert_with(|| vec![containing_bag_entry]);
                    }
                }
            }
        }
    }

    Ok(bags)
}

fn get_ultimate_parent_bags(bags: &BagMap, child_bag: &str) -> Result<HashSet<String>> {
    fn get_ultimate_parent_bags_inner(bags: &BagMap, cur_bag: &str) -> Result<HashSet<String>> {
        if let Some(parent_bags) = bags.get(cur_bag) {
            let mut result_set = HashSet::new();

            if !parent_bags.is_empty() {
                // This bag is contained by other bags. Loop through them, and recursively
                // call this function with each parent bag as input. Assemble the results
                // into a big set.
                for (_amount, parent_bag_color) in parent_bags {
                    result_set.extend(get_ultimate_parent_bags_inner(bags, parent_bag_color)?);
                }
            }

            result_set.insert(cur_bag.to_string());

            Ok(result_set)
        } else {
            Err(anyhow!("Could not find bag \"{}\" in the bag map", cur_bag))
        }
    };

    let mut result_set = get_ultimate_parent_bags_inner(bags, child_bag)?;

    result_set.remove(child_bag);

    Ok(result_set)
}
