use std::{
    collections::{HashSet, VecDeque},
    io::{BufRead, Lines},
};

use crate::common;

const AIR: u8 = 0;
const WALL: u8 = 1;
const SAND: u8 = 2;
const FLOOR_Y_REL: usize = 2;

const SAND_SRC: [i32; 2] = [500, 0];
const DIR_X: [i32; 3] = [0, -1, 1];

#[derive(Clone, Debug, PartialEq, Eq)]
struct Point {
    pub x: i32,
    pub y: i32,
}

pub fn run(mut input: Lines<impl BufRead>) {
    let lines = common::parse(input);

    let (mut grid, max_y) = process(&lines);

    let sim_res_1 = simulate(&mut grid, max_y, false);
    println!("(p1) all-wall filled: {}", sim_res_1);
    let sim_res_2 = simulate(&mut grid, max_y, true);
    println!("(p2) flooded: {}", sim_res_1 + sim_res_2);

    if grid.len() > 0 {}
}

fn simulate(grid: &mut VecDeque<Vec<u8>>, max_y: usize, flood: bool) -> usize {
    if flood {
        let mut last_drop: VecDeque<Point> = VecDeque::new();
        let mut cnt = 0;

        last_drop.push_front(Point {
            x: SAND_SRC[0],
            y: SAND_SRC[1],
        });

        while !last_drop.is_empty() {
            let cur_pos = last_drop.pop_back().unwrap();
            let cx = cur_pos.x as usize;
            let cy = cur_pos.y as usize;

            if grid[cx][cy] == AIR {
                grid[cx][cy] = SAND;
                cnt += 1;
                for dx in DIR_X {
                    let nx = cur_pos.x + dx;
                    let ny = cur_pos.y + 1;

                    if nx < 0 || ny as usize >= max_y {
                        if nx < 0 {}
                        continue;
                    }
                    last_drop.push_front(Point { x: nx, y: ny });
                }
            }
        }

        cnt
    } else {
        let mut last_drop: Vec<Point> = vec![];
        last_drop.push(Point {
            x: SAND_SRC[0],
            y: SAND_SRC[1],
        });

        let mut cnt = 0;

        while !last_drop.is_empty() {
            let cur_pos = last_drop.last().unwrap().clone();
            let cx = cur_pos.x as usize;
            let cy = cur_pos.y as usize;

            if (cur_pos.y + 1) as usize >= max_y {
                break;
            }

            let mut settled = true;
            for dx in DIR_X {
                let nx = cur_pos.x + dx;
                let ny = cur_pos.y + 1;
                if nx < 0 {
                    continue;
                }

                let nx = nx as usize;
                let mut ny = ny as usize;
                if grid[nx][ny] == AIR {
                    settled = false;

                    // Jumps to the nearest vertically adjacent non-AIR cell
                    while ny < grid[nx].len() - 1 {
                        let nny = ny + 1;
                        if grid[nx][nny] != AIR {
                            break;
                        }
                        ny = nny;
                    }

                    last_drop.push(Point {
                        x: nx as i32,
                        y: ny as i32,
                    });

                    break;
                }
            }

            if settled {
                cnt += 1;
                last_drop.pop();
                grid[cx][cy] = SAND
            }
        }

        cnt
    }
}

fn process(lines: &[String]) -> (VecDeque<Vec<u8>>, usize) {
    let mut max_x = 0;
    let mut max_y = 0;

    let mut walls: HashSet<(usize, usize)> = HashSet::new();

    for line in lines {
        let stone_paths: Vec<Point> = line
            .split("->")
            .map(|el| {
                let nums: Vec<i32> = el
                    .trim()
                    .split(",")
                    .map(|num| num.parse::<i32>().unwrap())
                    .collect();

                max_x = max_x.max(nums[0]);
                max_y = max_y.max(nums[1]);

                Point {
                    x: nums[0],
                    y: nums[1],
                }
            })
            .collect();

        for pi in 1..stone_paths.len() {
            let prev = &stone_paths[pi - 1];
            let cur = &stone_paths[pi];

            for y in (prev.y.min(cur.y))..=(prev.y.max(cur.y)) {
                for x in (prev.x.min(cur.x))..=(prev.x.max(cur.x)) {
                    walls.insert((x as usize, y as usize));
                }
            }
        }
    }

    let max_y = max_y as usize + FLOOR_Y_REL;
    let max_x = max_x as usize + 1;

    let mut grid = VecDeque::new();
    grid.resize(max_x * 2, vec![AIR; max_y + 1]);
    grid.reserve((max_x * 2) as usize);
    for wp in walls {
        grid[wp.0][wp.1] = WALL;
    }

    for x in 0..max_x {
        grid[x][max_y] = WALL;
    }

    if grid.len() > 0{}

    (grid, max_y)
}
