use regex::{Regex, Matches};
use clap::Parser;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::fs;

mod codereader;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    file: String,
}

fn main() {
    let args = Cli::parse();
    }
}