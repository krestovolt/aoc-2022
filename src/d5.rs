use std::{io::{BufRead, Lines}, collections::VecDeque};

use crate::common;

type CrateStack = VecDeque<char>;

struct Move {
    cnt: usize,
    from: usize,
    to: usize,
}

pub fn run(input: Lines<impl BufRead>) {
    let strs = common::parse(input);

    let (data, moves) = process(&strs);
    let res = compute(&mut data.clone(), &moves, false);
    let res_2 = compute(&mut data.clone(), &moves, true);

    println!("top items={}", res);
    println!("top items (mover 9001)={}", res_2);
}

fn compute(ds: &mut [CrateStack], mvs: &[Move], use_mover_9001: bool) -> String {
    let mut rs = String::from("");

    // sim
    for m in mvs {
        let cf = &mut ds[m.from];
        let mut sl_cf = vec![' '; m.cnt];
        for i in 0..m.cnt {
            if use_mover_9001 {
                sl_cf[(m.cnt - 1) - i] = cf.pop_back().unwrap();
            } else {
                sl_cf[i] = cf.pop_back().unwrap();
            }
        }

        let ct = &mut ds[m.to];
        ct.extend(sl_cf);
    }

    for cs in ds {
        let last_ch = cs.back();
        if let Some(ch) = last_ch {
            rs.push(*ch);
        }
    }

    rs
}

fn process(lines: &[String]) -> (Vec<CrateStack>, Vec<Move>) {
    let mut v: Vec<CrateStack> = vec![];
    let mut moves: Vec<Move> = vec![];

    let mut lit = lines.iter().peekable();

    for line in lit.by_ref() {
        let mut init = true;
        let mut cs: Vec<char> = vec![];

        // Parse each stack line
        let l: Vec<char> = line.chars().collect();
        let mut li = 1_usize;
        let l_len = l.len();
        while li < l_len {
            let ch = l[li];
            let bv = ch as u8;
            if (bv >= 65 && bv <= 90) || bv == 32 {
                cs.push(ch);

                li += 4;
            } else {
                init = false;
                break;
            }
        }

        if !init {
            break;
        }

        if v.len() < cs.len() {
            v.resize(cs.len(), VecDeque::new());
        }

        for ci in 0..cs.len() {
            let c = cs[ci];
            if c != ' ' {
                v[ci].push_front(c);
            }
        }
    }

    for line in lit.by_ref() {
        let mut spl = line.split(" ");
        spl.next();
        let cnt = spl.next().unwrap().parse::<usize>().unwrap();
        spl.next();
        let from = spl.next().unwrap().parse::<usize>().unwrap() - 1;
        spl.next();
        let to = spl.next().unwrap().parse::<usize>().unwrap() - 1;

        let m = Move { cnt, from, to };
        moves.push(m);
    }

    (v, moves)
}
