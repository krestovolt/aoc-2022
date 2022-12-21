use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, Lines},
};

use crate::common;

#[derive(Clone, Debug)]
enum Expr {
    Add,
    Subtract,
    Multiply,
    Divide,
    Constant(i64),
}

impl Expr {
    fn get_val(&self) -> Option<i64> {
        if let Expr::Constant(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    fn inv(&self) -> Self {
        match self {
            Expr::Add => Expr::Subtract,
            Expr::Subtract => Expr::Add,
            Expr::Multiply => Expr::Divide,
            Expr::Divide => Expr::Multiply,
            // not likely
            Expr::Constant(v) => Expr::Constant(*v),
        }
    }

    fn eval(&self, a: i64, b: i64) -> i64 {
        match self {
            Expr::Add => a + b,
            Expr::Subtract => a - b,
            Expr::Multiply => a * b,
            Expr::Divide => a / b,
            // not likely
            Expr::Constant(v) => *v,
        }
    }
}

pub fn run(input: Lines<impl BufRead>) {
    let lines = common::parse(input);

    let deps = process(&lines);

    let (_sorted_node, mut eval_map, humn) = compute(&deps);

    println!("root: {}", eval_map["root"]);

    let mut dh: Vec<String> = vec!["".into(); humn.len()];
    for d in humn.iter() {
        eval_map.insert(d.0.into(), i64::MAX);
        dh[humn.len() - *d.1 - 1] = d.0.into()
    }

    let mut humn_val = 0;
    let mut inv_op = Expr::Constant(0);

    for node in dh.iter() {
        if node == "root" {
            let v = deps.get(node).unwrap();
            humn_val =
                v.1.iter()
                    .filter(|v| !humn.contains_key(v.0))
                    .map(|v| eval_map[v.0])
                    .next()
                    .unwrap();

            println!("v={humn_val}");
            continue;
        }

        if node == "humn" {
            break;
        }

        let dep_val = deps.get(node).unwrap();
        let op = dep_val.0.clone();
        let (val, position) = dep_val
            .1
            .iter()
            .filter(|v| !humn.contains_key(v.0))
            .map(|v| (eval_map[v.0], *v.1))
            .take(1)
            .next()
            .unwrap();

        if val == i64::MAX {
            panic!("INVALID");
        }

        // Handling non-commutative expr
        let a = match op {
            Expr::Divide | Expr::Subtract => {
                let val_is_lhs = position == 0;
                if val_is_lhs {
                    let r = op.eval(val, humn_val);
                    // println!("[NOT] {r} = val({val})({:?})humn_val({humn_val})", op);
                    r
                } else {
                    let r = op.inv().eval(humn_val, val);
                    // println!("[INV] {r} = humn_val({humn_val})({:?})val({val})", op.inv());
                    r
                }
            }

            Expr::Add | Expr::Multiply => {
                let r = op.inv().eval(humn_val, val);
                // println!("[INV] {r} = humn_val({humn_val})({:?})val({val})", op.inv());
                r
            }

            _ => unimplemented!(),
        };

        humn_val = a;
    }

    // println!("{}={}", eval_map["gvhs"], eval_map["bzrn"]);

    println!("humn = {}", humn_val);
}

fn compute(
    deps: &HashMap<String, (Expr, HashMap<String, u8>)>,
) -> (Vec<String>, HashMap<String, i64>, HashMap<String, usize>) {
    let mut topo_sorted: Vec<String> = vec![];
    let mut idx = 1;
    let mut dependen_hmn: HashMap<String, usize> = HashMap::new();
    dependen_hmn.insert("humn".into(), 0);

    // Topological Sorting part
    {
        let mut graph = HashMap::<String, (Expr, HashMap<String, u8>)>::new();
        for node in deps.iter() {
            graph.insert(node.0.clone(), (node.1 .0.clone(), node.1 .1.clone()));
        }

        let mut resolved_node = HashSet::<String>::new();
        while !graph.is_empty() {
            for node in graph.iter() {
                if node.1 .1.is_empty() {
                    resolved_node.insert(node.0.clone());
                }
            }

            if resolved_node.len() == 0 {
                panic!("circular");
            }

            for node in resolved_node.iter() {
                let _removed = graph.remove(node).unwrap();

                topo_sorted.push(node.into());
            }

            for node in graph.iter_mut() {
                for node_resolved in resolved_node.iter() {
                    let removed = node.1 .1.remove(node_resolved);
                    if dependen_hmn.contains_key(node_resolved) && removed.is_some() {
                        dependen_hmn.insert(node.0.into(), idx);
                        idx += 1;
                    }
                }
            }

            resolved_node.clear();
        }
    }

    let mut eval_res = HashMap::<String, i64>::new();

    // Evaluating expression based on topo-sorted order
    {
        for node in topo_sorted.iter() {
            let dep = deps.get(node).unwrap();
            if dep.1.is_empty() {
                eval_res.insert(node.into(), dep.0.get_val().unwrap());
            } else {
                let mut di = dep.1.iter();
                let node_a = di.next().unwrap();
                let node_b = di.next().unwrap();

                let val_lhs = if *node_a.1 == 0 {
                    eval_res.get(node_a.0).copied().unwrap()
                } else {
                    eval_res.get(node_b.0).copied().unwrap()
                };

                let val_rhs = if *node_b.1 == 1 {
                    eval_res.get(node_b.0).copied().unwrap()
                } else {
                    eval_res.get(node_a.0).copied().unwrap()
                };

                let val = dep.0.eval(val_lhs, val_rhs);

                eval_res.insert(node.into(), val);
            }
        }
    }

    (topo_sorted, eval_res, dependen_hmn)
}

fn process(lines: &[String]) -> HashMap<String, (Expr, HashMap<String, u8>)> {
    let mut deps = HashMap::<String, (Expr, HashMap<String, u8>)>::new();

    for line in lines {
        let mut splt = line.split(":").map(|s| s.trim());

        let node_name = splt.next().unwrap();
        let expr = splt
            .next()
            .unwrap()
            .split(" ")
            .map(|s| s.trim())
            .collect::<Vec<_>>();

        if expr.len() == 1 {
            deps.insert(
                node_name.into(),
                (
                    Expr::Constant(expr[0].parse::<i64>().unwrap()),
                    HashMap::new(),
                ),
            );
        } else {
            let ex = match expr[1] {
                "+" => Expr::Add,
                "-" => Expr::Subtract,
                "*" => Expr::Multiply,
                "/" => Expr::Divide,
                _ => unimplemented!(),
            };

            deps.insert(
                node_name.into(),
                (
                    ex,
                    HashMap::from_iter(vec![(expr[0].into(), 0), (expr[2].into(), 1)]),
                ),
            );
        }
    }

    deps
}
