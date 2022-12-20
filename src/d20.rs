use std::io::{BufRead, Lines};

use crate::common;

pub fn run(input: Lines<impl BufRead>) {
    let lines = common::parse(input);

    let (nums, indexes) = process(&lines);

    let key = 1;
    let iter = 1;
    let triplet_seq = decrypt(&nums, indexes.clone(), iter, key);
    let sum_nums = triplet_seq.iter().sum::<i128>();
    println!(
        "sum from {:?} (iter={iter}, key={key}) = {}",
        triplet_seq, sum_nums
    );

    let key = 811589153;
    let iter = 10;
    let triplet_seq = decrypt(&nums, indexes.clone(), iter, key);
    let sum_nums = triplet_seq.iter().sum::<i128>();
    println!(
        "sum from {:?} (iter={iter}, key={key}) = {}",
        triplet_seq, sum_nums
    );
}

fn decrypt(
    nums: &[isize],
    mut indexes: Vec<usize>,
    iteration: usize,
    decryption_key: i128,
) -> Vec<i128> {
    let indexes = &mut indexes;
    let len = nums.len();
    for _iter in 0..iteration {
        for num_i in 0..len {
            let num_pos = indexes.iter().position(|idx| *idx == num_i).unwrap() as i128;
            let num_val = (nums[num_i] as i128) * decryption_key;

            let modulo = (len as i128) - 1;
            let fut_index = num_pos + num_val;
            let fut_index = if fut_index < 0 {
                let a = fut_index;
                // divide and round up (ceil)
                let b = (a + 1) / modulo - 1;
                // modulo produce:
                // (-21) % 4
                // b = ((-21) + 1) / 4 - 1 = (-5) - 1 = (-6)
                // (-21) - 4 * (-6) = (-21) + 24 = 3
                // same result with rem_euclid
                a - modulo * b
            } else {
                fut_index % modulo
            };

            let fut_index = fut_index as usize;
            let num_pos = num_pos as usize;

            indexes.remove(num_pos as usize);
            indexes.insert(fut_index, num_i);
        }
    }

    // for i in indexes.iter() {
    //     print!("{} ", nums[*i]);
    // }
    // println!();
    let zero = indexes.iter().position(|v| nums[*v] == 0).unwrap();

    [1000, 2000, 3000]
        .iter()
        .map(|v| nums[indexes[(*v + zero) % len]] as i128 * decryption_key)
        .collect::<Vec<_>>()
}

fn process(lines: &[String]) -> (Vec<isize>, Vec<usize>) {
    let mut nums = vec![];
    let mut indexes = vec![];

    let mut index = 0usize;
    for line in lines {
        let n = line.parse::<isize>().unwrap();

        nums.push(n);
        indexes.push(index);

        index += 1;
    }

    (nums, indexes)
}
