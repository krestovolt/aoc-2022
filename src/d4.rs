use std::io::{BufRead, Lines};

use crate::common;

/// Stores the [start, end] of an assigned section
///
/// `start` & `end` are inclusive
struct SectionRange {
    start: i32,
    end: i32,
}

impl SectionRange {
    fn full_contains(&self, other: &Self) -> bool {
        let start = self.start <= other.start;
        let end = self.end >= other.end;

        start && end
    }

    fn overlap(&self, other: &Self) -> bool {
        let start = other.start >= self.start && other.start <= self.end; // between
        let end = other.end >= self.start && other.end <= self.end; // between

        start || end
    }
}

pub fn run(input: Lines<impl BufRead>) {
    let strs = common::parse(input);

    let secs = process(&strs);

    let fc = compute(&secs);
    println!("full contains = {}", fc);

    let fc_2 = compute_2(&secs);
    println!("overlap = {}", fc_2);
}

fn compute(secs: &[[SectionRange; 2]]) -> i32 {
    let mut fcnt = 0;

    for sec in secs {
        if sec[0].full_contains(&sec[1]) || sec[1].full_contains(&sec[0]) {
            fcnt += 1
        }
    }

    fcnt
}

fn compute_2(secs: &[[SectionRange; 2]]) -> i32 {
    let mut ocnt = 0;

    for sec in secs {
        if sec[0].overlap(&sec[1]) || sec[1].overlap(&sec[0]) {
            ocnt += 1
        }
    }

    ocnt
}

fn process(lines: &[String]) -> Vec<[SectionRange; 2]> {
    let mut v = vec![];

    for line in lines {
        let mut spl = line.split(",");

        // first part
        let fs = spl.next().unwrap();
        let mut s1 = fs.split("-");
        let st1 = s1.next().unwrap().parse::<i32>().unwrap();
        let ed1 = s1.next().unwrap().parse::<i32>().unwrap();
        let sec1 = SectionRange {
            start: st1,
            end: ed1,
        };

        // second part
        let ss = spl.next().unwrap();
        let mut s2 = ss.split("-");
        let st2 = s2.next().unwrap().parse::<i32>().unwrap();
        let ed2 = s2.next().unwrap().parse::<i32>().unwrap();
        let sec2 = SectionRange {
            start: st2,
            end: ed2,
        };

        v.push([sec1, sec2]);
    }

    v
}
