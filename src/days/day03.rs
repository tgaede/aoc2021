pub fn solve(input: &str) {
    test_part1();
    solve_part1(input);

    test_part2();
    solve_part2(input);
}

fn test_part1() {
    solve_part1(
        "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010",
    );
}

fn solve_part1(input: &str) {
    let mut one_count: Vec<i32> = vec![0; input.trim().find('\n').unwrap()];
    let mut gamma_rate: u32 = 0; // most common
    let epsilon_rate: u32; // least common

    for line in input.trim().split('\n') {
        let mut i: usize = 0;
        for c in line.chars() {
            if c == '0' {
                one_count[i] -= 1;
            } else {
                one_count[i] += 1;
            }
            i += 1;
        }
    }

    for i in 0..one_count.len() {
        if one_count[i] > 0 {
            gamma_rate |= 0x1;
        } else if one_count[i] == 0 {
            panic!("even stephen bits, shouldn't happen?, i: {}", i);
        }
        if (i + 1) < one_count.len() {
            gamma_rate = gamma_rate << 1;
        }
    }

    epsilon_rate = (0xffffffff >> (32 - one_count.len())) & (!gamma_rate);

    println!(
        "gamma_rate: {:b}, epsilon rate: {:b}",
        gamma_rate, epsilon_rate
    );
    println!("part 1 result: {}", gamma_rate * epsilon_rate);
}

fn test_part2() {
    solve_part2(
        "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010",
    );
}

fn solve_part2(input: &str) -> u32 {
    let parsed_input: Vec<Vec<u8>> = input
        .trim()
        .split('\n')
        .map(|x| x.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();

    let result: u32 = solve_oxy(&parsed_input) * solve_co2(&parsed_input);
    println!("part 2: {}", result);
    return result;
}

fn solve_oxy(input: &Vec<Vec<u8>>) -> u32 {
    let mut oxy_options: Vec<Vec<u8>> = input.clone();
    let mut oxygen_generator_rating: Vec<u8> = Vec::new();
    let mut most_common: u8;
    let mut i: usize = 0;

    while oxy_options.len() > 1 {
        most_common = most_common_bit(&oxy_options, 1, i);
        oxygen_generator_rating.push(most_common);
        oxy_options.retain(|x| {
            x.iter()
                .zip(oxygen_generator_rating.iter())
                .all(|(a, b)| *a == *b)
        });
        println!(
            "i: {} most_common: {} oxy_options.len() now: {} rating now: {:?}",
            i,
            most_common,
            oxy_options.len(),
            oxygen_generator_rating
        );
        i += 1;
    }

    let result: u32 = bit_array_to_u32(&oxy_options.first().unwrap());
    println!("oxygen result: {}", result);
    return result;
}

fn solve_co2(input: &Vec<Vec<u8>>) -> u32 {
    let mut co2_options: Vec<Vec<u8>> = input.clone();
    let mut co2_rating: Vec<u8> = Vec::new();
    let mut most_common: u8;
    let mut i: usize = 0;

    while co2_options.len() > 1 {
        most_common = most_common_bit(&co2_options, 1, i) ^ 1;
        co2_rating.push(most_common);
        co2_options.retain(|x| x.iter().zip(co2_rating.iter()).all(|(a, b)| *a == *b));
        println!(
            "i: {} most_common: {} co2_options.len() now: {} rating now: {:?}",
            i,
            most_common,
            co2_options.len(),
            co2_rating
        );
        i += 1;
    }

    let result: u32 = bit_array_to_u32(&co2_options.first().unwrap());
    println!("co2 result: {}", result);
    return result;
}

fn most_common_bit(data: &Vec<Vec<u8>>, default_value: u8, index: usize) -> u8 {
    let one_count: i32 = data.iter().fold(0, |acc, x| match x.get(index).unwrap() {
        0 => acc - 1,
        1 => acc + 1,
        _ => panic!("unknown array entry"),
    });

    let most_common: u8;
    if one_count > 0 {
        most_common = 1;
    } else if one_count < 0 {
        most_common = 0;
    } else {
        most_common = default_value;
    }

    return most_common;
}

fn bit_array_to_u32(array: &Vec<u8>) -> u32 {
    let mut result: u32 = 0;
    for i in 0..array.len() {
        result |= array[i] as u32;
        if i != (array.len() - 1) {
            result = result << 1;
        }
    }
    return result;
}
