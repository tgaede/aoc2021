use std::collections::HashMap;

pub fn solve(input: &str) {
	test_part1();
	solve_part1(input);

	test_part2();
	solve_part2(input);
}

fn test_part1() {
	solve_part1(
		"0,9 -> 5,9\n8,0 -> 0,8\n9,4 -> 3,4\n2,2 -> 2,1\n7,0 -> 7,4\n6,4 -> 2,0\n0,9 -> 2,9\n3,4 -> 1,4\n0,0 -> 8,8\n5,5 -> 8,2"
	);
}

fn solve_part1(input: &str) -> u32 {
	let mut grid: HashMap<(u32, u32), u32> = HashMap::new();

	input.trim().split("\n").map(|line| {
		let mut line_parts = line.split_whitespace();
		let mut number_parts = line_parts.next().unwrap().split(",");
		let x1: u32 = number_parts.next().unwrap().parse::<u32>().unwrap();
		let y1: u32 = number_parts.next().unwrap().parse::<u32>().unwrap();
		number_parts = line_parts.skip(1).next().unwrap().split(",");
		let x2: u32 = number_parts.next().unwrap().parse::<u32>().unwrap();
		let y2: u32 = number_parts.next().unwrap().parse::<u32>().unwrap();

		(x1, y1, x2, y2)
	}).filter(|(x1, y1, x2, y2)| {
		x1 == x2 || y1 == y2
	}).for_each(|(x1, y1, x2, y2)| {
		draw_line(&mut grid, x1, y1, x2, y2);
	});

	let result: u32 = grid.values().filter(|x| **x > 1).count() as u32;
	println!("part 1 result: {}", result);
	return result;
}

fn draw_line(grid: &mut HashMap<(u32, u32), u32>, x1: u32, y1: u32, x2: u32, y2: u32) {
	let x_increment: i32;
	let y_increment: i32;

	if x1 < x2 {
		x_increment = 1;
	}
	else if x1 > x2 {
		x_increment = -1;
	} else {
		x_increment = 0;
	}

	if y1 < y2 {
		y_increment = 1;
	}
	else if y1 > y2 {
		y_increment = -1;
	} else {
		y_increment = 0;
	}

	let mut x: u32 = x1;
	let mut y: u32 = y1;
	while x != x2 || y != y2 {
		if grid.contains_key(&(x, y)) {
			grid.insert((x, y), grid.get(&(x, y)).unwrap() + 1);
		}
		else {
			grid.insert( (x, y), 1);
		}

		if x_increment > 0 {
			x += 1;
		} else if x_increment < 0 {
			x -= 1;
		}
		if y_increment > 0 {
			y += 1;
		} else if y_increment < 0 {
			y -= 1;
		}
	}
	if grid.contains_key(&(x, y)) {
		grid.insert((x, y), grid.get(&(x, y)).unwrap() + 1);
	}
	else {
		grid.insert( (x, y), 1);
	}
}

fn print_grid(grid: &HashMap<(u32, u32), u32>) {
	let mut max_x: u32 = 0;
	let mut max_y: u32 = 0;

	grid.keys().for_each(|(x, y)| {
		if *x > max_x {
			max_x = *x;
		}
		if *y > max_y {
			max_y = *y;
		}
	});

	for y in 0..(max_y+1) {
		let mut line: String = String::new();
		for x in 0..(max_x+1) {
			if grid.contains_key(&(x, y)) {
				line.push_str(grid.get(&(x,y)).unwrap().to_string().as_str());
			}
			else {
				line.push('.');
			}
		}
		println!("{}", line);
	}
}

fn test_part2() {
	solve_part2(
		"0,9 -> 5,9\n8,0 -> 0,8\n9,4 -> 3,4\n2,2 -> 2,1\n7,0 -> 7,4\n6,4 -> 2,0\n0,9 -> 2,9\n3,4 -> 1,4\n0,0 -> 8,8\n5,5 -> 8,2"
	);
}

fn solve_part2(input: &str) -> u32 {
	let mut grid: HashMap<(u32, u32), u32> = HashMap::new();

	input.trim().split("\n").map(|line| {
		let mut line_parts = line.split_whitespace();
		let mut number_parts = line_parts.next().unwrap().split(",");
		let x1: u32 = number_parts.next().unwrap().parse::<u32>().unwrap();
		let y1: u32 = number_parts.next().unwrap().parse::<u32>().unwrap();
		number_parts = line_parts.skip(1).next().unwrap().split(",");
		let x2: u32 = number_parts.next().unwrap().parse::<u32>().unwrap();
		let y2: u32 = number_parts.next().unwrap().parse::<u32>().unwrap();

		(x1, y1, x2, y2)
	}).for_each(|(x1, y1, x2, y2)| {
		draw_line(&mut grid, x1, y1, x2, y2);
	});

	let result: u32 = grid.values().filter(|x| **x > 1).count() as u32;
	println!("part 2 result: {}", result);
	return result;
}
