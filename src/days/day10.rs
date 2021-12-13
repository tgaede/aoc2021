
pub fn solve(input: &str) {
	test_part1();
	solve_part1(input);

	test_part2();
	solve_part2(input);
}

fn test_part1() {
	solve_part1(
		"[({(<(())[]>[[{[]{<()<>>\n[(()[<>])]({[<{<<[]>>(\n{([(<{}[<>[]}>{[]{[(<()>\n(((({<>}<{<{<>}{[]{[]{}\n[[<[([]))<([[{}[[()]]]\n[{[{({}]{}}([{[{{{}}([]\n{<[[]]>}<{[{[{[]{()[[[]\n[<(<(<(<{}))><([]([]()\n<{([([[(<>()){}]>(<<{{\n<{([{{}}[<[[[<>{}]]]>[]]"
	);
}

fn solve_part1(input: &str) -> u32 {
	let result: u32 = input.trim()
		.split("\n")
		.map(|line| score_line(line).0)
		.sum();

	println!("part 1 result: {}",  result);

	return result;
}

fn score_line(line: &str) -> (u32, u64) {
	let score: u32;
	let mut my_stack: Vec<char> = Vec::new();
	let mut invalid_char: char = 'a';

	for c in line.chars() {
		match c {
			'(' | '[' | '{' | '<' => my_stack.push(c),
			')' => if my_stack.len() == 0 || my_stack.pop().unwrap() != '(' { invalid_char = ')'},
			']' => if my_stack.len() == 0 || my_stack.pop().unwrap() != '[' { invalid_char = ']'},
			'}' => if my_stack.len() == 0 || my_stack.pop().unwrap() != '{' { invalid_char = '}'},
			'>' => if my_stack.len() == 0 || my_stack.pop().unwrap() != '<' { invalid_char = '>'},
			_ => panic!("invalid char found! c: {}", c)
		};
		if invalid_char != 'a' {
			break;
		}
	}

	match invalid_char {
		')' => score = 3,
		']' => score = 57,
		'}' => score = 1197,
		'>' => score = 25137,
		_ => score = 0
	};

	let score_part2 = score_part2_stack(&my_stack);
	// println!("line={}, invalid_char={}, score={} score_part2={}", line, invalid_char, score, score_part2);

	return (score, score_part2);
}

fn test_part2() {
	solve_part2(
		"[({(<(())[]>[[{[]{<()<>>\n[(()[<>])]({[<{<<[]>>(\n{([(<{}[<>[]}>{[]{[(<()>\n(((({<>}<{<{<>}{[]{[]{}\n[[<[([]))<([[{}[[()]]]\n[{[{({}]{}}([{[{{{}}([]\n{<[[]]>}<{[{[{[]{()[[[]\n[<(<(<(<{}))><([]([]()\n<{([([[(<>()){}]>(<<{{\n<{([{{}}[<[[[<>{}]]]>[]]"
	);
}

fn solve_part2(input: &str) -> u64 {
	let mut scores: Vec<u64> = input.trim()
		.split("\n")
		.filter(|&line| score_line(line).0 == 0)
		.map(|line| score_line(line).1)
		.collect();

	scores.sort();

	let result = *scores.get(scores.len()/2).unwrap();
	println!("part 2 result: {}", result);

	return result;
}

fn score_part2_stack(stack: &Vec<char>) -> u64 {
	let mut score: u64 = 0;
	for &x in stack.iter().rev() {
		score *= 5;
		match x {
			'(' => score += 1,
			'[' => score += 2,
			'{' => score += 3,
			'<' => score += 4,
			_ => panic!("invalid char c={}", x)
		};
	}

	return score;
}
