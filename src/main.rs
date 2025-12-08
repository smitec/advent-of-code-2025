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

#[instrument]
pub fn day04(filename: String, part_b: bool) -> Result<()> {
    let content = fs::read_to_string(filename).context("Couldn't read input")?;

    let mut width = 0;
    let mut height = 0;
    let mut map: HashSet<(i32, i32)> = HashSet::new();

    let mut moveable: Vec<(i32, i32)> = Vec::new();
    let mut first = true;

    for line in content.lines() {
        width = line.len() as i32;

        for (i, v) in line.chars().enumerate() {
            if v == '@' {
                map.insert((height, i as i32));
            }
        }
        height += 1;
    }

    let mut removed = 0;
    while first || moveable.len() > 0 {
        moveable = Vec::new();
        first = false;

        for (row, col) in map.iter() {
            let mut surrounds = 0;
            for dr in [-1, 0, 1] {
                for dc in [-1, 0, 1] {
                    if dr == 0 && dc == 0 {
                        continue;
                    }

                    if map.contains(&(row + dr, col + dc)) {
                        surrounds += 1;
                    }
                }
            }
            // println!("{:?} {:?} -> {:?}", row, col, surrounds);
            if surrounds < 4 {
                moveable.push((*row, *col));
            }
        }

        if !part_b {
            println!("{:?}", moveable.len());
            return Ok(());
        }

        for v in moveable.iter() {
            removed += 1;
            map.remove(&v);
        }
    }

    println!("{:?}", removed);

    Ok(())
}

#[instrument]
pub fn day05(filename: String, part_b: bool) -> Result<()> {
    let content = fs::read_to_string(filename).context("Couldn't read input")?;

    let mut ranges: Vec<(i64, i64)> = Vec::new();
    let mut items: Vec<i64> = Vec::new();
    let mut range_mode: bool = true;

    for line in content.lines() {
        if line.len() == 0 {
            range_mode = false;
            continue;
        }

        if range_mode {
            let (l, r) = line.split_once('-').unwrap();
            let l = l.parse::<i64>().unwrap();
            let r = r.parse::<i64>().unwrap();
            ranges.push((l, r));
        } else {
            items.push(line.parse::<i64>().unwrap());
        }
    }

    if !part_b {
        let mut count = 0;
        for item in items.iter() {
            for (lower, upper) in ranges.iter() {
                if item >= lower && item <= upper {
                    count += 1;
                    break;
                }
            }
        }

        println!("{:?}", count);
    } else {
        // Need all the non-overlapping ranges
        // insert a range a - b
        // when looking to insert a new range, loop through all existin ranges
        // If the range has a partial overlap, update the original range
        // If the range does not overlap, add it
        // What if the new range overlaps 2 ranges?
        // Is there a data structure for this?
        // SkipList maybe?
        // How about. Sort by min.
        // Grab the first range.
        // Consume ranges until end of current range is < start of the next range
        // Repeat until end.
        // That feels like it'd work.
        ranges.sort_unstable();
        let mut index = 0;
        let mut count: i64 = 0;
        let mut current_start = ranges.iter().nth(0).unwrap().0;
        let mut current_end = ranges.iter().nth(0).unwrap().1;
        while index < ranges.len() {
            let this_end = ranges.iter().nth(index).unwrap().1;
            if this_end > current_end {
                current_end = this_end;
            }
            if index == ranges.len() - 1 {
                count += current_end - current_start + 1;
                // println!("{:?}-{:?} final", current_start, current_end);
                break;
            }
            let next_start = ranges.iter().nth(index + 1).unwrap().0;

            if next_start <= current_end {
                // println!("{:?} <= {:?} no gap", next_start, current_end);
                index += 1;
            } else {
                // There is a gap
                // println!("{:?}-{:?} gap", current_start, current_end);
                count += current_end - current_start + 1;
                index += 1;
                current_start = ranges.iter().nth(index).unwrap().0;
            }
        }

        println!("{:?}", count);
    }

    Ok(())
}

#[instrument]
pub fn day06(filename: String, part_b: bool) -> Result<()> {
    let content = fs::read_to_string(filename).context("Couldn't read input")?;

    let mut problems: Vec<Vec<String>> = Vec::new();
    if !part_b {
        let mut first_row = true;
        for line in content.lines() {
            for (index, item) in line.split_whitespace().enumerate() {
                if first_row {
                    problems.push(Vec::new());
                }

                problems.get_mut(index).unwrap().push(item.to_string());
            }
            first_row = false;
        }
    } else {
        // Reverse the rows
        // Find out how far apart the operators are, they always seem to be column one
        // The that info to split the other rows and contstruct the numbers
        let ops = content.lines().last().unwrap();
        let mut op_list: Vec<String> = Vec::new();
        let mut lens: Vec<usize> = Vec::new();
        let mut current_len: usize = 1;
        for c in ops.chars() {
            match c {
                '+' => {
                    op_list.push(c.to_string());
                    lens.push(current_len - 1);
                    current_len = 1;
                }
                '*' => {
                    op_list.push(c.to_string());
                    lens.push(current_len - 1);
                    current_len = 1;
                }
                ' ' => {
                    current_len += 1;
                }
                _ => {}
            };
        }
        lens.push(current_len);
        lens.remove(0);

        //  println!("{:?}", lens);

        for (index, i) in lens.iter().enumerate() {
            let mut v: Vec<String> = Vec::new();
            for j in 0..*i {
                v.push("".to_string())
            }
            v.push(op_list.get(index).unwrap().clone());
            problems.push(v);
        }

        for (index, line) in content.lines().enumerate() {
            if line.chars().nth(0).unwrap() == '+' || line.chars().nth(0).unwrap() == '*' {
                break;
            }
            let mut line_index = 0;
            for (p_index, len) in lens.iter().enumerate() {
                for j in 0..*len {
                    let current = problems.get(p_index).unwrap().get(j).unwrap();
                    problems.get_mut(p_index).unwrap()[j] =
                        format!("{}{}", current, line.chars().nth(line_index).unwrap());
                    line_index += 1;
                }
                line_index += 1;
            }
        }

        // println!("{:?}", problems);
    }

    let mut total = 0;
    for problem in problems.iter() {
        let op = problem.last().unwrap();
        let mut running = 0;
        if op == "*" {
            running = 1;
        }
        for val in problem {
            if val == op {
                break;
            }
            if op == "*" {
                running *= val.replace(" ", "").parse::<i64>().unwrap();
            } else {
                // only other option is +
                running += val.replace(" ", "").parse::<i64>().unwrap();
            }
        }
        total += running;
    }

    println!("{:?}", total);

    Ok(())
}

#[instrument]
pub fn day07(filename: String, part_b: bool) -> Result<()> {
    let content = fs::read_to_string(filename).context("Couldn't read input")?;

    let mut row_vec: HashMap<usize, i64> = HashMap::new();
    let mut hit = 0;

    for row in content.lines() {
        let mut next_row: HashMap<usize, i64> = HashMap::new();
        for (index, value) in row.chars().enumerate() {
            match value {
                'S' => {
                    next_row.insert(index, 1);
                    break; // Assuming only thing on the S row...
                }
                '^' => {
                    // Check if there was a spot above
                    if let Some(v) = row_vec.get(&index) {
                        hit += 1;

                        let mut next_v: i64 = *v;
                        if let Some(q) = next_row.get(&(index - 1)) {
                            next_v = q + v;
                        }
                        next_row.insert(index - 1, next_v);

                        let mut next_v: i64 = *v;
                        if let Some(q) = next_row.get(&(index + 1)) {
                            next_v = q + v;
                        }
                        next_row.insert(index + 1, next_v);
                    }
                }
                '.' => {
                    // Check if there was a spot above
                    if let Some(v) = row_vec.get(&index) {
                        let mut next_v: i64 = *v;
                        if let Some(q) = next_row.get(&index) {
                            next_v = q + v;
                        }
                        next_row.insert(index, next_v);
                    }
                }
                _ => {}
            }
        }
        row_vec = next_row;
    }

    println!("{:?}", hit);

    if part_b {
        let mut b_count = 0;
        for (k, v) in row_vec.iter() {
            b_count += v;
        }
        println!("{:?}", b_count);
    }

    Ok(())
}

#[instrument]
pub fn day08(filename: String, part_b: bool, n: usize) -> Result<()> {
    let content = fs::read_to_string(filename).context("Couldn't read input")?;

    let mut nodes: Vec<(i64, i64, i64)> = Vec::new();

    for line in content.lines() {
        let mut parts = line.split(',');

        // Nth(0) because consuming moves the pointer
        nodes.push((
            parts.nth(0).unwrap().parse::<i64>().unwrap(),
            parts.nth(0).unwrap().parse::<i64>().unwrap(),
            parts.nth(0).unwrap().parse::<i64>().unwrap(),
        ));
    }

    let node_count = nodes.len();

    println!("Loaded {:?} Nodes", node_count);

    // Not the most efficient choice of data structure but it'll do
    let mut distances: HashMap<(usize, usize), Option<i64>> = HashMap::new();

    for i in 0..node_count {
        println!("{}/{}", i, node_count);
        for j in 0..i {
            if i == j {
                continue;
            }

            let a = nodes.get(i).unwrap();
            let b = nodes.get(j).unwrap();

            let d = (b.0 - a.0).pow(2) + (b.1 - a.1).pow(2) + (b.2 - a.2).pow(2);

            distances.insert((i, j), Some(d));
        }
    }

    let mut circuits_labels: HashMap<usize, usize> = HashMap::new();
    let mut circuit_index = 0;

    // Get the distances as items and sort them
    let mut d_values: Vec<((usize, usize), i64)> = distances
        .iter()
        .filter(|x| !matches!(x.1, None))
        .sorted_by(|a, b| a.1.cmp(b.1))
        .rev()
        .map(|x| (*x.0, x.1.unwrap()))
        .collect();

    if !part_b {
        for i in 0..n {
            println!("{}/{}", i, n);

            let pair = d_values.pop().unwrap();

            let idx_1 = pair.0.0;
            let idx_2 = pair.0.1;

            for idx in [idx_1, idx_2] {
                if let Some(v) = circuits_labels.get(&idx) {
                    // Find all items with value v and replace them with current index
                    let to_remap: Vec<usize> = circuits_labels
                        .iter()
                        .filter(|x| x.1 == v)
                        .map(|x| *x.0)
                        .collect();
                    for key in to_remap {
                        circuits_labels.insert(key, circuit_index);
                    }
                    circuits_labels.insert(idx, circuit_index);
                } else {
                    // Label the node with the current index
                    circuits_labels.insert(idx, circuit_index);
                }
            }

            // Update the distance to None for i,j and j,i
            distances.remove(&(idx_1, idx_2));
            distances.remove(&(idx_2, idx_1));

            circuit_index += 1;
        }

        let mut total = 1;
        let mut sizes: Vec<usize> = Vec::new();
        for i in 0..circuit_index {
            let circuit_size = circuits_labels.iter().filter(|x| *x.1 == i).count();
            sizes.push(circuit_size);
        }

        for v in sizes.iter().sorted().rev().take(3) {
            total *= v;
        }

        println!("{:?}", total);
    } else {
        let mut pair: ((usize, usize), i64) = ((0, 0), 0);
        while circuits_labels.len() < node_count {
            println!("{}/{}", circuits_labels.len(), node_count);

            pair = d_values.pop().unwrap();

            let idx_1 = pair.0.0;
            let idx_2 = pair.0.1;

            for idx in [idx_1, idx_2] {
                if let Some(v) = circuits_labels.get(&idx) {
                    // Find all items with value v and replace them with current index
                    let to_remap: Vec<usize> = circuits_labels
                        .iter()
                        .filter(|x| x.1 == v)
                        .map(|x| *x.0)
                        .collect();
                    for key in to_remap {
                        circuits_labels.insert(key, circuit_index);
                    }
                    circuits_labels.insert(idx, circuit_index);
                } else {
                    // Label the node with the current index
                    circuits_labels.insert(idx, circuit_index);
                }
            }

            // Update the distance to None for i,j and j,i
            distances.remove(&(idx_1, idx_2));
            distances.remove(&(idx_2, idx_1));

            circuit_index += 1;
        }

        let node_1 = nodes.get(pair.0.0).unwrap();
        let node_2 = nodes.get(pair.0.1).unwrap();

        println!(
            "{:?} * {:?} = {:?}",
            node_1.0,
            node_2.0,
            node_1.0 * node_2.0
        );
    }

    Ok(())
}

fn main() {
    // day01("./inputs/day01a.txt".to_string(), true);
    // day02("./inputs/day02a.txt".to_string(), true);
    // day03("./inputs/day03a.txt".to_string(), true);
    /*
    day04("./inputs/day04mini.txt".to_string(), false); // 13
    day04("./inputs/day04a.txt".to_string(), false); // 1560
    day04("./inputs/day04mini.txt".to_string(), true); // 43
    day04("./inputs/day04a.txt".to_string(), true); //
     */
    /*
    day05("./inputs/day05mini.txt".to_string(), false); // 3
    day05("./inputs/day05a.txt".to_string(), false); // 643
    day05("./inputs/day05mini.txt".to_string(), true); // 14
    day05("./inputs/day05a.txt".to_string(), true); // 342018167474526
    */

    /*
    day06("./inputs/day06mini.txt".to_string(), false); // 4277556
    day06("./inputs/day06a.txt".to_string(), false); // 3
    day06("./inputs/day06mini.txt".to_string(), true); // 4277556
    day06("./inputs/day06a.txt".to_string(), true); // 4277556
    */

    /*
    day07("./inputs/day07mini.txt".to_string(), false); // 21
    day07("./inputs/day07a.txt".to_string(), false); // 1667
    day07("./inputs/day07mini.txt".to_string(), true); // 40
    day07("./inputs/day07a.txt".to_string(), true); // 62943905501815
    */

    day08("./inputs/day08mini.txt".to_string(), false, 10); // 40
    day08("./inputs/day08a.txt".to_string(), false, 1000); // 47040
    day08("./inputs/day08mini.txt".to_string(), true, 10); // 25272
    day08("./inputs/day08a.txt".to_string(), true, 10); // 4884971896
}
