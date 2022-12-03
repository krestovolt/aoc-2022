use std::{
    collections::HashSet,
    io::{BufRead, Lines},
};

use crate::common;

const CAP_VAL: u8 = b'A';
const DEC_LOWER: u8 = 31;
const INC_UPPER: u8 = 27;

struct Ruck {
    comp_1: HashSet<u8>,
    comp_2: HashSet<u8>,
    comp_3: HashSet<u8>,
}

pub fn run(input: Lines<impl BufRead>) {
    let strs = common::parse(input);

    let rucks = process(&strs);
    let rucks_2 = process_2(&strs);

    let sum_p = compute(&rucks);
    let sum_p3 = compute(&rucks_2);

    println!("sum of priorities: {}", sum_p);
    println!("sum of priorities (3-part): {}", sum_p3)
}

fn compute(rucks: &[Ruck]) -> i32 {
    let mut sum_p = 0;

    for r in rucks {
        let isec_interm = r.comp_1.intersection(&r.comp_2);
        let isec = isec_interm.filter(|v| {
            if r.comp_3.len() > 0 {
                r.comp_3.contains(v)
            } else {
                true
            }
        });

        for v in isec {
            let mut p = *v - CAP_VAL;
            if p > DEC_LOWER {
                p = p - DEC_LOWER;
            } else {
                p = p + INC_UPPER;
            }

            sum_p += p as i32;
        }
    }

    sum_p
}

fn process_2(lines: &[String]) -> Vec<Ruck> {
    let sz = lines.len() / 3;
    let mut v = vec![];
    v.reserve(sz);

    for i in 0..sz {
        let x = i * 3;
        let y = x + 1;
        let z = y + 1;

        let r = Ruck {
            comp_1: lines[x].as_bytes().to_vec().iter().cloned().collect(),
            comp_2: lines[y].as_bytes().to_vec().iter().cloned().collect(),
            comp_3: lines[z].as_bytes().to_vec().iter().cloned().collect(),
        };

        v.push(r)
    }

    v
}

fn process(lines: &[String]) -> Vec<Ruck> {
    let mut v = vec![];

    for line in lines {
        let l = line.as_str();

        let h_len = l.len() / 2;
        let fh = &l[0..h_len];
        let sh = &l[h_len..];

        let r = Ruck {
            comp_1: fh.as_bytes().to_vec().iter().cloned().collect(),
            comp_2: sh.as_bytes().to_vec().iter().cloned().collect(),
            comp_3: HashSet::new(),
        };

        v.push(r);
    }

    v
}
