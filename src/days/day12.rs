use std::collections::HashSet;

// room for many optimizations - don't clone paths, dont clone strings, do depth first push/pop
// deduplicate code

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
    let connections: Vec<(String, String)> = input
        .trim()
        .split("\n")
        .map(|line| {
            let mut parts = line.split("-");
            (parts.next().unwrap().to_string(), parts.next().unwrap().to_string())
        })
        .collect();

    let mut active_paths: Vec<Vec<String>> = vec![vec!["start".to_string()]];
	let mut done_paths: Vec<Vec<String>> = Vec::new();

	// let mut round: usize = 1;
	let mut added: usize;
	loop  {
		added = add_connections(&mut active_paths, &mut done_paths, &connections);
		// println!("round {}, added {}", round, added);
		// print_paths(&paths);
		// println!("");
		// round += 1;

		if added == 0 {
			break;
		}
	}

	let result = done_paths.iter().filter(|path| {
		*path.last().unwrap() == "end".to_string()
	})
		.count();

	println!("part 1 result: {}", result);
}

fn add_connections(active_paths: &mut Vec<Vec<String>>, done_paths: &mut Vec<Vec<String>>, connections: &Vec<(String, String)>) -> usize {
	let mut new_paths: Vec<Vec<String>> = Vec::new();
	let connections_added: usize;

	// println!("paths={:?}, connections={:?}", paths, connections);

    for path in active_paths.drain(..) {
        let last_item = path.last().unwrap().clone();
        let items_to_add = connections
            .iter()
            .filter(|(k, v)| {
				// println!("*k={} *v={} last_item={} filter={}", *k, *v, last_item, *k == last_item || *v == last_item);
				*k == last_item || *v == last_item
			})
			.map(|(k, v)| if *k == last_item { v.clone() } else { k.clone() })
			.filter(|item| {
				// println!("item={} filter={}", item, item.chars().next().unwrap().is_lowercase() && !path.contains(item) || item.chars().next().unwrap().is_uppercase());
				item.chars().next().unwrap().is_lowercase() && !path.contains(item) || item.chars().next().unwrap().is_uppercase()
			});

		for item in items_to_add {
			let mut new_path = path.clone();
			new_path.push(item.clone());
			if item == "end" {
				done_paths.push(new_path);
			}
			else if ! new_paths.contains(&new_path) {
				new_paths.push(new_path);
			}
		}
		if new_paths.len() == 0 {
			done_paths.push(path);
		}
    }

	connections_added = new_paths.len();
	active_paths.extend(new_paths);

    return connections_added;
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
	let connections: Vec<(String, String)> = input
		.trim()
		.split("\n")
		.map(|line| {
			let mut parts = line.split("-");
			(parts.next().unwrap().to_string(), parts.next().unwrap().to_string())
		})
		.collect();

	let mut active_paths: Vec<Vec<String>> = vec![vec!["start".to_string()]];
	let mut done_paths: Vec<Vec<String>> = Vec::new();

	let mut round: usize = 1;
	let mut added: usize;
	loop  {
		added = add_connections_part2(&mut active_paths, &mut done_paths, &connections);
		println!("round {}, added {}", round, added);
		// println!("active paths=");
		// print_paths(&active_paths);
		// println!("done paths=");
		// print_paths(&done_paths);
		// println!("");
		round += 1;

		if added == 0 {
			break;
		}
	}

	let result = done_paths.iter().filter(|path| {
		*path.last().unwrap() == "end".to_string()
	})
		.count();

	println!("part 2 result: {}", result);
}


fn add_connections_part2(active_paths: &mut Vec<Vec<String>>, done_paths: &mut Vec<Vec<String>>, connections: &Vec<(String, String)>) -> usize {
	let mut new_paths: Vec<Vec<String>> = Vec::new();
	let connections_added: usize;

	// println!("paths={:?}, connections={:?}", paths, connections);

	for path in active_paths.drain(..) {
		let last_item = path.last().unwrap().clone();
		let lowercase_okay = can_revisit_lowercase(&path);
		let items_to_add = connections
			.iter()
			.filter(|(k, v)| {
				// println!("*k={} *v={} last_item={} filter={}", *k, *v, last_item, *k == last_item || *v == last_item);
				*k == last_item || *v == last_item
			})
			.map(|(k, v)| if *k == last_item { v.clone() } else { k.clone() })
			.filter(|item| {
				// println!("item={} filter={}", item, item.chars().next().unwrap().is_lowercase() && !path.contains(item) || item.chars().next().unwrap().is_uppercase());
				if item == "start" {
					false
				} else if item.chars().next().unwrap().is_lowercase() && path.contains(item) {
					lowercase_okay
				} else {
					true
				}
			});

		for item in items_to_add {
			let mut new_path = path.clone();
			new_path.push(item.clone());
			if item == "end" {
				done_paths.push(new_path);
			}
			else if ! new_paths.contains(&new_path) {
				new_paths.push(new_path);
			}
		}
		if new_paths.len() == 0 {
			done_paths.push(path);
		}
	}

	connections_added = new_paths.len();
	active_paths.extend(new_paths);

	return connections_added;
}

fn can_revisit_lowercase(path: &Vec<String>) -> bool {
	let mut lowercase_items: HashSet<&String> = HashSet::new();
	for item in path.iter() {
		if item.chars().next().unwrap().is_lowercase() {
			if ! lowercase_items.insert(item) {
				return false;
			}
		}
	}

	return true;
}
