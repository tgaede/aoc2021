use itertools::Itertools;
use std::collections::HashSet;

pub fn solve(input: &str) {
	test_part1();
	solve_part1(input);

	test_part2();
	solve_part2(input);
}

fn test_part1() {
	solve_part1(
		"2199943210\n3987894921\n9856789892\n8767896789\n9899965678"
	);
}

fn solve_part1(input: &str) -> u32 {
	let grid: Vec<Vec<u32>> = input.trim().split("\n")
		.map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
		.collect();

	let mut result: u32 = 0;
	for i in 0..grid.len() {
		for j in 0..grid.get(i).unwrap().len() {
			if is_low_point(&grid, i, j) {
				result += grid[i][j] + 1;
			}
		}
	}

	println!("part 1 result: {}", result);
	return result;
}

fn is_low_point(grid: &Vec<Vec<u32>>, row: usize, col: usize) -> bool {
	let mut result: bool = grid.len() > 0 && grid.first().unwrap().len() > 0
		&& row < grid.len()
		&& col < grid.first().unwrap().len();

	if result && col >= 1 { // check left
		result &= grid[row][col] < grid[row][col -1];
	}
	if result && row >= 1 { // check up
		result &= grid[row][col] < grid[row-1][col];
	}
	if result && col + 1 < grid.first().unwrap().len() { // check right
		result &= grid[row][col] < grid[row][col+1];
	}
	if result && row +1 < grid.len() { // check down
		result &= grid[row][col] < grid[row+1][col];
	}

	return result;
}


fn test_part2() {
	solve_part2(
		"2199943210\n3987894921\n9856789892\n8767896789\n9899965678"
	);
}

fn solve_part2(input: &str) -> u32 {
	let grid: Vec<Vec<u32>> = input.trim().split("\n")
		.map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
		.collect();

	let mut basins: Vec<usize> = Vec::new();
	for r in 0..grid.len() {
		for c in 0..grid.get(r).unwrap().len() {
			if is_low_point(&grid, r, c) {
				let mut my_basin: HashSet<(usize, usize)> = HashSet::new();
				collect_basin(&grid, r, c, &mut my_basin);
				basins.push(my_basin.len());
			}
		}
	}

	basins.sort();
	let result: u32 = basins.iter().rev().take(3).product::<usize>() as u32;
	println!("part 2 result: {}", result);
	return result;
}

fn collect_basin(grid: &Vec<Vec<u32>>, row: usize, col: usize, basin: &mut HashSet<(usize, usize)>) {
	let mut to_collect: Vec<(usize, usize)> = Vec::new();
	basin.insert((row, col));

	// down
	if row + 1 < grid.len()
		&& ! basin.contains(&(row+1, col))
		&& grid[row+1][col] != 9 {
		to_collect.push((row+1, col));
	}
	// right
	if col + 1 < grid.first().unwrap().len()
		&& ! basin.contains(&(row, col+1))
		&& grid[row][col+1] != 9 {
		to_collect.push((row, col+1));
	}
	// up
	if row >= 1
		&& ! basin.contains( &(row-1, col))
		&& grid[row-1][col] != 9 {
		to_collect.push((row-1, col));
	}
	// left
	if col >= 1
		&& ! basin.contains(&(row, col-1))
		&& grid[row][col-1] != 9 {
		to_collect.push((row, col-1));
	}

	to_collect.iter().for_each(|(newrow, newcol)| collect_basin(&grid, *newrow, *newcol, basin));
}
