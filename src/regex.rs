#[derive(Debug)]
pub struct FindPos {
    start: u32,
    end: u32,
}

#[derive(Debug)]
pub enum CheckResult {
    Fail,
    Success{end: u32},
}

pub fn check_at(find_str: String, regex: String, start_pos: u32) -> CheckResult{
    let strlist: Vec<char> = find_str.split_at(start_pos as usize).1.chars().collect();
    let rgxlist: Vec<char> = regex.chars().collect();

    let mut i = start_pos as usize;

    for c in strlist.iter() {
        if rgxlist.get(i) == None {return CheckResult::Success { end:i as u32 + start_pos}}
        match c {
            _ => {
                if c == &rgxlist[i] {} else {return CheckResult::Fail}
            }
        }
        println!("{:?} {:?} {:?}", i, strlist[i], rgxlist[i]);
        i += 1;
    }
    return CheckResult::Fail
}