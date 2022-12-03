use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

mod common;
mod d1;
mod d2;
mod d3;

fn read_file(path: &str) -> BufReader<File> {
    let file = File::open(path).unwrap();
    BufReader::new(file)
}

fn main() {
    let d = env::args().nth(1).or(Some("1".into())).unwrap();

    println!("Running Day-{}", d);
    println!("{:=>10}", "");

    match d.as_str() {
        "1" => {
            // Day-1
            let input = read_file("input-d1");
            d1::run(input.lines());
        }

        "2" => {
            // Day-2
            let input = read_file("input-d2");
            d2::run(input.lines());
        }

        "3" => {
            // Day-3
            let input = read_file("input-d3");
            d3::run(input.lines());
        }

        _ => unimplemented!(),
    }
}
