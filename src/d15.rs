use std::io::{BufRead, Lines};

const Y_MAX: isize = 4000000;

struct SensorInfo {
    be_dist: isize,
    pos: [isize; 2],
}

use crate::common;

pub fn run(input: Lines<impl BufRead>) {
    let lines = common::parse(input);
    let (sensors, pos_y) = process(&lines);

    let segments = create_segments(pos_y, &sensors);

    let mut covered_area = 0;
    let mut prev_end = isize::MIN;
    for sg in segments.iter() {
        let width = sg[1] - sg[0].max(prev_end);
        if width > 0 {
            covered_area += width;
        }
        prev_end = sg[1].max(prev_end);
    }

    let mut distress_beacon = [-1_isize, -1_isize];
    // trying to refactor using this approach
    // https://www.reddit.com/r/adventofcode/comments/zmfwg1/2022_day_15_part_2_seekin_for_the_beacon
    //
    // and this
    // https://github.com/tdpetrou/Advent-of-Code-Pandas/blob/master/2022/Problems.ipynb
    let bounds = sensors
        .iter()
        .map(|s| {
            let d = s.be_dist;
            let top = [s.pos[0], s.pos[1] - d - 1];
            let right = [s.pos[0] + d + 1, s.pos[1]];
            let bottom = [s.pos[0], s.pos[1] + d + 1];
            let left = [s.pos[0] - d - 1, s.pos[1]];

            [top, right, bottom, left]
        })
        .collect::<Vec<_>>();

    let perimeter_move: [[isize; 2]; 4] = [[1, 1], [-1, 1], [-1, -1], [1, -1]];

    let in_radius = |p1: isize, p2: isize, d: isize| -> bool { (p1 - p2).abs() < d };
    let point_valid = |x: isize, y: isize| -> bool { 0 <= x && x <= Y_MAX && 0 <= y && y <= Y_MAX };

    for bi in 0..bounds.len() {
        let sdist = sensors[bi].be_dist;
        let dirs = &bounds[bi];

        for (p, m) in dirs.iter().zip(perimeter_move.iter()) {
            let mut pc = p.clone();

            while point_valid(pc[0], pc[1]) && in_radius(pc[0], p[0], sdist) {
                let mut mdist = sensors
                    .iter()
                    .map(|s| {
                        let d_to_pc = (s.pos[0] - pc[0]).abs() + (s.pos[1] - pc[1]).abs();
                        s.be_dist - d_to_pc
                    })
                    .max()
                    .unwrap();

                if mdist < 0 {
                    distress_beacon[0] = pc[0];
                    distress_beacon[1] = pc[1];
                    break;
                } else {
                    // center/scaling factor for new pos
                    mdist = (mdist / 2).max(1);
                }
                pc[0] = pc[0] + m[0] * mdist;
                pc[1] = pc[1] + m[1] * mdist
            }
        }
    }

    println!("covered at y={pos_y} : {covered_area}");
    println!(
        "distress beacon at ({}, {})",
        distress_beacon[0], distress_beacon[1]
    );
    println!(
        "tuning frequency = {}",
        distress_beacon[0] * Y_MAX + distress_beacon[1]
    );
}

fn create_segments(pos_y: isize, sensors: &[SensorInfo]) -> Vec<[isize; 2]> {
    let mut segments = vec![];

    for si in 0..sensors.len() {
        let s = &sensors[si];
        let triangle_h = s.be_dist - (s.pos[1] - pos_y).abs();
        if triangle_h > -1 {
            // Add segment start-end x-coordinate
            segments.push([s.pos[0] - triangle_h, s.pos[0] + triangle_h]);
        }
    }
    // Sort by lowest segment's start position
    segments.sort_by(|a, b| a[0].cmp(&b[0]));

    segments
}

fn process(lines: &[String]) -> (Vec<SensorInfo>, isize) {
    let mut beacon_dist: Vec<SensorInfo> = vec![];

    let objective_y = lines[0]
        .split(" ")
        .last()
        .map_or(-1, |s| s.trim().parse::<isize>().unwrap());

    for li in 1..lines.len() {
        let mut sen_bea = lines[li].split(":").map(|s| {
            let mut coords = s
                .trim()
                .split("at")
                .last()
                .unwrap()
                .split(",")
                .map(|coord| coord.split("=").last().unwrap().parse::<isize>().unwrap());

            [coords.next().unwrap(), coords.next().unwrap()]
        });

        let sensor = sen_bea.next().unwrap();
        let beacon = sen_bea.next().unwrap();
        let mdist = sensor[0].abs_diff(beacon[0]) + sensor[1].abs_diff(beacon[1]);

        beacon_dist.push(SensorInfo {
            pos: sensor,
            be_dist: mdist as isize,
        });
    }

    (beacon_dist, objective_y)
}
