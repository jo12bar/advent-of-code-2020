use anyhow::{Context, Result};
use std::collections::HashSet;
use std::fs;

const INPUT_FILE: &str = "./input/dec-06-part-01/input.txt";

fn main() -> Result<()> {
    let input = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Could not open and read file: {}", INPUT_FILE))?;

    let mut answered_questions = HashSet::new();
    let mut num_answered_questions_per_group = Vec::new();

    for group in input.split("\n\n") {
        for answered_question in group.trim().replace('\n', "").chars() {
            answered_questions.insert(answered_question);
        }

        num_answered_questions_per_group.push(answered_questions.len());
        answered_questions.clear();
    }

    println!(
        "Answer: {}",
        num_answered_questions_per_group.iter().sum::<usize>()
    );

    Ok(())
}
