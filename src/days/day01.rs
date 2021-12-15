pub fn solve(input: &str) {
    solve_part1(input);
    solve_part2(input);
}

fn solve_part1(input: &str) {
    let result: usize = input
        .split("\n")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
        .windows(4)
        .filter(|window| window[1] > window[0])
        .count();
    println!("part 1: {}", result);
}

fn solve_part2(input: &str) {
    let result: usize = input
        .split("\n")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
        .windows(4)
        .filter(|window| (window[1] + window[2] + window[3]) > (window[0] + window[1] + window[2]))
        .count();
    println!("part 2: {}", result);
}
