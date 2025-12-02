use core::panic;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

use anyhow::{Context, Result};
use itertools::{Combinations, Itertools, Permutations};
use tracing::{Level, debug, error, event, info, instrument, warn};
use tracing_subscriber::{EnvFilter, field::debug};

mod util;

#[instrument]
pub fn day01(filename: String, part_b: bool) -> Result<()> {
    let content = fs::read_to_string(filename).context("Couldn't read input")?;

    let mut current: i32 = 50;
    let mut count = 0;

    for line in content.lines() {
        let mut mul = 1;
        if line.chars().nth(0).unwrap() == 'L' {
            mul = -1;
        }

        let val = line[1..].parse::<i32>().unwrap();

        println!("Starting at {:?}", current);
        let initial = current;
        let turns = val.div_euclid(100).abs();
        current = current + val.rem_euclid(100) * mul;
        println!("Ending at {:?} ({:?})", current, current.rem_euclid(100));

        if part_b {
            if current > 100 {
                println!(
                    "Adding for full Rotation {:?} -> {:?} = {:?}",
                    line,
                    current,
                    current.div_euclid(100)
                );
                count += 1
            }

            if current < 0 && initial > 0 {
                println!(
                    "Adding for mini Negative Rotation {:?} -> {:?} = {:?}",
                    line, current, 1
                );
                count += 1;
            }

            if initial == 0 && current.rem_euclid(100) == 0 {
                count -= 1;
            }
            count += turns;
        }

        current = current.rem_euclid(100);

        if current == 0 {
            println!("Adding for at 0");
            count += 1;
        }
    }

    println!("Final Score {:?}", count);

    Ok(())
}

fn main() {
    day01("./inputs/day01a.txt".to_string(), true);
}
