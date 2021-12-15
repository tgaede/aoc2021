use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn solve(input: &str) {
    test_part1();
    solve_part1(input);

    test_part2();
    solve_part2(input);
}

fn test_part1() {
    solve_part1(
		"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\nedbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc\nfgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg\nfbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb\naecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea\nfgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb\ndbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe\nbdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef\negadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb\ngcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
	);
}

fn solve_part1(input: &str) -> u32 {
    let result: u32 = input
        .trim()
        .split("\n")
        .map(|line| line.split("|").skip(1).next().unwrap())
        .map(|rightside| {
            rightside
                .split_whitespace()
                .filter(|word| {
                    word.len() == 2 || word.len() == 3 || word.len() == 7 || word.len() == 4
                })
                .count() as u32
        })
        .sum();

    println!("part 1 result: {}", result);
    return result;
}

fn test_part2() {
    solve_part2(
		"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\nedbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc\nfgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg\nfbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb\naecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea\nfgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb\ndbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe\nbdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef\negadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb\ngcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
	);
}

fn solve_part2(input: &str) -> u32 {
    let result: u32 = input
        .trim()
        .split("\n")
        .map(|line| process_line(line))
        .sum();

    println!("part 2 result: {}", result);

    return result;
}

fn process_line(line: &str) -> u32 {
    let mut patterns: HashSet<String> = HashSet::new();
    let mut mapping: HashMap<String, u32> = HashMap::new();
    let mut output: Vec<String> = Vec::new();
    let mut parts = line.split('|');

    patterns.extend(
        parts
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|s| s.to_string()),
    );
    output.extend(
        parts
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|s| s.to_string()),
    );

    calculate_mapping(&mut patterns, &mut mapping);

    return output
        .iter()
        .map(|out| {
            mapping
                .iter()
                .find_map(|(pat, _num)| {
                    if pat.len() == out.len() && contains_chars(&pat, out) {
                        Some(pat)
                    } else {
                        None
                    }
                })
                .unwrap()
                .clone()
        })
        .fold(String::new(), |number, x| {
            format!("{}{}", number, mapping.get(&x).unwrap())
        })
        .parse::<u32>()
        .unwrap();
}

fn calculate_mapping(patterns: &mut HashSet<String>, mapping: &mut HashMap<String, u32>) {
    // 2l -> 1
    // 3l -> 7
    // 4l -> 4
    // 7l -> 8
    // 5l + contains 1 -> 3
    // 6l + contains 4 -> 9
    // 5l + not 3 + contained within 9 -> 5
    // 5l + not 3 + not 5 -> 2
    // 6l + contains 5 + not 9 -> 6
    // 6l + not 9 + not 6 -> 0

    let mut word: String = patterns
        .iter()
        .filter(|x| x.len() == 2)
        .next()
        .unwrap()
        .to_string();
    patterns.remove(word.as_str());
    mapping.insert(word, 1);

    word = patterns
        .iter()
        .filter(|x| x.len() == 3)
        .next()
        .unwrap()
        .to_string();
    patterns.remove(word.as_str());
    mapping.insert(word, 7);

    word = patterns
        .iter()
        .filter(|x| x.len() == 4)
        .next()
        .unwrap()
        .to_string();
    patterns.remove(word.as_str());
    mapping.insert(word, 4);

    word = patterns
        .iter()
        .filter(|x| x.len() == 7)
        .next()
        .unwrap()
        .to_string();
    patterns.remove(word.as_str());
    mapping.insert(word, 8);

    word = patterns
        .iter()
        .filter(|x| x.len() == 5 && contains_chars(x, &get_word(1, &mapping)))
        .next()
        .unwrap()
        .to_string();
    patterns.remove(word.as_str());
    mapping.insert(word, 3);

    word = patterns
        .iter()
        .filter(|x| x.len() == 6 && contains_chars(x, &get_word(4, &mapping)))
        .next()
        .unwrap()
        .to_string();
    patterns.remove(word.as_str());
    mapping.insert(word, 9);

    word = patterns
        .iter()
        .filter(|x| x.len() == 5 && contains_chars(&get_word(9, &mapping), x))
        .next()
        .unwrap()
        .to_string();
    patterns.remove(word.as_str());
    mapping.insert(word, 5);

    word = patterns
        .iter()
        .filter(|x| x.len() == 5)
        .next()
        .unwrap()
        .to_string();
    patterns.remove(word.as_str());
    mapping.insert(word, 2);

    word = patterns
        .iter()
        .filter(|x| x.len() == 6 && contains_chars(x, &get_word(5, &mapping)))
        .next()
        .unwrap()
        .to_string();
    patterns.remove(word.as_str());
    mapping.insert(word, 6);

    word = patterns
        .iter()
        .filter(|x| x.len() == 6)
        .next()
        .unwrap()
        .to_string();
    patterns.remove(word.as_str());
    mapping.insert(word, 0);
}

fn contains_chars(str1: &String, str2: &String) -> bool {
    HashSet::<char>::from_iter(str1.chars()).is_superset(&HashSet::<char>::from_iter(str2.chars()))
}

fn get_word(num: u32, mapping: &HashMap<String, u32>) -> String {
    mapping
        .iter()
        .find_map(|(key, &val)| if val == num { Some(key) } else { None })
        .unwrap()
        .clone()
}
