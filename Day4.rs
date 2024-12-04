//Day 4 prob1
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_to_string<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    reader.lines().collect()
}

fn count_occurrences(grid: &Vec<String>, word: &str) -> u32 {
    let directions = [
        (0, 1),   // Right
        (1, 0),   // Down
        (1, 1),   // Diagonal down-right
        (1, -1),  // Diagonal down-left
        (0, -1),  // Left
        (-1, 0),  // Up
        (-1, -1), // Diagonal up-left
        (-1, 1),  // Diagonal up-right
    ];

    let rows = grid.len();
    let cols = grid[0].len();
    let word_len = word.len();
    let mut count = 0;

    for i in 0..rows {
        for j in 0..cols {
            for &(dx, dy) in &directions {
                let mut found = true;
                for k in 0..word_len {
                    let ni = i as isize + k as isize * dx;
                    let nj = j as isize + k as isize * dy;

                    if ni < 0
                        || nj < 0
                        || ni >= rows as isize
                        || nj >= cols as isize
                        || grid[ni as usize].chars().nth(nj as usize).unwrap() != word.chars().nth(k).unwrap()
                    {
                        found = false;
                        break;
                    }
                }
                if found {
                    count += 1;
                }
            }
        }
    }

    count
}

fn main() {
    let contents = read_to_string("H:/Rust/Practise/src/text.txt").unwrap();
    let total_occurrences = count_occurrences(&contents, "XMAS");
    println!("Total occurrences of 'XMAS': {}", total_occurrences);
}

//Day4 prob 2
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_to_string<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    reader.lines().collect()
}
fn count_occurrences(grid: &Vec<String>) -> u32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for i in 1..rows-1{
        for j in 1..cols-1{
            if grid[i].chars().nth(j).unwrap() != 'A'{
                continue;
            }
            if grid[i-1].chars().nth(j-1).unwrap() == 'M'  && grid[i+1].chars().nth(j+1).unwrap() == 'S'{
                if grid[i+1].chars().nth(j-1).unwrap() == 'M' && grid[i-1].chars().nth(j+1).unwrap() == 'S'{
                    count += 1;
                    continue;
                }
                else if grid[i+1].chars().nth(j-1).unwrap() == 'S' && grid[i-1].chars().nth(j+1).unwrap() == 'M'{
                    count += 1;
                    continue;
                }
            }
            else if grid[i-1].chars().nth(j-1).unwrap() == 'S'  && grid[i+1].chars().nth(j+1).unwrap() == 'M'{
                if grid[i+1].chars().nth(j-1).unwrap() == 'S' && grid[i-1].chars().nth(j+1).unwrap() == 'M'{
                    count += 1;
                    continue;
                }
                else if grid[i+1].chars().nth(j-1).unwrap() == 'M' && grid[i-1].chars().nth(j+1).unwrap() == 'S'{
                    count += 1;
                    continue;
                }
            }

        }
    }

    count
}

fn main() {
    let str = "MAMXSAMXMXMMMMSMMXMXMASMMSMMSSSMAXXMSMSSXMASMXSSSXSMMSSSSSXMASXSAMXSMMSSMXXXXXMASXMXMSSSMMSSMSSSMMMMSSXM";
    let length: u32 = str.chars().count() as u32;
    println!("{}", length);
    let contents = read_to_string(r"H:\Rust\Practise\src\text.txt").unwrap();

    for i in &contents {
        println!("{:#?}", i);
    }
    println!("{:?}", count_occurrences(&contents));
}


