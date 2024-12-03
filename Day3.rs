//Problem 1
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

fn read_to_string<P>(filename: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn is_valid_mul_instruction(s: &str) -> bool {
    s.starts_with("mul(") &&
        s.ends_with(")") &&
        s.chars().filter(|&c| c == ',').count() == 1
}

fn extract_numbers(s: &str) -> Option<(i32, i32)> {
    // Extract numbers from a valid mul instruction
    let trimmed = s.trim_start_matches("mul(").trim_end_matches(")");
    let parts: Vec<&str> = trimmed.split(',').collect();

    if parts.len() != 2 {
        return None;
    }

    match (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
        (Ok(x), Ok(y)) => Some((x, y)),
        _ => None
    }
}

fn solve(input: &str) -> i32 {
    let mut total = 0;

    // Slide a window through the input to find mul instructions
    for i in 0..input.len() {
        for j in i+1..=input.len() {
            let substr = &input[i..j];

            if is_valid_mul_instruction(substr) {
                if let Some((x, y)) = extract_numbers(substr) {
                    total += x * y;
                }
            }
        }
    }

    total
}

fn main() {
    let contents = read_to_string(r"H:\Rust\Practise\src\text.txt").unwrap();
    println!("Total: {}", solve(contents.as_str()));
}

//Problem 2
