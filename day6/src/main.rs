use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");
    let result = solve_part2(&input);
    println!("{}", result);
}

fn solve_part2(input: &str) -> i64 {
    let lines: Vec<&str> = input.lines().collect();
    let max_len = lines.iter().map(|line| line.len()).max().unwrap_or(0);
    
    // Create padded grid
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let mut chars: Vec<char> = line.chars().collect();
        chars.resize(max_len, ' ');
        grid.push(chars);
    }
    
    let rows = grid.len();
    let cols = max_len;

    let mut separator = vec![true; cols];
    for c in 0..cols {
        for r in 0..rows {
            if grid[r][c] != ' ' {
                separator[c] = false;
                break;
            }
        }
    }

    let mut problems: Vec<Vec<usize>> = Vec::new();
    let mut current_problem: Vec<usize> = Vec::new();
    
    for c in 0..cols {
        if separator[c] {
            if !current_problem.is_empty() {
                problems.push(current_problem);
                current_problem = Vec::new();
            }
        } else {
            current_problem.push(c);
        }
    }
    
    if !current_problem.is_empty() {
        problems.push(current_problem);
    }
    
    let mut total = 0i64;
    
    for prob_cols in problems {
        let mut op = ' ';
        for &c in &prob_cols {
            let ch = grid[rows - 1][c];
            if ch != ' ' {
                op = ch;
                break;
            }
        }

        let mut numbers: Vec<i64> = Vec::new();
        for &c in prob_cols.iter().rev() {
            let mut digits = String::new();
            for r in 0..rows - 1 {
                let ch = grid[r][c];
                if ch != ' ' {
                    digits.push(ch);
                }
            }
            
            if !digits.is_empty() {
                numbers.push(digits.parse().unwrap());
            } else {
                numbers.push(0);
            }
        }

        let result = match op {
            '+' => numbers.iter().fold(0i64, |acc, &x| acc + x),
            '*' => numbers.iter().fold(1i64, |acc, &x| acc * x),
            _ => panic!("Unknown operator: {}", op),
        };
        
        total += result;
    }
    
    total
}
