extern crate colored;
use colored::*;
use std::fs::File;
use std::io;
use std::io::BufRead;

trait Balanced {
    /// Returns true if the brackets are balanced
    fn count_braces(&self) -> i64;
}

impl<'a> Balanced for str {
    fn count_braces(&self) -> i64 {
        let mut count = 0;

        for bracket in self.chars() {
            let change = match bracket {
                '(' => 1,
                ')' => -1,
                _ => 0,
            };

            count += change;
        }

        count
    }
}

fn count_newlines(s: &str) -> usize {
    s.as_bytes().iter().filter(|&&c| c == b'\n').count()
}

#[allow(dead_code)]
#[allow(unused_assignments)]
fn main() -> io::Result<()> {
    let f = File::open("foo.txt")?;
    let mut result = String::new();
    let call_line = 19;
    let mut start_loc: i64 = call_line - 2;

    if start_loc <= 0 {
        start_loc = 1
    }

    let lines = io::BufReader::new(&f).lines();
    let mut vec_lines = Vec::new();
    vec_lines = lines.collect();
    let file_length = vec_lines.len() as i64;

    let mut position = start_loc;
    let mut start_coloring = false;
    let mut paranthesis_count = 0;
    let code_frame = &vec_lines[(start_loc - 1) as usize..];
    for line in code_frame {
        if let Ok(ip) = line {
            let mut formatted_line = format!("{} | {}", position, ip);
            if position == call_line {
                paranthesis_count += ip.count_braces();
                if paranthesis_count != 0 {
                    start_coloring = true;
                }
                formatted_line = formatted_line.red().to_string();
            } else if start_coloring {
                paranthesis_count += ip.count_braces();
                if paranthesis_count == 0 {
                    start_coloring = false;
                }
                formatted_line = formatted_line.red().to_string();
            } else {
                if count_newlines(&result) >= 5 {
                    break;
                }
                formatted_line = formatted_line.dimmed().to_string();
                if position > file_length {
                    println!("{} - {}", position, file_length);
                    break;
                }
            }
            result.push_str(formatted_line.as_str());
            result.push_str("\n");
        }
        position += 1;

        // if result.len() >= 5 {
        //     break;
        // }
    }

    println!("{}", result);
    Ok(())
}
