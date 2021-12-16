use std::collections::HashMap;

pub fn solve(input: &str) {
    test_part1();
    solve_part1(input);

    test_part2();
    solve_part2(input);
}

fn test_part1() {
    solve_part1("start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end");
    solve_part1("dc-end\nHN-start\nstart-kj\ndc-start\ndc-HN\nLN-dc\nHN-end\nkj-sa\nkj-HN\nkj-dc");
    solve_part1(
		"fs-end\nhe-DX\nfs-he\nstart-DX\npj-DX\nend-zg\nzg-sl\nzg-pj\npj-he\nRW-he\nfs-DX\npj-RW\nzg-RW\nstart-pj\nhe-WI\nzg-he\npj-fs\nstart-RW"
	);
}

fn solve_part1(input: &str) {
	let mut connections: HashMap<&str, Vec<&str>> = HashMap::new();
	for line in input.trim().lines() {
		let (a, b) = line.split_once('-').unwrap();
		connections.entry(a).or_insert(Vec::new()).push(b);
		connections.entry(b).or_insert(Vec::new()).push(a);
	}

	let result = count_paths(&connections, "start", &mut Vec::new(), true);
	println!("part 1 result: {}", result);
}

fn count_paths<'a>(connections: &HashMap<&'a str, Vec<&'a str>>, current_cave: &'a str, current_path: & mut Vec<&'a str>, mut seen_twice: bool ) -> usize {
	if current_cave == "end" {
		return 1;
	}

	if current_cave.chars().any(|c| c.is_lowercase()) && current_path.contains(&current_cave) {
		if seen_twice || current_cave == "start" {
			return 0
		}
		seen_twice = true;
	}

	current_path.push(current_cave);
	let result: usize = connections[current_cave].iter().map(|next_cave| {
		count_paths(connections, next_cave, current_path, seen_twice)
	}).sum();
	current_path.pop();

	return result;
}

// fn print_paths(paths: &Vec<Vec<String>>) {
// 	for path in paths.iter() {
// 		let line = path.iter().fold(String::new(), |acc, item| {
// 			format!("{},{}", acc, item)
// 		});
// 		println!("{}", line);
// 	}
// }

fn test_part2() {
	solve_part2("start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end");
	solve_part2("dc-end\nHN-start\nstart-kj\ndc-start\ndc-HN\nLN-dc\nHN-end\nkj-sa\nkj-HN\nkj-dc");
	solve_part2(
		"fs-end\nhe-DX\nfs-he\nstart-DX\npj-DX\nend-zg\nzg-sl\nzg-pj\npj-he\nRW-he\nfs-DX\npj-RW\nzg-RW\nstart-pj\nhe-WI\nzg-he\npj-fs\nstart-RW"
	);
}

fn solve_part2(input: &str) {
	let mut connections: HashMap<&str, Vec<&str>> = HashMap::new();
	for line in input.trim().lines() {
		let (a, b) = line.split_once('-').unwrap();
		connections.entry(a).or_insert(Vec::new()).push(b);
		connections.entry(b).or_insert(Vec::new()).push(a);
	}

	let result = count_paths(&connections, "start", &mut Vec::new(), false);
	println!("part 2 result: {}", result);
}

