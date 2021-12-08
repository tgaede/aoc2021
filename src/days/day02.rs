pub fn solve(input: &str) {
	test_part1();
	solve_part1(input);

	test_part2();
	solve_part2(input);
}

fn test_part1() {
	solve_part1("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2");
}

fn solve_part1(input: &str) {
	let mut horiz_position: u32 = 0;
	let mut depth: u32 = 0;

	for line in input.trim().split("\n") {
		let mut items = line.split(" ");
		let command: String = items.next().unwrap().parse().unwrap();
		let distance: u32 = items.next().unwrap().parse().unwrap();

		match command.as_str() {
			"forward" => horiz_position += distance,
			"down" => depth += distance,
			"up" => depth -= distance,
			_ => unreachable!(),
		};
	}

	let result: u32 = horiz_position * depth;
	println!("part 1: {}", result);
}

fn test_part2() {
	solve_part2("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2");
}


fn solve_part2(input: &str) {
	let mut horiz_position: u32 = 0;
	let mut depth: u32 = 0;
	let mut aim: u32 = 0;

	for line in input.trim().split("\n") {
		let mut items = line.split(" ");
		let command: String = items.next().unwrap().parse().unwrap();
		let distance: u32 = items.next().unwrap().parse().unwrap();

		match command.as_str() {
			"forward" => {
				horiz_position += distance;
				depth += distance*aim;
			},
			"down" => aim += distance,
			"up" => aim -= distance,
			_ => unreachable!(),
		};
	}

	let result: u32 = horiz_position * depth;
	println!("part 1: {}", result);
}
