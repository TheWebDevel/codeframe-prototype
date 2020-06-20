extern crate colored;
use colored::*;
use std::fs::File;
use std::io;
use std::io::BufRead;

#[allow(dead_code)]
#[allow(unused_assignments)]
fn main() -> io::Result<()> {
    let f = File::open("foo.txt")?;
    let mut result = String::new();
    let call_line = 19;
    let mut start_loc: i64 = call_line - 2;
    let mut end_loc: i64 = call_line + 3;

    if start_loc <= 0 {
        start_loc = 1
    }

    let lines = io::BufReader::new(&f).lines();
    let mut vec_lines = Vec::new();
    vec_lines = lines.collect();
    let file_length = vec_lines.len() as i64;
    if end_loc > file_length {
        end_loc = file_length;
    }

    println!("\n====================");
    println!(
        "File Length: {file_length}
Line: {line}

Code Frame:
Start Location: {start_loc}
End Location: {end_loc}
    ",
        file_length = file_length,
        line = call_line,
        start_loc = start_loc,
        end_loc = end_loc,
    );
    println!("====================");

    let mut position = start_loc;
    let code_break = &vec_lines[(start_loc - 1) as usize..(end_loc) as usize];
    for line in code_break {
        if let Ok(ip) = line {
            let mut formatted_line = format!("{} | {}", position, ip);
            if position == call_line {
                formatted_line = formatted_line.red().to_string();
            } else {
                formatted_line = formatted_line.dimmed().to_string();
            }
            result.push_str(formatted_line.as_str());
            result.push_str("\n");
        }
        position += 1;
    }

    println!("{}", result);
    Ok(())
}
