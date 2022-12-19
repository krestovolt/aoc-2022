use std::io::{BufRead, Lines};

use crate::common;

const R_ORE: usize = 0;
const R_CLAY: usize = 1;
const R_OBSI: usize = 2;
const R_GEOD: usize = 3;

type Resource = [isize; 4];

#[derive(Clone, Debug)]
struct State {
    t: usize,
    miners: [isize; 4],
    resource: Resource,
}

struct RobotBlueprint {
    miners: [Resource; 4],
}

pub fn run(input: Lines<impl BufRead>) {
    let lines = common::parse(input);

    let bps = process(&lines);

    let mut sum = 0;
    for (bpi, bp) in bps.iter().enumerate() {
        let max_res = simulate(bp, 24);
        println!("bp#{} max geodes: {}", bpi + 1, max_res);

        sum += (bpi + 1) as isize * max_res;
    }

    println!("sum = {sum}");

    let top3_sum = bps
        .iter()
        .take(3)
        .enumerate()
        .map(|(bpi, bp)| {
            let max_res = simulate(bp, 32);
            println!("bp#{} @32min max geodes: {}", bpi + 1, max_res);
            max_res
        })
        .product::<isize>();

    println!("top3 @32min = {top3_sum}");
}

fn simulate(bp: &RobotBlueprint, time_limit: usize) -> isize {
    let needed_resource =
        bp.miners
            .iter()
            .fold([isize::MIN, isize::MIN, isize::MIN, isize::MIN], |b, v| {
                [
                    b[R_ORE].max(v[R_ORE]),
                    b[R_CLAY].max(v[R_CLAY]),
                    b[R_OBSI].max(v[R_OBSI]),
                    isize::MAX,
                ]
            });

    let initial_state = State {
        t: 0,
        miners: [1, 0, 0, 0],
        resource: [0, 0, 0, 0],
    };

    let mut max_geod = 0;

    let mut open_state = Vec::<State>::new();
    open_state.push(initial_state);

    while let Some(state) = open_state.pop() {
        let mut no_miner_created = true;
        for ri in R_ORE..=R_GEOD {
            if needed_resource[ri] == state.miners[ri] {
                // Skip creating new miner if the optimal number of miners has been satisfied
                continue;
            }

            let miner_required_res = &bp.miners[ri];

            // calculate how long we should wait before we have enough resource to create the new miner for this type
            let wait_time = (R_ORE..=R_GEOD)
                .map(|resource_type| {
                    let resource_delta =
                        state.resource[resource_type] - miner_required_res[resource_type];

                    if resource_delta >= 0 {
                        0 // ready now
                    } else if state.miners[resource_type] < 1 {
                        time_limit + time_limit // will never be possible since we don't have any miner for this resource type
                    } else {
                        ((resource_delta * -1 + state.miners[resource_type] - 1)
                            / state.miners[resource_type]) as usize
                    }
                })
                .max()
                .unwrap();

            let wait_done = wait_time + 1;
            let t_created = state.t + wait_done;
            if t_created >= time_limit {
                // not possible to create this type of miner on-time
                continue;
            }

            // calculate the added value if we actualy build this new miner
            let mut next_resource: Resource = [0, 0, 0, 0];
            let mut next_miners: [isize; 4] = state.miners.clone();
            for nri in R_ORE..=R_GEOD {
                next_resource[nri] = state.resource[nri] + (state.miners[nri] * wait_done as isize)
                    - miner_required_res[nri];

                next_miners[nri] += isize::from(ri == nri);
            }

            let t_remaining = time_limit - t_created;
            let t_coef = ((t_remaining - 1) * t_remaining / 2) as isize;
            let future_geod =
                t_coef + next_resource[R_GEOD] + (next_miners[R_GEOD] * t_remaining as isize);

            if future_geod < max_geod {
                continue;
            }

            open_state.push(State {
                resource: next_resource,
                miners: next_miners,
                t: t_created,
            });

            no_miner_created = false;
        }

        if no_miner_created {
            let t_remaining = (time_limit - state.t) as isize;
            let cur_geod = state.resource[R_GEOD] + state.miners[R_GEOD] * t_remaining;
            // simulate until time_limit
            max_geod = max_geod.max(cur_geod);
        }
    }

    max_geod
}

fn process(lines: &[String]) -> Vec<RobotBlueprint> {
    let mut v = vec![];

    for line in lines {
        let mut s = line
            .split(":")
            .last()
            .unwrap()
            .split(".")
            .map(|s| s.trim().split(" ").collect::<Vec<&str>>());

        let miner_ore = s.next().unwrap();
        let miner_ore = [miner_ore[4].parse::<isize>().unwrap(), 0, 0, 0];

        let miner_clay = s.next().unwrap();
        let miner_clay = [miner_clay[4].parse::<isize>().unwrap(), 0, 0, 0];

        let miner_obsidian = s.next().unwrap();
        let miner_obsidian = [
            miner_obsidian[4].parse::<isize>().unwrap(),
            miner_obsidian[7].parse::<isize>().unwrap(),
            0,
            0,
        ];

        let miner_geode = s.next().unwrap();
        let miner_geode = [
            miner_geode[4].parse::<isize>().unwrap(),
            0,
            miner_geode[7].parse::<isize>().unwrap(),
            0,
        ];

        v.push(RobotBlueprint {
            miners: [miner_ore, miner_clay, miner_obsidian, miner_geode],
        })
    }

    v
}
