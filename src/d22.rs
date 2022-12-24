use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap},
    hash::Hash,
    io::{BufRead, Lines},
};

use crate::common;

const DIRS: [[isize; 2]; 4] = [[1, 0], [0, 1], [-1, 0], [0, -1]];

#[derive(Debug, Clone)]
enum Tile {
    Dirt = 1,
    Wall = 2,
}

#[derive(Debug, Clone)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Direction {
    fn turn_right(&self) -> Self {
        let ndir = (self.value() + 1) % 4;
        Self::from(ndir)
    }

    fn turn_left(&self) -> Self {
        let ndir = (self.value() as isize - 1).rem_euclid(4) as usize;
        Self::from(ndir)
    }

    fn inv(&self) -> Self {
        match self {
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Up => Direction::Down,
        }
    }

    fn from(i: usize) -> Self {
        match i {
            0 => Direction::Right,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Up,
            _ => unreachable!(),
        }
    }

    fn value(&self) -> usize {
        match self {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }
}

#[derive(Debug, Clone)]
enum Move {
    L,
    R,
    F(usize),
}

#[derive(Clone, Debug, Eq, Ord)]
struct Point {
    x: usize,
    y: usize,
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let y_cmp = self.y.cmp(&other.y);
        match y_cmp {
            Ordering::Equal => Some(self.x.cmp(&other.x)),
            Ordering::Greater | Ordering::Less => Some(y_cmp),
        }
    }
}

impl Point {
    fn warped_shift_by_one(&self, grid: &HashMap<Point, Tile>, d: &Direction) -> (Self, Tile) {
        let mut dir = DIRS[d.value()];
        let mut np = Self {
            x: (self.x as isize + dir[0]) as usize,
            y: (self.y as isize + dir[1]) as usize,
        };

        let mut tile = grid.get(&np);
        if let Some(_) = tile {
            return (np, tile.unwrap().clone());
        }

        assert!(tile.is_none());

        // Find the spot on the edge with the inversed direction
        let d_inv = d.inv();
        dir = DIRS[d_inv.value()];

        let mut next_p = self.clone();

        let mut next_tile = grid.get(&next_p);
        tile = next_tile;
        np = next_p.clone();
        while next_tile.is_some() {
            np = next_p.clone();
            tile = next_tile;

            next_p.x = (next_p.x as isize + dir[0])
                .checked_abs()
                .expect("Should not be negative") as usize;
            next_p.y = (next_p.y as isize + dir[1])
                .checked_abs()
                .expect("Should not be negative") as usize;
            next_tile = grid.get(&next_p);
        }

        (np, tile.expect("Must be a valid Tile").clone())
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Hash for Point {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

struct Cube {}

pub fn run(mut input: Lines<impl BufRead>) {
    let line_map = common::parse_mut(&mut input);
    let line_moves = common::parse_mut(&mut input);

    let state = process(&line_map, &line_moves);

    let x = walking_warp(&state);

    println!("x = {x}");
}

fn walking_warp(state: &(Point, HashMap<Point, Tile>, Vec<Move>)) -> usize {
    let grid = &state.1;
    let moves = &state.2;

    let mut cur_pos = state.0.clone();
    let mut cur_dir = Direction::Right;

    for mv in moves {
        match mv {
            Move::R => {
                cur_dir = cur_dir.turn_right();
            }
            Move::L => {
                cur_dir = cur_dir.turn_left();
            }
            Move::F(amnt) => {
                for _ in 0..(*amnt) {
                    let (next_pos, tile) = cur_pos.warped_shift_by_one(grid, &cur_dir);
                    match tile {
                        Tile::Wall => break,
                        Tile::Dirt => {
                            cur_pos = next_pos;
                        }
                    }
                }
            }
        }
    }

    // let tile = grid.get(&cur_pos);

    // if cur_dir.value() == 0 {}

    (1000 * cur_pos.y) + (4 * cur_pos.x) + cur_dir.value()
}

fn process(lmap: &[String], lmov: &[String]) -> (Point, HashMap<Point, Tile>, Vec<Move>) {
    let mut v = vec![];
    let mut pmap = HashMap::<Point, Tile>::new();
    let mut start = None;
    let mut height = 0;
    let mut width = 0;
    for (y, l) in lmap.iter().enumerate() {
        v.push(vec![]);
        for (x, chr) in l.chars().enumerate() {
            let p = Point { x: x + 1, y: y + 1 };
            match chr {
                '#' => {
                    v[y].push((p.clone(), Tile::Wall));
                    pmap.insert(p, Tile::Wall);
                    height = height.max(y + 1);
                    width = width.max(x + 1);
                }
                '.' => {
                    v[y].push((p.clone(), Tile::Dirt));
                    pmap.insert(p.clone(), Tile::Dirt);
                    height = height.max(y + 1);
                    width = width.max(x + 1);
                    if start.is_none() {
                        start = Some(p);
                    }
                }
                _ => {}
            }
        }
    }

    //// Movement
    let mut mov = vec![];
    let mut pstr = String::from("");
    for l in lmov[0].chars() {
        if l == 'L' {
            mov.push(Move::F(pstr.parse::<usize>().unwrap()));
            mov.push(Move::L);
            pstr = "".into();
        } else if l == 'R' {
            mov.push(Move::F(pstr.parse::<usize>().unwrap()));
            mov.push(Move::R);
            pstr = "".into();
        } else {
            pstr.push(l);
        }
    }

    if pstr.len() > 0 {
        mov.push(Move::F(pstr.parse::<usize>().unwrap()));
    }
    ////

    //// Cube pre-processing (folding)
    let start = start.expect("Not None");
    // Cube size is the biggest common factor of input dimension
    let cube_size = common::gcd(width, height);

    let mut quad_ids = HashMap::<(usize, usize), usize>::new();
    let mut cube_face_index = BTreeMap::<(usize, usize), Vec<Point>>::new();
    for vv in v.iter() {
        for (pt, _) in vv.iter() {
            let key = ((pt.x - 1) / cube_size, (pt.y - 1) / cube_size);
            (*cube_face_index.entry(key.clone()).or_insert(vec![])).push(pt.clone());

            let id_len = quad_ids.len();
            quad_ids.entry(key.clone()).or_insert_with(|| id_len);
        }
    }

    // Direction cycle counter-clock-wise starting from DOWN(0) -> RIGHT(1) -> UP(2) -> LEFT(3)
    const DX: &[isize; 4] = &[0, 1, 0, -1];
    const DY: &[isize; 4] = &[1, 0, -1, 0];

    // (2, 0) Some(Point { x: 9, y: 1 }) Some(Point { x: 12, y: 4 })
    // (0, 1) Some(Point { x: 1, y: 5 }) Some(Point { x: 4, y: 8 })
    // (1, 1) Some(Point { x: 5, y: 5 }) Some(Point { x: 8, y: 8 })
    // (2, 1) Some(Point { x: 9, y: 5 }) Some(Point { x: 12, y: 8 })
    // (2, 2) Some(Point { x: 9, y: 9 }) Some(Point { x: 12, y: 12 })
    // (3, 2) Some(Point { x: 13, y: 9 }) Some(Point { x: 16, y: 12 })
    // # cube sample
    // # --------------
    // #        |20|
    // # --------------
    // #  |01|11|21|
    // # --------------
    // #        |22|32|
    // # --------------

    // (1, 0) Some(Point { x: 51, y: 1 }) Some(Point { x: 100, y: 50 })
    // (2, 0) Some(Point { x: 101, y: 1 }) Some(Point { x: 150, y: 50 })
    // (1, 1) Some(Point { x: 51, y: 51 }) Some(Point { x: 100, y: 100 })
    // (0, 2) Some(Point { x: 1, y: 101 }) Some(Point { x: 50, y: 150 })
    // (1, 2) Some(Point { x: 51, y: 101 }) Some(Point { x: 100, y: 150 })
    // (0, 3) Some(Point { x: 1, y: 151 }) Some(Point { x: 50, y: 200 })
    // # cube real input
    // # --------------
    // #     |10|20|
    // # --------------
    // #     |11|
    // # --------------
    // #  |02|12|
    // # --------------
    // #  |03|
    // # --------------

    // Stitching each cube's face, something like the comments above
    let mut quads_move = [[Option::<(usize, usize)>::None; 4]; 6];
    let mut rem_side = 6 * 4;
    for ((x, y), qid) in quad_ids.iter() {
        for d in 0..4 {
            let nx = *x as isize + DX[d];
            let ny = *y as isize + DY[d];
            if nx < 0 || ny < 0 {
                continue;
            }

            if let Some(&sqid) = quad_ids.get(&(nx as usize, ny as usize)) {
                assert_ne!(*qid, sqid);

                quads_move[*qid][d] = Some((sqid, d));

                rem_side -= 1;
            }
        }
    }

    while rem_side > 0 {
        for sid in 0..6 {
            for dir in 0..4 {
                if quads_move[sid][dir].is_some() {
                    continue;
                }

                // reverse
                let rdi = (dir + 3) % 4;

                if let Some((direct_adj_face, access_dir)) = quads_move[sid][rdi] {
                    let rot_dir = (access_dir + 1) % 4;

                    if let Some((new_adj_face, origin_access_dir)) =
                        quads_move[direct_adj_face][rot_dir]
                    {
                        let dir_perspective = (origin_access_dir + 3) % 4;
                        quads_move[sid][dir] = Some((new_adj_face, dir_perspective));
                        rem_side -= 1;
                    }
                }
            }
        }
    }

    let cube_face_move_map = {
        let mut fmap = [[(0, 0); 4]; 6];
        for si in 0..6 {
            for di in 0..4 {
                fmap[si][di] = quads_move[si][di].unwrap();
            }
        }
        fmap
    };

    for c in cube_face_index.iter() {
        println!("{:?} {:?} {:?}", c.0, c.1.iter().min(), c.1.iter().max());
    }
    ////

    (start, pmap, mov)
}
