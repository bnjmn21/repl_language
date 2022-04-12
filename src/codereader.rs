use regex::Regex;

fn doLine(line_index: u32, str: String) {
    let lineopt = str.lines().nth(line_index);
    if lineopt == None {return}
    else {
        let line = lineopt.unwrap();
        if line.starts_with('<') {

        }
    }
}

fn rgxOp(str: &str, rgx: &str, sub: &str) -> String {
    let regex = Regex::new(rgx).unwrap();
    return regex.replace_all(str, sub)
}