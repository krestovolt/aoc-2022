use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, Lines},
};

use crate::common;

pub fn run(input: Lines<impl BufRead>) {
    let lines = common::parse(input);

    let mut elfs = process(&lines);
    for iter in 0..10 {
        let nopt = expands(iter, &elfs);

        if let Some(new_elfs) = nopt {
            elfs = new_elfs;
        } else {
            break;
        }
    }

    let minc = elfs.iter().map(|e| e.0).min().unwrap();
    let maxc = elfs.iter().map(|e| e.0).max().unwrap();
    let minr = elfs.iter().map(|e| e.1).min().unwrap();
    let maxr = elfs.iter().map(|e| e.1).max().unwrap();

    let area = (maxc - minc + 1) * (maxr - minr + 1) - elfs.len() as isize;

    let mut iter = 10;
    loop {
        let nopt = expands(iter, &elfs);
        iter += 1;

        if let Some(new_elfs) = nopt {
            elfs = new_elfs;
        } else {
            break;
        }
    }

    println!("area = {area}");
    println!("max iter = {iter}");
}

fn propose(
    round: usize,
    my_pos: (isize, isize),
    elfs: &HashSet<(isize, isize)>,
) -> Option<(isize, isize)> {
    let mut found_adj = false;
    for dir_y in -1..=1 {
        for dir_x in -1..=1 {
            if dir_x == 0 && dir_y == 0 {
                continue;
            }
            if elfs.get(&(my_pos.0 + dir_x, my_pos.1 + dir_y)).is_some() {
                found_adj = true;
                break;
            }
        }
        if found_adj {
            break;
        }
    }
    if !found_adj {
        // No elfs in any 8 dirs, cannot move
        return None;
    }
    // Move previous first-choice to last in each round
    let move_choice_range = round..round + 4;
    for dir_choice in move_choice_range {
        let dir = dir_choice % 4;
        if dir < 2 {
            // N / S
            let dir_y = if dir == 0 { -1 } else { 1 };
            let mut found_adj = false;
            for dir_x in -1..=1 {
                if elfs.get(&(my_pos.0 + dir_x, my_pos.1 + dir_y)).is_some() {
                    found_adj = true;
                    break;
                }
            }
            if !found_adj {
                return Some((my_pos.0, my_pos.1 + dir_y));
            }
        } else {
            // W / E
            let dir_x = if dir == 2 { -1 } else { 1 };
            let mut found_adj = false;
            for dir_y in -1..=1 {
                if elfs.get(&(my_pos.0 + dir_x, my_pos.1 + dir_y)).is_some() {
                    found_adj = true;
                    break;
                }
            }
            if !found_adj {
                return Some((my_pos.0 + dir_x, my_pos.1));
            }
        }
    }
    None
}

fn expands(round: usize, elfs: &HashSet<(isize, isize)>) -> Option<HashSet<(isize, isize)>> {
    let mut staging = HashMap::<(isize, isize), usize>::new();
    elfs.iter().for_each(|e| {
        let p = propose(round, e.clone(), elfs);
        if let Some(np) = p {
            let ent = staging.entry(np).or_insert(0);
            *ent += 1;
        }
    });

    if staging.len() == 0 {
        return None;
    }

    // let mut nelves = HashSet::new();
    // for e in elfs.iter() {
    //     let mut e = *e;
    //     if let Some(p) = propose(round, e, elfs) {
    //         if *proposals.get(&p).unwrap() == 1 {
    //             e = p;
    //         }
    //     }
    //     assert!(nelves.insert(e));
    // }
    // Some(nelves)

    let ns = elfs
        .iter()
        .flat_map(|e| {
            let p = propose(round, e.clone(), elfs);
            if p.is_none() {
                return Some(*e);
            }
            let p = p.expect("not none");
            if staging[&p] == 1 {
                Some(p)
            } else {
                Some(*e)
            }
        })
        .collect::<HashSet<(isize, isize)>>();

    Some(ns)
}

fn process(lines: &[String]) -> HashSet<(isize, isize)> {
    let mut elfs = HashSet::<(isize, isize)>::new();
    for (r, line) in lines.iter().enumerate() {
        for (c, chr) in line.chars().enumerate() {
            if chr == '#' {
                elfs.insert((c as isize, r as isize));
            }
        }
    }
    elfs
}
