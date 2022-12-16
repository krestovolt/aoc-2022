use std::{
    collections::{hash_map::Entry, HashMap},
    io::{BufRead, Lines},
};

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
    for y in 0..=Y_MAX {
        let segments = create_segments(y, &sensors);
        let mut tentative_x = 0_isize;

        let mut found_valid_pos = true;

        for s in segments.iter() {
            if s[0] <= tentative_x && tentative_x <= s[1] {
                // Move potential x coord to the outside of current segment
                tentative_x = s[1] + 1;

                if tentative_x > Y_MAX {
                    found_valid_pos = false;
                    break;
                }
            }
        }

        if found_valid_pos {
            distress_beacon[0] = tentative_x;
            distress_beacon[1] = y;
            break;
        }
    }

    println!("covered at y={pos_y} : {covered_area}");
    println!("distress beacon at ({}, {})", distress_beacon[0], distress_beacon[1]);
    println!("tuning frequency = {}", distress_beacon[0] * Y_MAX + distress_beacon[1]);
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
