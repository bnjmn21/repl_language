#[derive(Debug, PartialEq, Copy, Clone)]
pub struct FindPos {
    pub start: u32,
    pub end: u32,
}

fn char_range(start: char, end: char) -> Option<Vec<char>>{
    let chars = vec!['0','1','2','3','4','5','6','7','8','9',
        'a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z',
        'A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];
    if start.is_ascii_digit() && end.is_ascii_digit() {
        let startpos = start.to_digit(10).unwrap();
        let endpos = end.to_digit(10).unwrap()+1;
        println!("{:?} {:?}",startpos,endpos);
        return Some(chars[startpos as usize.. endpos as usize].to_vec())
    } else if start.is_ascii_alphabetic() && end.is_ascii_alphabetic() {
        if start.is_ascii_lowercase() && end.is_ascii_lowercase() {
            let startpos = start.to_digit(36).unwrap();
            let endpos = end.to_digit(36).unwrap()+1;
            return Some(chars[startpos as usize.. endpos as usize].to_vec())
        } else {
            let startpos = start.to_digit(36).unwrap()+26;
            let endpos = end.to_digit(36).unwrap()+27;
            return Some(chars[startpos as usize.. endpos as usize].to_vec())
        }
    } else {return None}
}

fn check_at(find_str: String, regex: String, start_pos: u32, case_insensitive: bool) -> Option<u32>{
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
        if rgxlist.get(rgxi) == None {return Some(stri as u32 + start_pos)}
        match rgxlist.get(rgxi) {
            Some(&'[') => {
                let mut matchlist: Vec<char> = Vec::new();
                let mut inverted = false;
                loop{
                    rgxi +=1;
                    match rgxlist.get(rgxi) {
                        None => return None,
                        Some(&']') => break,
                        Some(&'^') => {
                            if matchlist.is_empty() {inverted = true;}
                            else {matchlist.push('^')}
                        },
                        Some(&'-') => {
                            if rgxlist.get(rgxi+1) == None {return None}
                            matchlist.append(&mut char_range(rgxlist[rgxi-1],rgxlist[rgxi+1]).unwrap_or(Vec::new()));
                            rgxi += 1;
                        }
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
                if matchlist.contains(&strlist[stri]) != inverted {stri+=1} 
                else {return None}
            },
            None => {
                if rgxlist.get(rgxi) == None {return Some(stri as u32 + start_pos)}
                else {return None}
            },
            _ => {
                if strlist[stri] == rgxlist[rgxi] {
                    rgxi += 1;
                    stri += 1;
                } else {return None}
            },
        }
    }
}

pub fn find(find_str: String, regex: String, flagstr: String) -> Option<Vec<FindPos>> {
    let flags: Vec<char> = flagstr.chars().collect();

    let mut i = 0;
    let mut res:Vec<FindPos> = Vec::new();

    for _c in find_str.chars() {
        let checkres = check_at(find_str.clone(), regex.clone(), i, flags.contains(&'i'));
        match checkres {
            None => {},
            Some(e) => res.push(FindPos{start:i, end: e-1})
        };
        i += 1;
    }
    if res.is_empty() {return None}
    if !flags.contains(&'g') {return Some(vec![res[0]])}
    return Some(res)
}