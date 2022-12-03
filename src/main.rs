use std::{fs::File, io::{BufReader, BufRead}};

mod d1;
// mod d2;
mod d3;

fn read_file(path: &str) -> BufReader<File> {
    let file = File::open(path).unwrap();
    BufReader::new(file)
}

fn main() {
    // Day-1
    // let input = read_file("input-d1");
    // d1::run(input.lines());

    // Day-3
    let input = read_file("input-d3");
    d3::run(input.lines());
}
