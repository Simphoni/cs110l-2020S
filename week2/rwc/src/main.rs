use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];
    // Your code here :)
    let fd = File::open(filename).unwrap();
    let nchars = fd.metadata().unwrap().len();
    let buf = BufReader::new(fd);
    let mut nlines: usize = 0;
    let mut nwords: usize = 0;
    for it in buf.lines() {
        let line_str = it.unwrap();
        nlines += 1;
        let mut status = false;
        for c in line_str.chars() {
            if !c.is_whitespace() {
                if !status {
                    nwords += 1;
                }
                status = true;
            } else {
                status = false;
            }
        }
    }
    println!("\t{}\t{}\t{}\t{}", nlines, nwords, nchars, filename);
}
