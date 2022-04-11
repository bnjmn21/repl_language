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

#[derive(Debug)]
pub enum FindResult {
    Fail,
    Success{pos: FindPos}
}

pub fn check_at(find_str: String, regex: String, start_pos: u32) -> CheckResult{
    let strlist: Vec<char> = find_str.split_at(start_pos as usize).1.chars().collect();
    let rgxlist: Vec<char> = regex.chars().collect();

    let mut i = 0;

    loop{
        if rgxlist.get(i) == None {return CheckResult::Success { end:i as u32 + start_pos}}
        if strlist.get(i) == None {return CheckResult::Fail}
        match strlist.get(i) {
            None => {
                if rgxlist.get(i) == None {return CheckResult::Success { end:i as u32 + start_pos}}
                else {return CheckResult::Fail}
            }
            _ => {
                if strlist[i] == rgxlist[i] {} else {return CheckResult::Fail}
            }
        }
        i += 1;
    }
}

pub fn find(find_str: String, regex: String, flagstr: String) -> FindResult {
    let flags: Vec<char> = flagstr.chars().collect();

    let mut i = 0;
    let mut res = FindResult::Fail;

    for c in find_str.chars() {
        let checkres = check_at(find_str.clone(), regex.clone(), i);
        match checkres {
            CheckResult::Fail => {res = res},
            CheckResult::Success{ end: e } => res = FindResult::Success { pos: FindPos{start:i, end:e-1}}
        };
        i += 1;
    }
    return res;
}