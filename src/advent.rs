// day_1.rs
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn read_file() -> BufReader<File> {
    let path = Path::new("./src/puzzle_input.txt");
    let file = File::open(path).expect("Could not open file");
    let puzzle_input = BufReader::new(file);
    return puzzle_input;
}

fn if_left(s: &str) -> bool {
    // Functionality to turn left or right will go here
    if s.contains("L") {
        return true;
    } else {
        return false;
    }
}

fn turn_dial(s: &str) -> i32 {
    // Functionality to turn the dial will go here
    let turns: i32 = s[1..].parse().expect("Could not parse number");
    turns
}

pub fn decode() -> u16 {
    let puzzle_input = read_file();
    // Decoding logic will go here

    let mut turns: i32 = 50;
    let mut count: u16 = 0;
    for line in puzzle_input.lines() {
        match line {
            Ok(content) => {
                if if_left(&content) {
                    for _ in 0..turn_dial(&content) {
                        turns -= 1;
                        if turns < 0 {
                            turns = 99;
                        }
                        if turns == 0 {
                            count += 1;
                        }
                    }
                } else {
                    for _ in 0..turn_dial(&content) {
                        turns += 1;
                        if turns > 99 {
                            turns = 0;
                        }
                        if turns == 0 {
                            count += 1;
                        }
                    }
                }
                // if turns<0{
                //     count += 1;
                //     count += (turns.abs()/99) as u16;
                //     turns = (( turns%99) + 99) % 99;
                //     println!("{}", turns);
                // } else if turns > 99 {
                //     count += (turns / 99) as u16;
                //     turns = turns % 99;
                //     println!("{}", turns);
                // }
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    return count;
}
