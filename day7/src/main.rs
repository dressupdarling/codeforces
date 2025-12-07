use std::collections::VecDeque;
use std::fs;

fn main() {
    let input = fs::read_to_string("day7input.txt").expect("Failed to read input file");

    let part1_result = solve_part1(&input);
    println!("Part 1: {}", part1_result);

    let part2_result = solve_part2(&input);
    println!("Part 2: {}", part2_result);
}

fn solve_part1(input: &str) -> i64 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    let mut has_beam = vec![vec![false; cols]; rows];
    let mut start_pos = (0, 0);
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == 'S' {
                start_pos = (r, c);
                break;
            }
        }
    }

    let mut queue = VecDeque::new();
    queue.push_back(start_pos);
    has_beam[start_pos.0][start_pos.1] = true;
    
    while let Some((r, c)) = queue.pop_front() {
        let current_c = c;
        let mut current_r = r;
        
        loop {
            current_r += 1;
            if current_r >= rows {
                break;
            }
            if has_beam[current_r][current_c] {
                break;
            }
            
            has_beam[current_r][current_c] = true;
            
            match grid[current_r][current_c] {
                '.' => continue,
                '^' => {
                    if current_c > 0 && !has_beam[current_r][current_c - 1] {
                        queue.push_back((current_r, current_c - 1));
                        has_beam[current_r][current_c - 1] = true;
                    }
                    if current_c + 1 < cols && !has_beam[current_r][current_c + 1] {
                        queue.push_back((current_r, current_c + 1));
                        has_beam[current_r][current_c + 1] = true;
                    }
                    break;
                }
                _ => break,
            }
        }
    }

    let mut split_count = 0;
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == '^' && has_beam[r][c] {
                split_count += 1;
            }
        }
    }
    
    split_count
}

fn solve_part2(input: &str) -> u128 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    let mut start_pos = (0, 0);
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == 'S' {
                start_pos = (r, c);
                break;
            }
        }
    }

    let mut timelines = vec![vec![0u128; cols]; rows];
    timelines[start_pos.0][start_pos.1] = 1;
    for r in 0..rows {
        for c in 0..cols {
            if timelines[r][c] == 0 {
                continue;
            }
            
            let current_timelines = timelines[r][c];
            if r + 1 < rows {
                match grid[r + 1][c] {
                    '.' | 'S' => {
                        timelines[r + 1][c] = timelines[r + 1][c].saturating_add(current_timelines);
                    }
                    '^' => {
                        if c > 0 {
                            timelines[r + 1][c - 1] = timelines[r + 1][c - 1].saturating_add(current_timelines);
                        }
                        if c + 1 < cols {
                            timelines[r + 1][c + 1] = timelines[r + 1][c + 1].saturating_add(current_timelines);
                        }
                    }
                    _ => {
                    }
                }
            }
        }
    }

    let mut total = 0u128;
    for c in 0..cols {
        total = total.saturating_add(timelines[rows - 1][c]);
    }
    
    total
}

