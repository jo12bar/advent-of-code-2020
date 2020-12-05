use anyhow::Result;
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::fs;

#[derive(Debug, Eq, PartialEq)]
struct Passport<'a> {
    /// Birth year (`byr`)
    birth_year: Option<&'a str>,
    /// Issue year (`iyr`)
    issue_year: Option<&'a str>,
    /// Expiration year (`eyr`)
    expiration_year: Option<&'a str>,
    /// Height (`hgt`)
    height: Option<&'a str>,
    /// Hair color (`hcl`)
    hair_color: Option<&'a str>,
    /// Eye color (`ecl`)
    eye_color: Option<&'a str>,
    /// Passport ID (`pid`)
    passport_id: Option<&'a str>,
    /// Country ID (`cid`)
    country_id: Option<&'a str>,
}

lazy_static! {
    static ref RE_BIRTH_YEAR: Regex = Regex::new(r"byr:(\S+)").unwrap();
    static ref RE_ISSUE_YEAR: Regex = Regex::new(r"iyr:(\S+)").unwrap();
    static ref RE_EXPIRATION_YEAR: Regex = Regex::new(r"eyr:(\S+)").unwrap();
    static ref RE_HEIGHT: Regex = Regex::new(r"hgt:(\S+)").unwrap();
    static ref RE_HAIR_COLOR: Regex = Regex::new(r"hcl:(\S+)").unwrap();
    static ref RE_EYE_COLOR: Regex = Regex::new(r"ecl:(\S+)").unwrap();
    static ref RE_PASSPORT_ID: Regex = Regex::new(r"pid:(\S+)").unwrap();
    static ref RE_COUNTRY_ID: Regex = Regex::new(r"cid:(\S+)").unwrap();
}

impl<'a> Passport<'a> {
    fn parse(passport_string: &'a str) -> Self {
        fn map_to_maybe_cap_1(maybe_caps: Option<Captures>) -> Option<&str> {
            maybe_caps
                .and_then(|caps| caps.get(1))
                .map(|mat| mat.as_str())
        }

        Self {
            birth_year: map_to_maybe_cap_1(RE_BIRTH_YEAR.captures(passport_string)),
            issue_year: map_to_maybe_cap_1(RE_ISSUE_YEAR.captures(passport_string)),
            expiration_year: map_to_maybe_cap_1(RE_EXPIRATION_YEAR.captures(passport_string)),
            height: map_to_maybe_cap_1(RE_HEIGHT.captures(passport_string)),
            hair_color: map_to_maybe_cap_1(RE_HAIR_COLOR.captures(passport_string)),
            eye_color: map_to_maybe_cap_1(RE_EYE_COLOR.captures(passport_string)),
            passport_id: map_to_maybe_cap_1(RE_PASSPORT_ID.captures(passport_string)),
            country_id: map_to_maybe_cap_1(RE_COUNTRY_ID.captures(passport_string)),
        }
    }

    /// If all fields *except for country id` are present, the passport is valid.
    fn is_valid(&'a self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.passport_id.is_some()
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("./input/dec-04-part-01/input.txt")?;

    let num_valid = input
        .split("\n\n")
        .map(Passport::parse)
        .filter(|p| p.is_valid())
        .count();

    println!("Num valid passports: {}", num_valid);

    Ok(())
}
