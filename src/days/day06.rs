pub fn solve(input: &str) {
    test_part1();
    solve_part1(input);

    test_part2();
    solve_part2(input);
}

fn test_part1() {
    solve_part1("3,4,3,1,2");
}

fn solve_part1(input: &str) -> u32 {
    let mut fish_today: Vec<u32> = vec![0; 9];
    let mut fish_tomorrow: Vec<u32>;

    input
        .trim()
        .split(",")
        .map(|c| c.parse::<usize>().unwrap())
        .for_each(|num| fish_today[num] += 1);

    println!("day=0 fish={:?}", fish_today);

    for round in 1..81 {
        fish_tomorrow = vec![0; 9];
        fish_tomorrow[6] = fish_today[0];
        fish_tomorrow[8] = fish_today[0];
        for i in 1..fish_today.len() {
            fish_tomorrow[i - 1] += fish_today[i];
        }
        fish_today = fish_tomorrow.clone();
        println!("day={} fish={:?}", round, fish_today);
    }

    let result: u32 = fish_today.iter().sum();
    println!("part 1 result: {}", result);
    return result;
}

fn test_part2() {
    solve_part2("3,4,3,1,2");
}

fn solve_part2(input: &str) -> u128 {
    let mut fish_today: Vec<u128> = vec![0; 9];
    let mut fish_tomorrow: Vec<u128>;

    input
        .trim()
        .split(",")
        .map(|c| c.parse::<usize>().unwrap())
        .for_each(|num| fish_today[num] += 1);

    println!("day=0 fish={:?}", fish_today);

    for round in 1..257 {
        fish_tomorrow = vec![0; 9];
        fish_tomorrow[6] = fish_today[0];
        fish_tomorrow[8] = fish_today[0];
        for i in 1..fish_today.len() {
            fish_tomorrow[i - 1] += fish_today[i];
        }
        fish_today = fish_tomorrow.clone();
        println!("day={} fish={:?}", round, fish_today);
    }

    let result: u128 = fish_today.iter().sum();
    println!("part 2 result: {}", result);
    return result;
}
