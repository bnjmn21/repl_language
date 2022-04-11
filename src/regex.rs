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

#[derive(Debug)]
pub enum FindResult {
    Fail,
    Success{pos: Vec<FindPos>},
}

pub fn check_at(find_str: String, regex: String, start_pos: u32, case_insensitive: bool) -> CheckResult{
    let strlist: Vec<char>;
    let rgxlist: Vec<char>;
    if case_insensitive {
        strlist = find_str.to_lowercase().to_string().split_at(start_pos as usize).1.chars().collect();
        rgxlist = regex.to_lowercase().to_string().chars().collect();
    } else {
        strlist = find_str.split_at(start_pos as usize).1.chars().collect();
        rgxlist = regex.chars().collect();
    }

    let mut rgxi = 0;
    let mut stri = 0;

    loop{
        if rgxlist.get(rgxi) == None {return CheckResult::Success { end:stri as u32 + start_pos}}
        if strlist.get(stri) == None {return CheckResult::Fail}
        //TODO check if still works without ^^^
        match rgxlist.get(rgxi) {
            Some(&'[') => {
                let mut matchlist: Vec<char> = Vec::new();
                loop{
                    rgxi +=1;
                    match rgxlist.get(rgxi) {
                        Some(&']') => break,
                        None => return CheckResult::Fail,
                        _ => {
                            match rgxlist.get(rgxi+1) {
                                Some(&'-') => {},
                                None => {},
                                _ => {matchlist.push(rgxlist[rgxi])}
                            }
                        }
                    }
                }
                rgxi +=1;
                match matchlist.contains(&strlist[stri]) {
                    true => stri+=1,
                    false => return CheckResult::Fail
                }
            },
            None => {
                if rgxlist.get(rgxi) == None {return CheckResult::Success { end:stri as u32 + start_pos}}
                else {return CheckResult::Fail}
            },
            _ => {
                if strlist[stri] == rgxlist[rgxi] {
                    rgxi += 1;
                    stri += 1;
                } else {return CheckResult::Fail}
            },
        }
    }
}

pub fn find(find_str: String, regex: String, flagstr: String) -> FindResult {
    let flags: Vec<char> = flagstr.chars().collect();

    let mut i = 0;
    let mut res:Vec<FindPos> = Vec::new();

    for _c in find_str.chars() {
        let checkres = check_at(find_str.clone(), regex.clone(), i, flags.contains(&'i'));
        match checkres {
            CheckResult::Fail => {},
            CheckResult::Success{ end: e } => res.push(FindPos{start:i, end: e-1})
        };
        i += 1;
    }
    if res.is_empty() {return FindResult::Fail}
    if flags.contains(&'s') {return FindResult::Success { pos: vec![res[0]] }}
    return FindResult::Success { pos: res }
}