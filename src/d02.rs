use std::io::{BufRead, Lines};

use crate::common;

const NORM_A: u8 = b'A';
const NORM_X: u8 = b'X';

const LOSE: u8 = 0; // 'X' - 'X'
const DRAW: u8 = 1; // 'Y' - 'X'
const WIN: u8 = 2; // 'Z' - 'X'

const ROCK: u8 = 0;
const PAPER: u8 = 1;
const SCISSORS: u8 = 2;

/// Result Point:
/// - Lose = 0
/// - DRAW = 3
/// - Win = 6
const RESULT_SCORING: [i32; 3] = [0, 3, 6];
const SHAPE_SCORING: [i32; 3] = [1, 2, 3];

/// Game outcome mapping from 'me' perspective
///
/// | score | op | me | (alias) | Name
/// |------ |----|----|---------|------
/// |   1   | A  | X  |   (0)   | Rock
/// |   2   | B  | Y  |   (1)   | Paper
/// |   3   | C  | Z  |   (2)   | Scissors
const OP_VS_ME_RESULT: [[u8; 3]; 3] = [
    // op:Rock vs
    [
        DRAW, // me:Rock
        WIN,  // me:Paper
        LOSE, // me:Scissors
    ],
    // op:Paper vs
    [
        LOSE, // me:Rock
        DRAW, // me:Paper
        WIN,  // me:Scissors
    ],
    // op:Scissors vs
    [
        WIN,  // me:Rock
        LOSE, // me:Paper
        DRAW, // me:Scissors
    ],
];

const OP_VS_ME_FOR_RESULT: [[u8; 3]; 3] = [
    // op:Rock
    [
        SCISSORS, // need:Lose
        ROCK,     // need:Draw
        PAPER,    // need:Win
    ],
    // op:Paper
    [
        ROCK,     // need:Lose
        PAPER,    // need:Draw
        SCISSORS, // need:Win
    ],
    // op:Scissors
    [
        PAPER,    // need:Lose
        SCISSORS, // need:Draw
        ROCK,     // need:Win
    ],
];

struct GameState {
    opp_ch: u8,
    m_ch: u8,
}

impl GameState {
    fn get_result(&self) -> u8 {
        OP_VS_ME_RESULT[self.opp_ch as usize][self.m_ch as usize]
    }

    fn get_score(&self) -> i32 {
        SHAPE_SCORING[self.m_ch as usize] + RESULT_SCORING[self.get_result() as usize]
    }

    /// In v2, the `m_ch` field is treated as the target result
    fn get_score_v2(&self) -> i32 {
        let m_ch_real = OP_VS_ME_FOR_RESULT[self.opp_ch as usize][self.m_ch as usize];

        SHAPE_SCORING[m_ch_real as usize] + RESULT_SCORING[self.m_ch as usize]
    }
}

pub fn run(input: Lines<impl BufRead>) {
    let strs = common::parse(input);

    let states = process(&strs);

    compute(&states);
    compute_2(&states);
}

fn compute(states: &[GameState]) {
    let mut score = 0;
    for s in states {
        score += s.get_score();
    }

    println!("total_score = {}", score);
}

fn compute_2(states: &[GameState]) {
    let mut score = 0;
    for s in states {
        score += s.get_score_v2();
    }

    println!("total_score_v2 = {}", score);
}

fn process(lines: &[String]) -> Vec<GameState> {
    let mut gs = vec![];

    for l in lines {
        let lbs = l.as_bytes();

        let g = GameState {
            opp_ch: lbs[0] - NORM_A,
            m_ch: lbs[2] - NORM_X,
        };

        gs.push(g);
    }

    gs
}
