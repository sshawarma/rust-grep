use std::io;
use std::env;
use std::fs;

extern crate regex;
use regex::Regex;

struct Line {
    n: usize,
    line: String,
    start: usize,
    end: usize,
}

fn get_file_args() -> Vec<String> {
    let v: Vec<String> = env::args().collect();
    v
}

fn read_file(filename: &str) -> Result<String, io::Error> {
    let contents = fs::read_to_string(filename).expect("Couldn't read file");
    Ok(contents)
}

fn get_lines(haystack: &str) -> Vec<Line> {
    let mut lines: Vec<Line> = Vec::new();
    let bytes: &[u8] = haystack.as_bytes();

    let mut n = 1;
    let mut j = 0;
    for i in 0..bytes.len() {
        if bytes[i] as char == '\n' {
            let l = Line {
                n,
                line: haystack[j..i].to_string(),
                start: j,
                end: i,
            };
            lines.push(l);
            j = i+1;
            n += 1;
        }
    }
    lines
}

fn get_line(lines: &Vec<Line>, start: usize, end: usize) -> &Line {
    let mut s = &lines[0];
    for line in lines.iter() {
        if start >= line.start && end <= line.end {
            s = line;
            break;
        }
    }
    s
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
    let lines = get_lines(&haystack);

    let re = Regex::new(needle).unwrap_or_else(|error| {
        panic!(error);
    });
    let matches = re.find_iter(&haystack);
    let mut processed: Vec<usize> = Vec::new();
    for m in matches {
        let l = get_line(&lines, m.start(), m.end());
        if processed.iter().any(|&n| n == l.n) {
            continue
        } else {
            println!("line {}: {}", l.n, l.line);
            processed.push(l.n);
        }
    }
}
