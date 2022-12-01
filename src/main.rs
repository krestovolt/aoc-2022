use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("./input-1").unwrap();
    let reader = BufReader::new(file);

    let mut ln = 0;

    let mut chosen_elve = -1_i64;
    let mut max_cal_sum = 0_i64;

    let mut elv_n = 0_i64;
    let mut cal_sum = 0_i64;

    println!("elve#{elv_n}");
    for line in reader.lines() {
        let l = line.unwrap();

        if l == "" {
            ln = 0;
            elv_n += 1;

            println!("elve={elv_n}\tcal={cal_sum}");

            choose_max(&mut chosen_elve, &mut max_cal_sum, elv_n, cal_sum);

            cal_sum = 0;
            continue;
        }

        let cal = l.parse::<i64>().unwrap();
        ln += 1;

        cal_sum += cal;
    }

    println!("=======");
    println!("final_elve={chosen_elve}\tfinal_cal={max_cal_sum}")
}

fn choose_max(me: &mut i64, msum: &mut i64, ce: i64, csum: i64) {
    if *msum < csum {
        *me = ce;
        *msum = csum;
    }
}
