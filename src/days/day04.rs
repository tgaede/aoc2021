pub fn solve(input: &str) {
    test_part1();
    solve_part1(input);

    test_part2();
    solve_part2(input);
}

fn test_part1() {
    solve_part1(
		"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\n22 13 17 11  0\n 8  2 23  4 24\n21  9 14 16  7\n 6 10  3 18  5\n 1 12 20 15 19\n\n 3 15  0  2 22\n 9 18 13 17  5\n19  8  7 25 23\n20 11 10 24  4\n14 21 16 12  6\n\n14 21 17 24  4\n10 16 15  9 19\n18  8 23 26 20\n22 11 13  6  5\n 2  0 12  3  7"
	);
}

fn solve_part1(input: &str) -> u32 {
    let mut parts = input.trim().split("\n\n");
    let numbers: Vec<u32> = parts
        .next()
        .unwrap()
        .split(",")
        .map(|c| c.parse::<u32>().unwrap())
        .collect();
    let mut boards: Vec<Vec<Vec<(u32, u32)>>> = parts
        .map(|board_block| {
            board_block
                .split("\n")
                .map(|board_line| {
                    board_line
                        .split_whitespace()
                        .map(|board_item| (board_item.parse::<u32>().unwrap(), 0u32))
                        .collect::<Vec<_>>()
                })
                .collect()
        })
        .collect();

    let mut result: u32 = 0;
    'outer: for num in numbers {
        for board in boards.iter_mut() {
            draw_number(num, board);
        }

        for board in boards.iter() {
            if has_bingo(board) {
                result = num * calculate_score(board);
                break 'outer;
            }
        }
    }

    println!("part 1 result: {}", result);
    return result;
}

fn draw_number(num: u32, board: &mut Vec<Vec<(u32, u32)>>) {
    for row in board {
        for (board_num, is_found) in row {
            if num == *board_num {
                *is_found = 1;
                return;
            }
        }
    }
}

fn has_bingo(board: &Vec<Vec<(u32, u32)>>) -> bool {
    // check rows
    let has_bingo_row: bool = board
        .iter()
        .any(|row| row.iter().all(|(_num, is_found)| *is_found == 1));

    let mut has_bingo_column: bool = false;
    for col in 0..board.first().unwrap().len() {
        has_bingo_column = true;
        for row in 0..board.len() {
            has_bingo_column &= (*(board.get(row).unwrap().get(col).unwrap())).1 == 1
        }

        if has_bingo_column == true {
            break;
        }
    }

    return has_bingo_row | has_bingo_column;
}

fn calculate_score(board: &Vec<Vec<(u32, u32)>>) -> u32 {
    let mut score: u32 = 0;
    for row in board.iter() {
        for (num, is_marked) in row.iter() {
            if *is_marked == 0 {
                score += num;
            }
        }
    }
    return score;
}

fn test_part2() {
    solve_part2(
		"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\n22 13 17 11  0\n 8  2 23  4 24\n21  9 14 16  7\n 6 10  3 18  5\n 1 12 20 15 19\n\n 3 15  0  2 22\n 9 18 13 17  5\n19  8  7 25 23\n20 11 10 24  4\n14 21 16 12  6\n\n14 21 17 24  4\n10 16 15  9 19\n18  8 23 26 20\n22 11 13  6  5\n 2  0 12  3  7"
	);
}

fn solve_part2(input: &str) -> u32 {
    let mut parts = input.trim().split("\n\n");
    let numbers: Vec<u32> = parts
        .next()
        .unwrap()
        .split(",")
        .map(|c| c.parse::<u32>().unwrap())
        .collect();
    let mut boards: Vec<Vec<Vec<(u32, u32)>>> = parts
        .map(|board_block| {
            board_block
                .split("\n")
                .map(|board_line| {
                    board_line
                        .split_whitespace()
                        .map(|board_item| (board_item.parse::<u32>().unwrap(), 0u32))
                        .collect::<Vec<_>>()
                })
                .collect()
        })
        .collect();

    let mut board_results: Vec<u32> = vec![0; boards.len()];
    let mut result: u32 = 0;
    for num in numbers {
        for board in boards.iter_mut() {
            draw_number(num, board);
        }

        for (i, board) in boards.iter().enumerate() {
            if *board_results.get(i).unwrap() == 0 && has_bingo(board) {
                result = calculate_score(board) * num;
                *board_results.get_mut(i).unwrap() = result;
                println!("board: {} won with number {}, result={}", i, num, result);
            }
        }
    }

    println!("part 2 last result: {}", result);
    return result;
}
