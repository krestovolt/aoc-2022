use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

mod common;
mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;
mod d09;
mod d10;
mod d11;

const LATEST_DAY: &str = "11";

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
            d01::run(input.lines());
        }

        "2" => {
            // Day-2
            let input = read_file("input-d2");
            d02::run(input.lines());
        }

        "3" => {
            // Day-3
            let input = read_file("input-d3");
            d03::run(input.lines());
        }

        "4" => {
            // Day-4
            let input = read_file("input-d4");
            d04::run(input.lines());
        }

        "5" => {
            // Day-5
            let input = read_file("input-d5");
            d05::run(input.lines());
        }

        "6" => {
            // Day-6
            let input = read_file("input-d6");
            d06::run(input.lines());
        }

        "7" => {
            // Day-7
            let input = read_file("input-d7");
            d07::run(input.lines());
        }

        "8" => {
            // Day-8
            let input = read_file("input-d8");
            d08::run(input.lines());
        }

        "9" => {
            // Day-9
            let input = read_file("input-d9");
            d09::run(input.lines());
        }

        "10" => {
            // Day-10
            // let input = read_file("input-d10-sample");
            let input = read_file("input-d10");
            d10::run(input.lines());
        }

        "11" => {
            // Day-11
            // let input = read_file("input-d11-sample");
            let input = read_file("input-d11");
            d11::run(input.lines());
        }

        _ => unimplemented!(),
    }
}
