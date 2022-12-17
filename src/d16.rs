use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::{BufRead, Lines},
};

use crate::common;

const START_VALVE_NAME: &str = "AA";

#[derive(Debug)]
struct Vert {
    dsts: Vec<usize>,
    vflow: isize,
}

pub fn run(input: Lines<impl BufRead>) {
    let lines = common::parse(input);

    let (start_vi, g) = process(&lines);

    let p = simulate_p1(start_vi, &g);

    println!("{:?}", p);
}

// fixme: unstable, cannot be reused for part 2... sadge :(
fn simulate_p1(start_vi: usize, valves: &[Vert]) -> isize {
    let mut sum = 0;
    let deadline = 30;

    // (t_minute, heuristic (negative number), valve index)
    // this stores triplet of arrival time, heuristic, valve index, opened valve
    type State = (usize, isize, usize, HashSet<usize>);
    let mut open_set = Vec::<State>::new();

    let mut visited = HashMap::<(usize, usize), isize>::new();

    open_set.push((1, 0, start_vi, HashSet::new()));

    while let Some(cur) = open_set.pop() {
        let (t, score, vi, opened) = cur;

        if score <= visited.get(&(t, vi)).copied().unwrap_or(isize::MIN) {
            continue;
        }
        visited.insert((t, vi), score);

        if t >= deadline {
            sum = sum.max(score);
            continue;
        }

        let flow = valves[vi].vflow;
        if flow > 0 && !opened.contains(&vi) {
            let mut new_opened = opened.clone();
            new_opened.insert(vi);

            let coef = (&new_opened).iter().fold(0, |b, v| b + valves[*v].vflow);
            let new_score = score + coef;

            open_set.push((t + 1, new_score, vi, new_opened));
        }

        let coef = (&opened).iter().fold(0, |b, v| b + valves[*v].vflow);

        let new_score = score + coef;
        let dst = &valves[vi].dsts;
        for di in 0..dst.len() {
            open_set.push((t + 1, new_score, dst[di], opened.clone()));
        }
    }

    sum
}
fn process(lines: &[String]) -> (usize, Vec<Vert>) {
    let mut last_id = 0_usize;
    let mut ids: HashMap<String, usize> = HashMap::new();
    let mut g: Vec<Vert> = vec![];

    let mut get_id = |node: &str| {
        if ids.contains_key(node) {
            (*ids.get(node).unwrap(), false)
        } else {
            let id = last_id;

            ids.insert(node.into(), last_id);

            last_id += 1;
            (id, true)
        }
    };

    for line in lines {
        let splt = line.split(" ").collect::<Vec<&str>>();

        // (1, "AA")
        let src = splt[1];
        // (4, "rate=0;")
        let rate = splt[4]
            .to_lowercase()
            .trim_end_matches(";")
            .trim_start_matches("rate=")
            .parse::<isize>()
            .unwrap();

        // (6, "lead")
        // (7, "to")
        // (8, "valves")
        // (9, "DD,")
        // (10, "II,")
        // (11, "BB")
        let dsts = splt[9..]
            .iter()
            .map(|s| s.trim_end_matches(",").to_owned())
            .collect::<Vec<String>>();

        let (sid, new_entry) = get_id(src);
        if new_entry {
            g.push(Vert {
                dsts: vec![],
                vflow: 0,
            });
        }
        g[sid].vflow = rate;
        for dst in dsts {
            let (did, new_entry) = get_id(&dst);
            if new_entry {
                g.push(Vert {
                    dsts: vec![],
                    vflow: 0,
                });
            }

            g[sid].dsts.push(did);
        }
    }

    (ids.get(START_VALVE_NAME).copied().unwrap(), g)
}
