pub fn solve(input: &str) {
	test_part1();
	solve_part1(input);

	test_part2();
	solve_part2(input);
}

fn test_part1() {
	solve_part1(
		"16,1,2,0,4,2,7,1,2,14"
	);
}

fn solve_part1(input: &str) -> u32 {
	let mut numbers: Vec<u32> = input.trim().split(",")
		.map(|c| c.parse::<u32>().unwrap())
		.collect();

	numbers.sort();
	let mut least_fuel: u32 = u32::MAX;
	for i in numbers.iter() {
		let mut fuel: u32 = 0;
		for j in numbers.iter() {
			if *j > *i {
				fuel += *j - *i;
			}
			else {
				fuel += *i - *j;
			}
		}

		println!("fuel={} at position={}", fuel, *i);
		if fuel < least_fuel {
			least_fuel = fuel;
		}
	}

	println!("part 1 result: {}", least_fuel);
	return least_fuel;
}

fn test_part2() {
	solve_part2(
		"16,1,2,0,4,2,7,1,2,14"
	);
}

fn solve_part2(input: &str) -> u32 {
	let mut numbers: Vec<u32> = input.trim().split(",")
		.map(|c| c.parse::<u32>().unwrap())
		.collect();

	numbers.sort();
	let mut least_fuel: u32 = u32::MAX;
	for i in numbers.iter() {
		let mut fuel: u32 = 0;
		for j in numbers.iter() {
			if *j > *i {
				fuel += (*j - *i)*(*j - *i + 1)/2;
			}
			else {
				fuel += (*i - *j)*(*i - *j + 1)/2;
			}
		}

		println!("fuel={} at position={}", fuel, *i);
		if fuel < least_fuel {
			least_fuel = fuel;
		}
	}

	println!("part 2 result: {}", least_fuel);
	return least_fuel;
}
