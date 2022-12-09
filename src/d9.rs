use std::{
    collections::HashSet,
    io::{BufRead, Lines},
};

use crate::common;

#[derive(Clone)]
struct Movement {
    dx: i32,
    dy: i32,
}

pub fn run(input: Lines<impl BufRead>) {
    let lines = common::parse(input);

    let mvs = process(&lines);

    let p1 = compute(&mvs, 2);

    println!("p1 = {}", p1);

    let p2 = compute(&mvs, 10);

    println!("p2 = {}", p2);
}

fn compute(mvs: &[Movement], segment_cnt: usize) -> usize {
    let mut segments = vec![(0_i32, 0_i32); segment_cnt];

    let mut tail_pos_history: HashSet<(i32, i32)> = HashSet::new();

    tail_pos_history.insert(segments[segment_cnt - 1].clone());

    // simulates movement
    for mv in mvs {
        move_segment(&mut segments, &mut tail_pos_history, mv);
    }

    tail_pos_history.len()
}

fn move_segment(
    segments: &mut [(i32, i32)],
    tail_history: &mut HashSet<(i32, i32)>,
    fmv: &Movement,
) {
    let mv = (*fmv).clone();

    let len = segments.len();

    let horizontal = fmv.dx != 0;
    let cnt = if horizontal { mv.dx.abs() } else { mv.dy.abs() };
    let inc = if horizontal { mv.dx / cnt } else { mv.dy / cnt };

    for _ in 0..cnt {
        let mut rhead = &mut segments[0];
        if horizontal {
            rhead.0 += inc;
        } else {
            rhead.1 += inc;
        }

        for i in 1..len {
            let head = segments[i - 1];
            let tail = segments[i];
            let mut ntail = tail.clone();

            let delta = (head.0 - tail.0, head.1 - tail.1);

            if delta.0.abs() >= 2 && delta.1.abs() >= 2 {
                // Diagonal movement
                ntail.0 = if tail.0 < head.0 {
                    head.0 - 1
                } else {
                    head.0 + 1
                };
                ntail.1 = if tail.1 < head.1 {
                    head.1 - 1
                } else {
                    head.1 + 1
                };
            } else if delta.0.abs() >= 2 {
                // Horizontal movement
                ntail.1 = head.1;
                ntail.0 = if tail.0 < head.0 {
                    head.0 - 1
                } else {
                    head.0 + 1
                };
            } else if delta.1.abs() >= 2 {
                // Vertical movement
                ntail.0 = head.0;
                ntail.1 = if tail.1 < head.1 {
                    head.1 - 1
                } else {
                    head.1 + 1
                };
            }
            segments[i] = ntail;
        }

        let real_tail = segments.last().unwrap();
        tail_history.insert(real_tail.clone());
    }
}

fn process(lines: &[String]) -> Vec<Movement> {
    let mut mvs = vec![];
    for line in lines {
        let spl: Vec<&str> = line.split(" ").collect();
        let dp = spl[1].parse::<i32>().unwrap();
        let mult = if spl[0] == "U" || spl[0] == "R" {
            1
        } else {
            -1
        };
        let vertical = spl[0] == "U" || spl[0] == "D";

        if vertical {
            mvs.push(Movement {
                dx: 0,
                dy: dp * mult,
            });
        } else {
            mvs.push(Movement {
                dx: dp * mult,
                dy: 0,
            });
        }
    }

    mvs
}
