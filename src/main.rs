use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

mod common;
mod d1;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;
mod d7;
mod d8;

const LATEST_DAY: &str = "8";

fn read_file(path: &str) -> BufReader<File> {
    let file = File::open(path).unwrap();
    BufReader::new(file)
}

fn main() {
    let d = env::args().nth(1).or(Some(LATEST_DAY.into())).unwrap();

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

        "4" => {
            // Day-4
            let input = read_file("input-d4");
            d4::run(input.lines());
        }

        "5" => {
            // Day-5
            let input = read_file("input-d5");
            d5::run(input.lines());
        }

        "6" => {
            // Day-6
            let input = read_file("input-d6");
            d6::run(input.lines());
        }

        "7" => {
            // Day-7
            let input = read_file("input-d7");
            d7::run(input.lines());
        }

        "8" => {
            // Day-8
            let input = read_file("input-d8");
            d8::run(input.lines());
        }

        _ => unimplemented!(),
    }
}
