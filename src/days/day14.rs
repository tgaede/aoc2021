use std::collections::HashMap;
use itertools::Itertools;

pub fn solve(input: &str) {
    test_part1();
    solve_part1(input);
}

fn test_part1() {
    solve_part1("NNCB\n\nCH -> B\nHH -> N\nCB -> H\nNH -> C\nHB -> C\nHC -> B\nHN -> C\nNN -> C\nBH -> H\nNC -> B\nNB -> B\nBN -> B\nBB -> N\nBC -> B\nCC -> N\nCN -> C");
}

fn solve_part1(input: &str) {
	let mut parts = input.trim().split("\n\n");
	let polymer: Vec<char> = parts.next().unwrap().chars().collect();
	let mut rules: HashMap<(char, char), char> = HashMap::new();
	for line in parts.next().unwrap().lines() {
		let (pair, to_insert) = line.split_once(" -> ").unwrap();
		let (pair_a, pair_b) = pair.chars().collect_tuple().unwrap();
		rules.insert((pair_a, pair_b), to_insert.chars().next().unwrap());
	}

	let mut step_count: usize = 1;
	let mut polymer_pairs: HashMap<(char, char), u128> = HashMap::new();
	for (a, b) in polymer.iter().tuple_windows() {
		polymer_pairs.entry((*a, *b)).and_modify(|v| *v += 1 ).or_insert(1);
	}
	while step_count < 41 {
		println!("step_count: {}", step_count);
		step_pairs(&mut polymer_pairs, &rules, *polymer.first().unwrap(), *polymer.last().unwrap());
		step_count += 1;
	}
}

fn step_pairs(polymer_pairs: &mut HashMap<(char, char), u128>, rules: &HashMap<(char, char), char>, firstchar: char, lastchar: char) {
	let mut new_char: char;
	let mut new_pairs: HashMap<(char, char), u128> = HashMap::new();
	let mut scores: HashMap<char, u128> = HashMap::new();

	for ((a, c), num) in polymer_pairs.iter() {
		if rules.contains_key(&(*a, *c)) {
			new_char = *rules.get(&(*a, *c)).unwrap();
			new_pairs.entry((*a, new_char)).and_modify(|v| *v += *num).or_insert(*num);
			new_pairs.entry((new_char, *c)).and_modify(|v| *v += *num).or_insert(*num);
		} else {
			new_pairs.insert((*a, *c), *num);
		}
	}

	polymer_pairs.clear();
	polymer_pairs.extend(new_pairs.iter());

	for ((a, b), num) in polymer_pairs.iter() {
		scores.entry(*a).and_modify(|v| *v += *num).or_insert(*num);
		scores.entry(*b).and_modify(|v| *v += *num).or_insert(*num);
	}

	let mut max_score: u128 = u128::MIN;
	let mut min_score: u128 = u128::MAX;
	let mut max_char: char = 'a';
	let mut min_char: char = 'a';
	let mut total_letters: u128 = 0;
	let mut temp_score: u128;
	for (c, v) in scores.iter() {
		if *c == firstchar || *c == lastchar {
			temp_score = *v + 1;
		}
		else {
			temp_score = *v;
		}

		if temp_score > max_score {
			max_score = temp_score;
			max_char = *c;
		}
		if temp_score < min_score {
			min_score = temp_score;
			min_char = *c;
		}
		total_letters += temp_score;
	}
	total_letters = total_letters / 2;

	println!("min={} minchar={} max={} maxchar={} total letters={} score={}", min_score/2, min_char, max_score/2, max_char, total_letters, max_score/2 - min_score/2);
}
