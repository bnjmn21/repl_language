use clap::Parser;

mod regex;

/*
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::fs;
*/

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    file: String,
}

fn main() {
    //let args = Cli::parse();
    //let teststr = String::from("Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.");
    let teststr = String::from("test tests");
    let findstr = String::from("sus[ s]");
    let flags = String::from("");
    let testres = regex::find(teststr, findstr, flags);
    println!("{:?}", testres);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_single_find() {
        let str1 = String::from("hello this is me!");
        let str2 = String::from(" is ");
        let str3 = String::from("");
        assert_eq!(regex::find(str1,str2,str3),
            Some(vec![regex::FindPos{start:10,end:13}]));
    }

    #[test]
    fn simple_single_find_i() {
        let str1 = String::from("hello this Is me!");
        let str2 = String::from(" is ");
        let str3 = String::from("gi");
        assert_eq!(regex::find(str1,str2,str3),
        Some(vec![regex::FindPos{start:10,end:13}]));
    }

    #[test]
    fn simple_find() {
        let str1 = String::from("hello this is me!");
        let str2 = String::from("is");
        let str3 = String::from("g");
        assert_eq!(regex::find(str1,str2,str3),
        Some(vec![regex::FindPos{start:8,end:9},regex::FindPos{start:11,end:12}]));
    }

    #[test]
    fn simple_find_single() {
        let str1 = String::from("hello this is me!");
        let str2 = String::from("is");
        let str3 = String::from("");
        assert_eq!(regex::find(str1,str2,str3),
        Some(vec![regex::FindPos{start:8,end:9}]));
    }

    #[test]
    fn find_single_char_of() {
        let str1 = String::from("hello this is me!");
        let str2 = String::from("h[ei]");
        let str3 = String::from("g");
        assert_eq!(regex::find(str1,str2,str3),
        Some(vec![regex::FindPos{start:0,end:1},regex::FindPos{start:7,end:8}]));
    }

    #[test]
    fn find_single_char_of_not() {
        let str1 = String::from("hi! hi!xd hi!! hi!uwu");
        let str2 = String::from("hi![^ x]");
        let str3 = String::from("g");
        assert_eq!(regex::find(str1,str2,str3),
        Some(vec![regex::FindPos{start:10,end:13},regex::FindPos{start:15,end:18}]));
    }

    #[test]
    fn find_single_char_in_range() {
        let str1 = String::from("hckushabcuoiscabsaedfabduoi");
        let str2 = String::from("[a-c][b-d][b-e]");
        let str3 = String::from("g");
        assert_eq!(regex::find(str1,str2,str3),
            Some(vec![regex::FindPos{start:6,end:8},regex::FindPos{start:21,end:23}]));
    }

    #[test]
    fn find_single_char_not_in_range() {
        let str1 = String::from("abcatacbcbcata");
        let str2 = String::from("[^a-c]");
        let str3 = String::from("g");
        assert_eq!(regex::find(str1,str2,str3),
            Some(vec![regex::FindPos{start:4,end:4},regex::FindPos{start:12,end:12}]));
    }

    #[test]
    fn find_single_char_in_multi_range() {
        let str1 = String::from("abcdefg");
        let str2 = String::from("[a-ce-g]");
        let str3 = String::from("g");
        assert_eq!(regex::find(str1,str2,str3),
            Some(vec![regex::FindPos{start:0,end:0},regex::FindPos{start:1,end:1},regex::FindPos{start:2,end:2},regex::FindPos{start:4,end:4},regex::FindPos{start:5,end:5},regex::FindPos{start:6,end:6}]));
    }
}