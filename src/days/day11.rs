use itertools::Itertools;
use std::cmp;
use std::collections::{HashSet, VecDeque};

pub fn solve(input: &str) {
    test_part1();
    solve_part1(input);

    test_part2();
    solve_part2(input);
}

fn test_part1() {
    solve_part1(
		"5483143223\n2745854711\n5264556173\n6141336146\n6357385478\n4167524645\n2176841721\n6882881134\n4846848554\n5283751526"
	);
}

fn solve_part1(input: &str) -> u32 {
    let mut octopi: Vec<Vec<u32>> = input
        .trim()
        .split("\n")
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect();

    println!("octopi after 0 steps");
    print_octopi(&octopi);
    println!("");

    let mut flash_count: u32 = 0;
    for i in 1..101 {
        flash_count += step(&mut octopi);

        if i % 10 == 0 || i <= 10 {
            println!("octopi after {} steps", i);
            print_octopi(&octopi);
            println!("");
        }
    }

    println!("part 1 result: {}", flash_count);

    return flash_count;
}

fn step(octopi: &mut Vec<Vec<u32>>) -> u32 {
    let mut flashes: HashSet<(usize, usize)> = HashSet::new();
    let mut to_process: VecDeque<(usize, usize)> = VecDeque::new();

    // first, the energy level of each octopus increases by 1
    for i in 0..octopi.len() {
        for j in 0..octopi[i].len() {
            octopi[i][j] += 1;
            if octopi[i][j] > 9 {
                // println!("adding i={} j={} to processing", i, j);
                to_process.push_back((i, j));
            }
        }
    }

    // then, any octopus >= 9 flashes increasing nearby by 1
    // if any of them get >= 9 they flash too
    while !to_process.is_empty() {
        let (i, j) = to_process.pop_front().unwrap();

        if !flashes.contains(&(i, j)) {
            flashes.insert((i, j));
            for r in i.saturating_sub(1)..cmp::min(octopi.len(), i + 2) {
                for c in j.saturating_sub(1)..cmp::min(octopi.first().unwrap().len(), j + 2) {
                    if !(r == i && j == c) {
                        // println!("processing i={} j={}, neighbor r={} c={}", i, j, r, c);
                        octopi[r][c] += 1;
                        if octopi[r][c] > 9 && !flashes.contains(&(r, c)) {
                            // println!("adding i={} j={} to processing", r, c);
                            to_process.push_back((r, c));
                        }
                    }
                }
            }
        }
    }

    // then reset all flashed ones to 0
    for (i, j) in flashes.iter() {
        octopi[*i][*j] = 0;
    }

    return flashes.len() as u32;
}

fn print_octopi(octopi: &Vec<Vec<u32>>) {
    for i in 0..octopi.len() {
        let mut line: String = String::new();
        for j in 0..octopi[i].len() {
            line.push_str(octopi[i][j].to_string().as_str());
        }
        println!("{}", line);
    }
}

fn test_part2() {
    solve_part2(
		"5483143223\n2745854711\n5264556173\n6141336146\n6357385478\n4167524645\n2176841721\n6882881134\n4846848554\n5283751526"
	);
}

fn solve_part2(input: &str) -> u64 {
    let mut octopi: Vec<Vec<u32>> = input
        .trim()
        .split("\n")
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect();

    let mut i: usize = 0;
    loop {
        i += 1;

        if octopi.len() * octopi.first().unwrap().len() == step(&mut octopi) as usize {
            break;
        }

        if i % 10 == 0 || i <= 10 {
            println!("ran {} steps", i);
        }
    }

    println!("part 2 result: {}", i);

    return i as u64;
}
