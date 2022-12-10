use std::io::{BufRead, Lines};

use crate::common;

const CYCLE_INCR: i32 = 40;

enum OpCode {
    Noop,
    Skip,
    FetchAddX(i32),
}

pub fn run(input: Lines<impl BufRead>) {
    let lines = common::parse(input);
    let ops = process(&lines);
    let sum = compute(&ops);
    println!("sum: {}", sum);
}

fn compute(ops: &[OpCode]) -> i32 {
    let mut sum = 0;

    let mut pc = 0;
    let mut reg_x = 1_i32;
    let mut checkpoint = 20;

    let mut sprites: Vec<char> = "###....................................."
        .chars()
        .into_iter()
        .collect();

    let mut pixels = vec!['.'; 40 * 6];

    for op in ops {
        print!("pc={:03} | ", pc);
        match op {
            // During fetch counter is not incremented
            OpCode::FetchAddX(num) => {
                let sm = &mut sprites;
                if reg_x - 1 >= 0 {
                    sm[(reg_x - 1) as usize] = '.';
                }
                if reg_x >=0 {
                    sm[(reg_x + 0) as usize] = '.';
                }
                if ((reg_x + 1) as usize) < sm.len() {
                    sm[(reg_x + 1) as usize] = '.';
                }
                reg_x += num;
                if reg_x - 1 >= 0 {
                    sm[(reg_x - 1) as usize] = '#';
                }
                if reg_x >= 0 {
                    sm[(reg_x + 0) as usize] = '#';
                }
                if ((reg_x + 1) as usize) < sm.len() {
                    sm[(reg_x + 1) as usize] = '#';
                }

                println!("reg_x={}", reg_x);
            }

            OpCode::Noop | OpCode::Skip => {
                let s = &sprites;
                let pm = &mut pixels;
                println!("noop");
                pm[pc as usize] = s[(pc % 40) as usize];
                pc += 1;
            }
        }

        if checkpoint == pc {
            let sig_strength = pc * reg_x;
            checkpoint += CYCLE_INCR;
            sum += sig_strength
        }
    }

    for r in 0..6 {
        for c in 0..40 {
            print!("{}", pixels[c + (r * 40)])
        }
        println!("");
    }

    sum
}

fn process(lines: &[String]) -> Vec<OpCode> {
    let mut ops = vec![];
    for line in lines {
        let spl: Vec<&str> = line.split(" ").collect();
        match spl[0] {
            "noop" => {
                ops.push(OpCode::Noop);
            }

            "addx" => {
                ops.push(OpCode::Skip);
                ops.push(OpCode::Skip);
                ops.push(OpCode::FetchAddX(spl[1].parse::<i32>().unwrap()));
            }
            _ => unimplemented!(),
        }
    }

    ops
}
