use core::panic;
use std::{
    collections::{HashMap, HashSet},
    fs, usize,
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

#[instrument]
pub fn day02(filename: String, part_b: bool) -> Result<()> {
    let content = fs::read_to_string(filename).context("Couldn't read input")?;

    // invalid IDs are the same sequence repeated twice.
    // For an integer of length N (where N is even), split N into 2 parts
    // There will be 1 invlid integer for each value of half(N) in the range
    // log_10(i) will give the length, log_10(min) log_10(max) will give the bounds
    // Can also just use the string length
    // if log_10(x) % 2 == 0 -> for v in x -> x*10 - 1 -> if v + v in range -> add v + v (str wise)
    // then increment x to 100*x

    let mut total = 0;
    let clean = content.replace('\n', "");
    for range in clean.split(',') {
        //println!("{:?}", range);
        let (r_low, r_high) = range.split_once('-').unwrap();
        let r_low = r_low.parse::<i64>().unwrap();
        let r_high = r_high.parse::<i64>().unwrap();
        let mut x = r_low;

        if !part_b {
            while x <= r_high {
                let digits = x.to_string().len();
                if digits % 2 == 0 {
                    let half = (digits / 2) as usize;
                    let half_digit = x.to_string()[..half].parse::<i64>().unwrap();
                    let mut h: i64 = half_digit;
                    let upper = 10i64.pow(half as u32);
                    //println!("{:?} -> upper", upper);
                    while h < upper {
                        //println!("{:?} -> h", h);
                        let test = h + h * 10i64.pow(half as u32);
                        if test >= r_low {
                            if test > r_high {
                                break;
                            }
                            //println!("{:?} -> test", test);
                            total += test;
                        }
                        h += 1;
                    }
                }
                x = 10i64.pow(digits as u32);
            }
        } else {
            // part b version, can be any repeating sequence
            // ugly version is start at 1, repeat it until its > min and < max
            // if repeated once is > max quit
            // println!("--");
            let mut t = 1;
            let mut added: Vec<i64> = Vec::new();
            while t <= r_high {
                let mut repeat = format!("{}{}", t, t);

                if repeat.parse::<i64>().unwrap() > r_high {
                    break;
                }

                while repeat.parse::<i64>().unwrap() <= r_high {
                    let val = repeat.parse::<i64>().unwrap();
                    if val >= r_low {
                        if !added.contains(&val) {
                            // println!("{:?} -> val", val);
                            total += val;
                            added.push(val);
                        }
                    }
                    repeat = format!("{}{}", repeat, t);
                }
                t += 1;
            }
        }
    }

    println!("{:?}", total);

    Ok(())
}

fn max_substr(digits: String, len: usize, cache: &mut HashMap<(String, usize), i64>) -> i64 {
    if len == 1 {
        return digits
            .chars()
            .map(|x| x.to_string().parse::<i64>().unwrap())
            .max()
            .unwrap();
    }

    let mut max = 0;
    let mut max_current = 0;
    for i in 0..digits.len() - len + 1 {
        let current = digits
            .chars()
            .nth(i)
            .unwrap()
            .to_string()
            .parse::<i64>()
            .unwrap();
        if current > max_current {
            let d = digits[i + 1..].to_string();
            let l = len - 1;
            let pls: i64;
            if cache.contains_key(&(d.clone(), l)) {
                pls = *cache.get(&(d, l)).unwrap();
            } else {
                pls = max_substr(d.clone(), l, cache);
                cache.insert((d, l), pls);
            }
            let check = current * 10i64.pow((len - 1) as u32) + pls;
            if check > max {
                max = check;
            }
            max_current = current;
        }
    }

    return max;
}

// 17613

#[instrument]
pub fn day03(filename: String, part_b: bool) -> Result<()> {
    let content = fs::read_to_string(filename).context("Couldn't read input")?;

    let mut total_a = 0;
    let mut total_b = 0;

    for line in content.lines() {
        let mut cache: HashMap<(String, usize), i64> = HashMap::new();
        total_a += max_substr(line.to_string(), 2, &mut cache);
        total_b += max_substr(line.to_string(), 12, &mut cache);
    }

    println!("{:?} {:?}", total_a, total_b);

    Ok(())
}

fn main() {
    // day01("./inputs/day01a.txt".to_string(), true);
    // day02("./inputs/day02a.txt".to_string(), true);
    day03("./inputs/day03a.txt".to_string(), true);
}
