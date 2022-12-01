use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ElveRank {
    total_calories: i64,
    total_snack: i64,
}

fn main() {
    let file = File::open("./input-1").unwrap();
    let reader = BufReader::new(file);

    let mut elve_list: BTreeMap<ElveRank, i32> = BTreeMap::new();

    {
        let mut elv_n = 0_i32;
        let mut sn_cnt = 0_i64;
        let mut cal_sum = 0_i64;

        for line in reader.lines() {
            let l = line.unwrap();

            if l == "" {
                elv_n += 1;

                elve_list.insert(
                    ElveRank {
                        total_calories: cal_sum,
                        total_snack: sn_cnt,
                    },
                    elv_n,
                );

                sn_cnt = 0;
                cal_sum = 0;
                continue;
            }

            let cal = l.parse::<i64>().unwrap();

            cal_sum += cal;
            sn_cnt += 1;
        }
    }

    println!("=============");

    let mut top_sum = 0;
    let mut iter = elve_list.iter().rev();
    for i in 1..4 {
        let el = iter.next().unwrap();
        top_sum += el.0.total_calories;
        println!("#{i} elv={} total={:?}", el.1, el.0)
    }
    println!("total={top_sum}")
}
