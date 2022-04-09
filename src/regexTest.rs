#[derive(Debug)]
pub struct FindResult {
    start: u32,
    end: u32,
}

pub fn find(input: String, rgx: String, flags: Vec<char>) -> Vec<FindResult> {
    let chars:Vec<char>= input.chars().collect();
    let rgx_char:Vec<char> = rgx.chars().collect();
    let mut res:Vec<FindResult> = Vec::new();
    if rgx_char.is_empty() {
        res =  vec![FindResult{start: 0, end: (chars.len() as u32)-1}];
    }
    let i = 0u32;
    for c in &chars {
        if c == &rgx_char[i as usize] {
            let mut is_checking = true;
            let mut check_i = i as usize;
            while is_checking {
                println!("{:?} {:?}",chars.get(check_i),rgx_char.get((check_i as u32 -i) as usize));
                if chars.get(check_i) == None {
                    return res;
                } else if rgx_char.get((check_i as u32 -i) as usize) == None {
                    res.push(FindResult{start: i, end: check_i as u32});
                    if flags.contains(&'s') {
                        return res;
                    } else {
                        is_checking = false;
                    }
                } else if &chars[check_i] == &rgx_char[(check_i as u32 -i) as usize] {
                    check_i += 1;
                } else {
                    is_checking = false;
                }
            }
        }
    }
    return res;
}