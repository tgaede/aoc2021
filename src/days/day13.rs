use std::cmp;

pub fn solve(input: &str) {
    test_part1();
    solve_part1(input);
}

fn test_part1() {
    solve_part1("6,10\n0,14\n9,10\n0,3\n10,4\n4,11\n6,0\n6,12\n4,1\n0,13\n10,12\n3,4\n3,0\n8,4\n1,10\n2,14\n8,10\n9,0\n\nfold along y=7\nfold along x=5");
}

fn solve_part1(input: &str) {
    let mut grid: Vec<Vec<u8>>;
    let mut parts = input.trim().split("\n\n");
    let points: Vec<(usize, usize)> = parts
        .next()
        .unwrap()
        .lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(xstr, ystr)| {
            (
                xstr.parse::<usize>().unwrap(),
                ystr.parse::<usize>().unwrap(),
            )
        })
        .collect();
    let max_x = points.iter().fold(0, |m, (x, _y)| cmp::max(m, *x))+1;
    let max_y = points.iter().fold(0, |m, (_x, y)| cmp::max(m, *y))+1;
    grid = vec![vec![0; max_x]; max_y];

    for (x, y) in points.iter() {
        grid[*y][*x] = 1;
    }

    for line in parts.next().unwrap().lines() {
        let (xy, valuestr) = line.rsplit(' ').next().unwrap().split_once("=").unwrap();
        let x: usize;
        let y: usize;

        if xy == "x" {
            x = valuestr.parse::<usize>().unwrap();
            y = 0;
        } else {
            y = valuestr.parse::<usize>().unwrap();
            x = 0;
        }
        fold(&mut grid, x, y);
    }

	print_grid(&grid);
}

fn fold(grid: &mut Vec<Vec<u8>>, x: usize, y: usize) {
	if x != 0 {
		// fold left
		for r in 0..grid.len() {
			let trunc_length = grid[r].len() - x - 1;
			for c in 1..(trunc_length+1) {
				grid[r][x - c] |= grid[r][x + c];
			}
			grid[r].truncate(x);
		}
	} else if y != 0 {
		// fold up
		let trunc_length = grid.len() - y - 1;
		for c in 0..grid.first().unwrap().len() {
			for r in 1..(trunc_length+1) {
				grid[y-r][c] |= grid[y+r][c];
			}
		}
		grid.truncate(y);
	}
}

fn print_grid(grid: &Vec<Vec<u8>>) {
	let mut result: usize = 0;
	for row in grid.iter() {
		let mut line: String = String::new();
		for item in row.iter() {
			if *item == 0 {
				line.push('.');
			}
			else {
				result += 1;
				line.push('#');
			}
		}
		println!("{}", line);
	}
	println!("visible dots={} rows={} cols={}", result, grid.len(), grid.first().unwrap().len());
	println!("");
}
