use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;

const INPUT_FILE: &str = "./input/dec-06-part-01/input.txt";

fn main() -> Result<()> {
    let input = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Could not open and read file: {}", INPUT_FILE))?;

    let mut answered_questions = HashMap::<char, usize>::new();
    let mut num_answered_questions_per_group = Vec::new();

    for group in input.split("\n\n") {
        let group = group.trim();
        let group_count = group.split('\n').count();

        for answered_question in group.replace('\n', "").chars() {
            answered_questions
                .entry(answered_question)
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }

        let count_questions_all_answered = answered_questions
            .iter()
            .filter(|(_ans, ans_count)| **ans_count == group_count)
            .count();

        num_answered_questions_per_group.push(count_questions_all_answered);
        answered_questions.clear();
    }

    println!(
        "Answer: {}",
        num_answered_questions_per_group.iter().sum::<usize>()
    );

    Ok(())
}
