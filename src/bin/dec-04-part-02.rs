use anyhow::Result;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Eq, PartialEq)]
struct Passport {
    /// Birth year (`byr`)
    birth_year: Option<u64>,
    /// Issue year (`iyr`)
    issue_year: Option<u64>,
    /// Expiration year (`eyr`)
    expiration_year: Option<u64>,
    /// Height (`hgt`)
    height: Option<String>,
    /// Hair color (`hcl`)
    hair_color: Option<String>,
    /// Eye color (`ecl`)
    eye_color: Option<String>,
    /// Passport ID (`pid`)
    passport_id: Option<u64>,
    /// Country ID (`cid`)
    country_id: Option<String>,
}

impl Passport {
    fn parse(passport_string: String) -> Self {
        let passport_string = passport_string.trim().replace('\n', " ");
        let mut keyvals = passport_string
            .split(' ')
            .map(|s| s.trim().split(':').collect::<Vec<_>>())
            .map(|v| (v[0], v[1].to_string()))
            .collect::<HashMap<_, _>>();

        Self {
            birth_year: keyvals
                .remove("byr")
                .filter(|s| s.len() == 4)
                .and_then(|s| s.parse().ok())
                .filter(|n| (1920..=2002).contains(n)),

            issue_year: keyvals
                .remove("iyr")
                .filter(|s| s.len() == 4)
                .and_then(|s| s.parse().ok())
                .filter(|n| (2010..=2020).contains(n)),

            expiration_year: keyvals
                .remove("eyr")
                .filter(|s| s.len() == 4)
                .and_then(|s| s.parse().ok())
                .filter(|n| (2020..=2030).contains(n)),

            height: keyvals.remove("hgt").filter(|s| {
                let num = s[..s.len() - 2].parse::<u64>();
                let units = &s[s.len() - 2..];

                if let Ok(num) = num {
                    return (units == "cm" && (150..=193).contains(&num))
                        || (units == "in" && (59..=76).contains(&num));
                }

                false
            }),

            hair_color: keyvals.remove("hcl").filter(|s| {
                let chars = s.chars().collect::<Vec<_>>();

                chars.len() == 7
                    && chars[0] == '#'
                    && chars
                        .iter()
                        .skip(1)
                        .all(|c| (*c >= '0' && *c <= '9') || (*c >= 'a' && *c <= 'f'))
            }),

            eye_color: keyvals.remove("ecl").filter(|s| {
                s == "amb"
                    || s == "blu"
                    || s == "brn"
                    || s == "gry"
                    || s == "grn"
                    || s == "hzl"
                    || s == "oth"
            }),

            passport_id: keyvals
                .remove("pid")
                .filter(|s| s.len() == 9)
                .and_then(|s| s.parse().ok()),

            country_id: keyvals.remove("cid"),
        }
    }

    /// If all fields *except for country id` are present, the passport is valid.
    fn is_valid(&self) -> bool {
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
        .map(|s| s.to_string())
        .map(Passport::parse)
        .filter(|p| p.is_valid())
        .count();

    println!("Num valid passports: {}", num_valid);

    Ok(())
}
