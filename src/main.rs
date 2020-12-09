use std::io;
use std::env;
use std::fs;

fn get_file_args() -> Vec<String> {
    let v: Vec<String> = env::args().collect();
    v
}

fn read_file(filename: &str) -> Result<String, io::Error> {
    let contents = fs::read_to_string(filename).expect("Couldn't read file");
    Ok(contents)
}

fn match_string(line: &str, needle: &str) -> bool {
      line.contains(needle)
}

fn main() {
    let args = get_file_args();
    if args.len() < 3 {
        println!("rustgrep [filename] [needle]");
        return;
    }

    let query = &args[1];
    let needle = &args[2];

    let haystack = read_file(query).unwrap_or_else(|error| {
        panic!(error);
    });

    let lines: Vec<&str> = haystack.split("\n").collect();
    for i in 0..lines.len() {
        if match_string(lines[i], needle) {
            println!("line {}: {}", i+1, lines[i]);
        }
    }
}
