use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader, Lines},
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ElveRank {
    total_cal: i64,
    total_snack: i64,
}

fn main() {
    let file = File::open("./input-1").unwrap();
    let reader = BufReader::new(file);

    let elve_list: BTreeMap<ElveRank, i32> = process_sorted(reader.lines());

    println!("{:=>10}", "");
    let mut top_sum = 0;
    let mut iter = elve_list.iter().rev();
    for i in 1..4 {
        let el = iter.next().unwrap();
        let rank = el.0;
        let elv_id = el.1;
        top_sum += rank.total_cal;
        println!(
            "#{:0>3} elve_id={:<4} total_snack={:<4} total_cal={:<12}",
            i, elv_id, rank.total_snack, rank.total_cal
        )
    }
    println!("total={top_sum}")
}

fn process_sorted(input: Lines<impl BufRead>) -> BTreeMap<ElveRank, i32> {
    let mut elve_list: BTreeMap<ElveRank, i32> = BTreeMap::new();

    {
        let mut elv_n = 0_i32;
        let mut sn_cnt = 0_i64;
        let mut cal_sum = 0_i64;

        for line in input {
            let l = line.unwrap();

            if l == "" {
                elv_n += 1;

                elve_list.insert(
                    ElveRank {
                        total_cal: cal_sum,
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

    elve_list
}
