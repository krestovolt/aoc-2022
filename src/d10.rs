use std::io::{BufRead, Lines};

use crate::common;

const SCREEN_W: usize = 40;
const SCREEN_H: usize = 6;
const SCREEN_DIM: usize = SCREEN_W * SCREEN_H;
const CYCLE_INCR: usize = 40;

enum OpCode {
    Noop,
    Skip,
    FetchAddX(i32),
}

pub fn run(input: Lines<impl BufRead>) {
    let lines = common::parse(input);
    let ops = process(&lines);
    let mut pixels: [char; SCREEN_DIM] = ['.'; SCREEN_DIM];
    let sum = compute(&ops, &mut pixels);
    println!("sum: {}", sum);
    render(pixels);
}

fn render(pixels: [char; SCREEN_DIM]) {
    for r in 0..SCREEN_H {
        for c in 0..SCREEN_W {
            let p = if pixels[c + (r * SCREEN_W)] == '#' {
                "#"
            } else {
                " "
            };
            print!("{}", p);
        }
        println!("");
    }
}

fn compute(ops: &[OpCode], pixels_out: &mut [char; SCREEN_DIM]) -> i32 {
    let mut sum = 0;

    let mut pc = 0;
    let mut reg_x = 1_i32;
    let mut checkpoint = 20;

    let mut sprites: Vec<char> = "###....................................."
        .chars()
        .into_iter()
        .collect();

    for op in ops {
        print!("pc={:03} | ", pc);
        match op {
            // During fetch counter is not incremented
            OpCode::FetchAddX(num) => {
                let sm = &mut sprites;
                if reg_x - 1 >= 0 {
                    sm[(reg_x - 1) as usize] = '.';
                }
                if reg_x >= 0 {
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
                pixels_out[pc as usize] = s[(pc % SCREEN_W) as usize];
                pc += 1;

                println!("noop");
            }
        }

        if checkpoint == pc {
            let sig_strength = (pc as i32) * reg_x;
            checkpoint += CYCLE_INCR;
            sum += sig_strength
        }
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
