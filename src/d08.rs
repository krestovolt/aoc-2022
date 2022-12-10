use std::io::{BufRead, Lines};

use crate::common;

pub fn run(input: Lines<impl BufRead>) {
    let strs = common::parse(input);
    let (vcnt, max_sc) = process(&strs);

    println!("count = {}", vcnt);
    println!("max scenic = {}", max_sc);
}

fn process(lines: &[String]) -> (usize, usize) {
    let mut grid: Vec<Vec<u8>> = vec![];

    let mut r = 0_usize;
    for line in lines {
        grid.push(vec![]);

        for cn in line.chars() {
            let n = cn as u8 - b'0';

            grid[r].push(n);
        }

        r += 1;
    }

    let len = grid.len();
    let cols = grid[0].len();
    let all_rows = 0..len;
    let all_cols = 0..cols;

    let mut vis: Vec<Vec<bool>> = vec![vec![false; cols]; len];

    for row in all_rows.clone() {
        let mut prev = 99_u8;
        all_cols
            .clone()
            .for_each(|col| is_visible(&mut vis, &grid, &mut prev, row, col));

        let mut prev = 99_u8;
        all_cols
            .clone()
            .rev()
            .for_each(|col| is_visible(&mut vis, &grid, &mut prev, row, col));
    }

    for col in all_cols.clone() {
        let mut prev = 99_u8;
        all_rows
            .clone()
            .for_each(|row| is_visible(&mut vis, &grid, &mut prev, row, col));

        let mut prev = 99_u8;
        all_rows
            .clone()
            .rev()
            .for_each(|row| is_visible(&mut vis, &grid, &mut prev, row, col));
    }

    let mut max_sc = 0;
    for r in all_rows.clone() {
        for c in all_cols.clone() {
            let sc = get_view_dist(&grid, grid[r][c], r, c);
            max_sc = max_sc.max(sc);
        }
    }


    (vis.into_iter().flatten().filter(|b| *b).count(), max_sc)
}

fn is_visible(vis: &mut [Vec<bool>], grid: &[Vec<u8>], prev: &mut u8, r: usize, c: usize) {
    if *prev == 99 || *prev < grid[r][c] {
        vis[r][c] = true;
        *prev = grid[r][c];
    }
}

fn get_view_dist(grid: &[Vec<u8>], cur: u8, r: usize, c: usize) -> usize {
    let rows = grid.len() as i32;
    let cols = grid[0].len()  as i32;
    
    
    
    let mut ri: i32 = (r as i32) + 1;
    let mut c1 = 0_usize;
    while ri < rows {
        c1 += 1;
        if grid[ri as usize][c] >= cur {
            break;
        }
        ri += 1;
    }

    ri = (r as i32) - 1;
    let mut c2 = 0_usize;
    while ri >= 0 {
        c2 += 1;
        if grid[ri as usize][c] >= cur {
            break;
        }
        ri -= 1;
    }

    let mut ci = (c  as i32) - 1;
    let mut c3 = 0_usize;
    while ci >= 0 {
        c3 += 1;
        if grid[r][ci as usize] >= cur {
            break;
        }
        ci -= 1;
    }

    ci = (c as i32) + 1;
    let mut c4 = 0_usize;
    while ci < cols {
        c4 += 1;
        if grid[r][ci as usize] >= cur {
            break;
        }
        ci += 1;
    }

    c1 * c2 * c3 * c4
}
