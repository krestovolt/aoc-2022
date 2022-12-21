use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, Lines},
};

use crate::common;

const START_VALVE_NAME: &str = "AA";

#[derive(Debug)]
struct Vert {
    mask: u64,
    vflow: usize,
}

pub fn run(input: Lines<impl BufRead>) {
    let lines = common::parse(input);

    let (start_vi, G, D) = process(&lines);

    let mut ans = HashMap::<u64, usize>::new();
    // p1 - no elephant
    simulate_mut(&G, &D, start_vi, 0, 30, 0, &mut ans);
    let max_flow = ans.values().max().unwrap();
    println!("p1 = {}", max_flow);

    // p2 - one elephant
    /*
    for part 2, we can simulate each actors (human and elephant) 1-by-1
    we're not necessarily simulate them at the same time because the result will be the same. The important
    things to remember when evaluating the final result is to accumulate the sum of maximum flow
    from N unique set of opened valves where N is the number of actors (in this case only 2)
     */
    ans.clear();
    simulate_mut(&G, &D, start_vi, 0, 26, 0, &mut ans);

    // make a pool of total flow from 2 different combinations of valve opening
    let mut max_flow = 0;
    for a in ans.iter() {
        for b in ans.iter() {
            if (*a.0 & *b.0) == 0 {
                max_flow = max_flow.max(*a.1 + *b.1);
            }
        }
    }

    println!("p2 = {}", max_flow);
}

// Simulate maximum total flow using distance map (D) computed with floyd-warshall shortest path
fn simulate_mut(
    G: &[Vert],
    D: &[Vec<usize>],
    start_valve_id: usize,
    opened_mask: u64,
    time_limit: usize,
    flow: usize,
    res_state: &mut HashMap<u64, usize>,
) {
    let cur_val = res_state.get(&opened_mask).copied().unwrap_or(0);
    res_state.insert(opened_mask, flow.max(cur_val));

    for next_valve_id in 0..G.len() {
        let vmask = G[next_valve_id].mask;
        let vflow = G[next_valve_id].vflow;
        let cost = D[start_valve_id][next_valve_id];
        if cost == usize::MAX || vflow < 1 || (opened_mask & vmask != 0) {
            continue;
        }

        let time_rem = (time_limit as isize) - (D[start_valve_id][next_valve_id] as isize) - 1; // minus one since opening the valve cost 1 minute
        if time_rem <= 0 {
            continue;
        }

        let time_rem = time_rem as usize;
        let accumulated_vflow = time_rem * vflow;

        simulate_mut(
            G,
            D,
            next_valve_id,
            opened_mask | vmask,
            time_rem,
            flow + accumulated_vflow,
            res_state,
        );
    }
}

fn process(lines: &[String]) -> (usize, Vec<Vert>, Vec<Vec<usize>>) {
    let len = lines.len();

    let mut G: Vec<Vert> = vec![];
    G.reserve(len);

    let mut adjs: Vec<HashSet<String>> = vec![];
    adjs.reserve(len);

    let mut ids: HashMap<String, usize> = HashMap::new();
    ids.reserve(len);

    for (idx, line) in lines.iter().enumerate() {
        let splt = line.split(" ").collect::<Vec<&str>>();

        // (1, "AA")
        let src = splt[1];
        ids.insert(src.into(), idx);

        // (4, "rate=0;")
        let rate = splt[4]
            .to_lowercase()
            .trim_end_matches(";")
            .trim_start_matches("rate=")
            .parse::<usize>()
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
            .collect::<HashSet<String>>();

        adjs.push(dsts);
        G.push(Vert {
            mask: 1 << idx,
            vflow: rate,
        });
    }

    let mut D: Vec<Vec<usize>> = vec![];
    D.resize(len, vec![usize::MAX; len]);

    for u in 0..len {
        let dsts = adjs[u].iter().map(|s| ids[s]).collect::<Vec<_>>();
        for v in dsts {
            D[u][v] = 1;
        }
    }

    // Build a shortest path distance using floyd-warshall algorithm
    // https://en.wikipedia.org/wiki/Floyd%E2%80%93Warshall_algorithm
    for t in 0..len {
        for u in 0..len {
            for v in 0..len {
                if D[u][t] == usize::MAX || D[t][v] == usize::MAX {
                    // prevent overflow
                    continue;
                }
                D[u][v] = D[u][v].min(D[u][t] + D[t][v])
            }
        }
    }

    (ids[START_VALVE_NAME], G, D)
}
