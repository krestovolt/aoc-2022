use std::io::{BufRead, Lines};

use crate::common;

enum OpSymbol {
    OpMult,
    OpAdd,
    ValOld,
    Val(u64),
}

impl OpSymbol {
    fn is_val_old(&self) -> bool {
        if let OpSymbol::ValOld = self {
            true
        } else {
            false
        }
    }

    fn get_value(&self) -> u64 {
        if let OpSymbol::Val(num) = self {
            *num
        } else {
            0
        }
    }
}

struct CMonkey<TF, THF>
where
    TF: Fn(u64) -> u64,
    THF: Fn(u64) -> usize,
{
    inspect_cnt: usize,
    items: Vec<u64>,
    super_mod: u64,
    transform_fn: TF,
    throw_fn: THF,
}

pub fn run(input: Lines<impl BufRead>) {
    let lines = common::parse(input);

    let mut p = process(&lines);
    let bak_items: Vec<Vec<u64>> = p.iter().map(|m| m.items.clone()).collect();

    let lb = compute(&mut p, true, 20);
    let lblen = lb.len();
    println!(
        "two most active: {} * {} = {}",
        lb[lblen - 1],
        lb[lblen - 2],
        lb[lblen - 1] * lb[lblen - 2]
    );

    for m in 0..p.len() {
        p[m].items = bak_items[m].clone();
        p[m].inspect_cnt = 0;
    }

    let lb = compute(&mut p, false, 10000);
    let lblen = lb.len();
    println!(
        "two most active: {} * {} = {}",
        lb[lblen - 1],
        lb[lblen - 2],
        lb[lblen - 1] * lb[lblen - 2]
    );
}

fn compute<'a>(
    mks: &mut [CMonkey<impl Fn(u64) -> u64 + 'a, impl Fn(u64) -> usize + 'a>],
    div3: bool,
    max_iter: usize,
) -> Vec<usize> {
    let mlen = mks.len();

    for _i in 0..max_iter {
        // println!("iter={i}");
        for mi in 0..mlen {
            let ilen = mks[mi].items.len();
            for x in 0..ilen {
                mks[mi].inspect_cnt += 1;
                let mut wr = (mks[mi].transform_fn)(mks[mi].items[x]);
                if div3 {
                    wr = wr / 3;
                } else {
                    wr = wr % mks[mi].super_mod;
                }
                let throw_target = (mks[mi].throw_fn)(wr);
                mks[throw_target].items.push(wr);
            }
            mks[mi].items.clear();
        }
    }

    let mut freqs = vec![0; mlen];
    for m in 0..mlen {
        println!("Monkey#{m} inspected items {} times", mks[m].inspect_cnt);
        freqs[m] = mks[m].inspect_cnt;
    }

    freqs.sort();

    freqs
}

fn process(lines: &[String]) -> Vec<CMonkey<impl Fn(u64) -> u64 + '_, impl Fn(u64) -> usize + '_>> {
    let len = lines.len() / 6;
    let mut mks = vec![];
    let mut smod = 1;
    for i in 0..len {
        // 6 lines per-monkey
        let items = &lines[(i * 6 + 1) as usize];
        let ops = &lines[(i * 6 + 2) as usize];
        let divisor = &lines[(i * 6 + 3) as usize];
        let true_cond = &lines[(i * 6 + 4) as usize];
        let false_cond = &lines[(i * 6 + 5) as usize];

        let items = items
            .split(":")
            .last()
            .unwrap()
            .split(",")
            .map(|s| s.trim().parse::<u64>().unwrap());

        let ops = ops
            .split(":")
            .last()
            .unwrap()
            .split("=")
            .last()
            .unwrap()
            .split(" ")
            .filter(|s| s.len() > 0)
            .map(|s| match s.trim() {
                "old" => OpSymbol::ValOld,
                "+" => OpSymbol::OpAdd,
                "*" => OpSymbol::OpMult,
                _ => OpSymbol::Val(s.parse::<u64>().unwrap()),
            });
        let ops: Vec<OpSymbol> = ops.collect();

        let divisor = divisor
            .split("divisible by")
            .last()
            .unwrap()
            .trim()
            .parse::<u64>()
            .unwrap();

        let true_cond = true_cond
            .split("throw to monkey")
            .last()
            .unwrap()
            .trim()
            .parse::<usize>()
            .unwrap();

        let false_cond = false_cond
            .split("throw to monkey")
            .last()
            .unwrap()
            .trim()
            .parse::<usize>()
            .unwrap();

        smod = smod * divisor;

        mks.push(CMonkey {
            inspect_cnt: 0,
            items: items.collect(),
            super_mod: 0,
            transform_fn: move |wl| -> u64 {
                let opr = &ops;

                let arg1 = &opr[0];
                let operator = &opr[1];
                let arg2 = &opr[2];

                let arg1_wl = arg1.is_val_old();
                let arg2_wl = arg2.is_val_old();

                match operator {
                    OpSymbol::OpAdd => {
                        if arg1_wl && arg2_wl {
                            2 * wl
                        } else {
                            let v = if arg1_wl {
                                arg2.get_value()
                            } else {
                                arg1.get_value()
                            };

                            wl + v
                        }
                    }

                    OpSymbol::OpMult => {
                        if arg1_wl && arg2_wl {
                            wl * wl
                        } else {
                            let v = if arg1_wl {
                                arg2.get_value()
                            } else {
                                arg1.get_value()
                            };

                            wl * v
                        }
                    }

                    _ => unimplemented!(),
                }
            },
            throw_fn: move |wl| -> usize {
                if wl % divisor == 0 {
                    true_cond
                } else {
                    false_cond
                }
            },
        })
    }

    for m in 0..mks.len() {
        mks[m].super_mod = smod;
    }

    mks
}
