use anyhow::Result;
use std::convert::From;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct LoopVec<T>(Vec<T>);

impl<T> From<Vec<T>> for LoopVec<T> {
    fn from(vec: Vec<T>) -> Self {
        Self(vec)
    }
}

impl<T> std::ops::Index<usize> for LoopVec<T> {
    type Output = T;

    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i % self.0.len()]
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum TileType {
    Open,
    Tree,
}

fn main() -> Result<()> {
    let input_file = File::open("./input/dec-03-part-01/input.txt")?;
    let reader = BufReader::new(input_file);

    let mut slope: Vec<LoopVec<TileType>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        let mut line_tiles = Vec::with_capacity(line.len());

        for tile in line.chars() {
            if tile == '.' {
                line_tiles.push(TileType::Open);
            } else if tile == '#' {
                line_tiles.push(TileType::Tree);
            }
        }

        slope.push(line_tiles.into());
    }

    let mut num_trees = 0usize;
    let mut line_index = 0;

    for line in slope {
        if line[line_index] == TileType::Tree {
            num_trees += 1;
        }

        line_index += 3;
    }

    println!("Number of trees encountered: {}", num_trees);

    Ok(())
}
