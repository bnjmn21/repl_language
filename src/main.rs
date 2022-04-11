use clap::Parser;

mod regex;

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
    //let args = Cli::parse();
    let teststr = String::from("sussy baka lol");
    let findstr = String::from("baka ");
    let testres = regex::check_at(teststr, findstr, 6);
    println!("{:?}", testres);
}