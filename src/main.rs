use regex::{Regex, Matches};
use clap::Parser;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::fs;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    file: String,
}

fn main() {
    let args = Cli::parse();
    let mut code_line = 0u32;

    let mut isRunning = true;

    while isRunning {
        let mut file = File::open(&args.file).unwrap();
        let mut filestr = String::new();
        file.read_to_string(&mut filestr).unwrap();

        let mut lines : Vec<&str>= filestr.lines().collect();
        
        if lines.get(code_line as usize) == None {code_line = 0}
        else if lines[code_line as usize] == "SUCESS" {return}
        else if lines[code_line as usize].starts_with('!') {
            let checkx = Regex::new().unwrap();

            fs::write(&args.file, );
        }

    }
}