use std::io::{Lines, BufRead};

pub fn parse(lines: Lines<impl BufRead>) -> Vec<String> {
    let mut vs = vec![];
    for line in lines {
        let l = line.unwrap();
        if l == "" {
            break;
        }
        vs.push(l)
    }
    println!("len={}", vs.len());
    vs
}
