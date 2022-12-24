use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, Lines},
};

use crate::common;

#[derive(Clone, Debug)]
enum Action {
    MoveUp = 0,
    MoveLeft = 1,
    MoveDown = 2,
    MoveRight = 3,
    Wait = 4,
}

impl Action {
    fn do_action(&self, pos: &(i32, i32)) -> ((i32, i32), (i32, i32)) {
        let old_pos = pos.clone();
        match self {
            Self::MoveUp => (old_pos, Direction::Up.next_pos(pos)),
            Self::MoveLeft => (old_pos, Direction::Left.next_pos(pos)),
            Self::MoveDown => (old_pos, Direction::Down.next_pos(pos)),
            Self::MoveRight => (old_pos, Direction::Right.next_pos(pos)),
            Self::Wait => (old_pos, old_pos),
        }
    }
}

#[derive(Clone, Debug)]
enum Direction {
    Up = 0,
    Left = 1,
    Down = 2,
    Right = 3,
}

impl Direction {
    fn next_pos(&self, pos: &(i32, i32)) -> (i32, i32) {
        match self {
            Self::Up => (pos.0, pos.1 - 1),
            Self::Down => (pos.0, pos.1 + 1),
            Self::Left => (pos.0 - 1, pos.1),
            Self::Right => (pos.0 + 1, pos.1),
        }
    }
}

#[derive(Clone, Debug)]
struct Bliz {
    x: i32,
    y: i32,
    dir: Direction,
}

impl Bliz {
    fn tick(&mut self, width: i32, height: i32) -> ((i32, i32), (i32, i32)) {
        let bound_x = width - 1;
        let bound_y = height - 1;

        let cur_pos = (self.x, self.y);
        let next_pos = {
            let np = self.dir.next_pos(&cur_pos);
            let nx = if np.0 >= bound_x {
                1
            } else if np.0 <= 0 {
                bound_x - 1
            } else {
                np.0
            };
            let ny = if np.1 >= bound_y {
                1
            } else if np.1 <= 0 {
                bound_y - 1
            } else {
                np.1
            };

            (nx, ny)
        };

        if next_pos.1 == 0 || next_pos.0 == 0 {
            panic!("Blizzard cannot be at wall position");
        }

        self.x = next_pos.0;
        self.y = next_pos.1;

        (cur_pos, next_pos)
    }
}

const ACTIONS: [Action; 5] = [
    Action::MoveUp,
    Action::MoveLeft,
    Action::MoveDown,
    Action::MoveRight,
    Action::Wait,
];

pub fn run(input: Lines<impl BufRead>) {
    let lines = common::parse(input);

    let (width, height, start, goal, new_bliz, new_tile_freq) = process(&lines);

    let (minimum_step_1, new_bliz, new_tile_freq) =
        simulate(&(width, height, start, goal, new_bliz, new_tile_freq));
    println!("1st Start to Goal = {minimum_step_1}");

    let (minimum_step_2, new_bliz, new_tile_freq) =
        simulate(&(width, height, goal, start, new_bliz, new_tile_freq));
    println!("2nd Goal to Start = {minimum_step_2}");

    let (minimum_step_3, _new_bliz, _new_tile_freq) =
        simulate(&(width, height, start, goal, new_bliz, new_tile_freq));
    println!("3rd Start to Goal = {minimum_step_3}");

    println!(
        "total minimum step = {}",
        minimum_step_1 + minimum_step_2 + minimum_step_3
    );
}

fn simulate(
    state: &(
        i32,
        i32,
        (i32, i32),
        (i32, i32),
        Vec<Bliz>,
        HashMap<(i32, i32), i32>,
    ),
) -> (usize, Vec<Bliz>, HashMap<(i32, i32), i32>) {
    let width = state.0;
    let height = state.1;

    let start = state.2.clone();
    let goal = state.3.clone();

    let mut blizs = state.4.clone();
    let mut tile_bliz_freq = state.5.clone();

    let mut t = 0usize;

    let valid_pos = |new_pos: (i32, i32)| {
        (new_pos.0 >= 1 && new_pos.0 <= width - 2 && new_pos.1 >= 1 && new_pos.1 <= height - 2)
            || new_pos == start
            || new_pos == goal
    };

    let mut action_q = HashSet::<(i32, i32)>::new();
    let mut action_tmp_q = HashSet::<(i32, i32)>::new();
    action_q.insert(start);

    while !action_q.contains(&goal) {
        t += 1;

        // Moves those blizzards first
        for b in blizs.iter_mut() {
            let (old_pos, cur_pos) = b.tick(width, height);
            *tile_bliz_freq.entry(old_pos).or_insert_with(|| 1) -= 1;
            *tile_bliz_freq.entry(cur_pos).or_insert_with(|| 0) += 1;
        }

        // Search valid movement after all of the blizzards moved
        for pos in action_q.drain() {
            for act in ACTIONS.iter() {
                let (_old_pos, new_pos) = act.do_action(&pos);

                let num_of_blizz = tile_bliz_freq.get(&new_pos).copied().unwrap_or(0);
                if valid_pos(new_pos) && num_of_blizz == 0 {
                    action_tmp_q.insert(new_pos);
                }
            }
        }
        action_q.extend(action_tmp_q.drain());
    }

    (t, blizs, tile_bliz_freq)
}

fn process(
    lines: &[String],
) -> (
    i32,
    i32,
    (i32, i32),
    (i32, i32),
    Vec<Bliz>,
    HashMap<(i32, i32), i32>,
) {
    let start = (1, 0);
    let width = lines[0].len() as i32;
    let height = lines.len() as i32;
    let end = (width - 2, height - 1);

    let mut freq = HashMap::<(i32, i32), i32>::new();
    let mut bls = vec![];

    for (y, line) in lines.iter().enumerate() {
        for (x, chr) in line.chars().enumerate() {
            if chr == '.' || chr == '#' {
                continue;
            }

            let x = x as i32;
            let y = y as i32;

            let dir = match chr {
                '^' => Direction::Up,
                '>' => Direction::Right,
                'v' => Direction::Down,
                '<' => Direction::Left,
                _ => unreachable!(),
            };
            bls.push(Bliz {
                x: x as i32,
                y: y as i32,
                dir,
            });

            *freq.entry((x, y)).or_insert_with(|| 0) += 1;
        }
    }

    (width, height, start, end, bls, freq)
}
