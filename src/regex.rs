#[derive(Debug, PartialEq, Copy, Clone)]
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

pub fn find(find_str: String, regex: String, flagstr: String) -> Vec<FindPos> {
    let flags: Vec<char> = flagstr.chars().collect();

    let mut i = 0;
    let mut res:Vec<FindPos> = Vec::new();

    for _c in find_str.chars() {
        let checkres = check_at(find_str.clone(), regex.clone(), i);
        match checkres {
            CheckResult::Fail => {},
            CheckResult::Success{ end: e } => res.push(FindPos{start:i, end: e-1})
        };
        i += 1;
    }
    if flags.contains(&'s') {
        if res.get(0) == None {return Vec::new()}
        return vec![res[0]]}
    return res
}