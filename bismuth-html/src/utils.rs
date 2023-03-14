use std::path::PathBuf;

pub fn get_dot_number(path: &PathBuf) -> usize {
    let mut path = path.clone();

    let mut dot_number: usize = 0;
    while path.pop() == true {
        dot_number += 1_usize;
    }
    let dot_number = dot_number.checked_sub(1).unwrap_or(dot_number);
    dot_number
}

pub fn get_dots(path: &PathBuf) -> String {
    let mut pre = "../".repeat(get_dot_number(path));
    pre.remove(pre.len() - 1);
    pre
}
