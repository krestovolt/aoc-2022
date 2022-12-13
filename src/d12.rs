use std::{
    collections::{BTreeSet, HashMap, VecDeque},
    io::{BufRead, Lines},
};

use crate::common;

const DIRS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

struct GridState {
    grid: Vec<Vec<u8>>,
    orig_start_pos: usize,
    start_pos: Vec<(usize, usize)>,
    end_pos: (usize, usize),
}

impl GridState {
    fn get_h_score(&self, pos: &(usize, usize)) -> usize {
        (self.end_pos.0.abs_diff(pos.0)) + (self.end_pos.1.abs_diff(pos.1))
    }

    fn get_origin(&self) -> (usize, usize) {
        self.start_pos[self.orig_start_pos]
    }

    fn get_reachable(&self, pos: &(usize, usize)) -> Vec<(usize, usize)> {
        let mut acc = vec![];

        let h = self.grid.len() as i32;
        let w = self.grid[0].len() as i32;

        let (px, py) = (pos.0, pos.1);
        let grid = &self.grid;

        for d in 0..4 {
            let (nx, ny) = (pos.0 as i32 + DIRS[d].0, pos.1 as i32 + DIRS[d].1);

            // bound checks
            if !(nx >= 0 && nx < w && ny >= 0 && ny < h) {
                continue;
            }

            let nx = nx as usize;
            let ny = ny as usize;
            // terrain height checks
            if grid[ny][nx] > grid[py][px] + 1 {
                continue;
            }

            acc.push((nx, ny));
        }

        acc
    }
}

pub fn run(input: Lines<impl BufRead>) {
    let lines = common::parse(input);

    let grid = process(&lines);

    let (min_step, _path) = compute(&grid, true);
    println!("p1 - min step = {}", min_step);

    let (min_step, _path) = compute(&grid, false);
    println!("p2 - min step = {}", min_step);
}

fn compute(grid: &GridState, start_from_origin: bool) -> (usize, Vec<(usize, usize)>) {
    let mut path = vec![];

    let mut step_cnt: HashMap<(usize, usize), usize> = HashMap::new();

    // Sorted/Priority "queue"
    let mut q: BTreeSet<(usize, (usize, usize))> = BTreeSet::new();

    let mut h_scores: HashMap<(usize, usize), usize> = HashMap::new();

    if start_from_origin {
        let s_h_score = grid.get_h_score(&grid.get_origin());

        h_scores.insert(grid.get_origin(), s_h_score);
        q.insert((s_h_score, grid.get_origin()));
        step_cnt.insert(grid.get_origin(), 0);
    } else {
        for s in 0..grid.start_pos.len() {
            let s_h_score = grid.get_h_score(&grid.start_pos[s]);

            h_scores.insert(grid.start_pos[s], s_h_score);
            q.insert((s_h_score, grid.start_pos[s]));
            step_cnt.insert(grid.start_pos[s], 0);
        }
    }

    let mut it = 0;
    while !q.is_empty() {
        let cur = q.iter().next().unwrap().to_owned();
        q.remove(&cur);

        let (_h_score, cpos) = cur;
        if grid.end_pos == cpos {
            println!("#{it} found it {:?}", step_cnt[&cpos]);
            // break;
        }

        let cs_cnt = step_cnt.get(&cpos).unwrap().to_owned();
        let reachables: Vec<(usize, usize)> = grid.get_reachable(&cpos);

        for npos in reachables {
            let ncs_cnt = cs_cnt + 1;
            let prev_ncs_cnt = step_cnt.get(&npos).copied();
            if ncs_cnt < prev_ncs_cnt.unwrap_or(usize::MAX) {
                if prev_ncs_cnt.is_some() {
                    q.remove(&(h_scores[&npos], npos));
                }

                let n_h_score = ncs_cnt + grid.get_h_score(&npos);
                q.insert((n_h_score, npos));
                step_cnt.insert(npos, ncs_cnt);
                h_scores.insert(npos, n_h_score);
            }
        }

        it += 1;
    }

    {
        // let mut x = step_cnt.keys();
        // let mut cnt = 0;
        // while cnt < 20 {
        //     let k = x.next().unwrap();
        //     println!("step_cnt {:?}", step_cnt.get_key_value(k).unwrap());
        //     println!("heuristic {:?}", h_scores.get_key_value(k).unwrap());
        //     println!("{:->10}", "");
        //     cnt += 1
        // }

        // {
        //     println!(
        //         "step_cnt {:?}",
        //         step_cnt.get_key_value(&grid.end_pos).unwrap()
        //     );
        //     println!(
        //         "heuristic {:?}",
        //         h_scores.get_key_value(&grid.end_pos).unwrap()
        //     );
        //     println!("{:->10}", "");
        // }
    }

    (step_cnt.get(&grid.end_pos).unwrap().to_owned(), path)
}

fn process(lines: &[String]) -> GridState {
    let mut grid = vec![];

    let mut orig_start_pos = 0;
    let mut start_pos = vec![];
    let mut end_pos = (0_usize, 0_usize);

    let mut y = 0;
    for line in lines {
        let chrs: Vec<u8> = line
            .as_bytes()
            .iter()
            .enumerate()
            .map(|(x, c)| {
                let c = *c;
                if c == b'S' {
                    orig_start_pos = start_pos.len();
                    start_pos.push((x, y));
                    0
                } else if c == b'E' {
                    end_pos.0 = x;
                    end_pos.1 = y;
                    b'z' - b'a'
                } else {
                    if c == b'a' {
                        start_pos.push((x, y));
                    }
                    c - b'a'
                }
            })
            .collect();

        y += 1;
        grid.push(chrs);
    }

    GridState {
        grid,
        orig_start_pos,
        start_pos,
        end_pos,
    }
}
