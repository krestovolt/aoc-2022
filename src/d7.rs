use std::{
    collections::{hash_map, HashMap},
    io::{BufRead, Lines},
};

use crate::common;

const TOTAL_SIZE: u64 = 70_000_000;
const NEED_SIZE: u64 = 30_000_000;

/// A representation of file-sys Dir node.
///
/// Field `size` behavior:
///  will be a total size from every child nodes.
struct DirNode {
    parent_index: usize,
    size: u64,
    _name: String,
}

/// A representation of file-sys Dir node.
///
/// Field `size` behavior:
///  will be the size of current file.
struct FileNode {
    dir_index: usize,
    size: u64,
    _name: String,
}

struct FsInfo {
    dirs: Vec<DirNode>,
    files: Vec<FileNode>,
}

pub fn run(input: Lines<impl BufRead>) {
    let lines = common::parse(input);

    let mut nds = process(&lines);
    fill_size(&mut nds);
    // get at most size 100000
    let sum: u64 = nds
        .dirs
        .iter()
        .filter(|d| d.size <= 100000)
        .fold(0_u64, |acc, d| acc + d.size);
    let root_size = nds.dirs.first().unwrap().size;
    let need_delete = NEED_SIZE - (TOTAL_SIZE - root_size);
    println!("root size: {}", root_size);
    println!("needed: {}", NEED_SIZE);
    println!("free: {}", TOTAL_SIZE - root_size);
    println!("delete: {}", NEED_SIZE - (TOTAL_SIZE - root_size));
    println!("sum (<= 100k) size: {sum}");

    let mut deltas: Vec<(u64, u64, bool)> = nds
        .dirs
        .iter()
        .map(|d| {
            if d.size < need_delete {
                (0, d.size, false)
            } else {
                (d.size - need_delete, d.size, true)
            }
        })
        .filter(|tpl| tpl.2)
        .collect();
    deltas.sort();

    println!("{:?}", deltas[0])
}

fn fill_size(info: &mut FsInfo) {
    // bottom up from leaf to its parent (file -> dir -> parent dir)
    let vf: &Vec<FileNode> = info.files.as_ref();
    for file in vf {
        let mut par = file.dir_index;
        let mut prev_par = info.dirs.len() + 1;
        while prev_par != par {
            let mut pref = &mut info.dirs[par];
            pref.size += file.size;

            prev_par = par;
            par = pref.parent_index
        }
    }
}

fn process(lines: &[String]) -> FsInfo {
    let mut vd: Vec<DirNode> = vec![];
    let mut vf: Vec<FileNode> = vec![];

    let mut stack: Vec<String> = vec![];
    let mut name_idx: HashMap<String, usize> = hash_map::HashMap::new();

    vd.push(DirNode {
        _name: "".into(),
        parent_index: 0,
        size: 0,
    });
    name_idx.insert("".into(), 0);

    for line in lines {
        let l = line.split(" ");
        let l: Vec<&str> = l.collect();

        if l[0].starts_with("$") {
            // handle command
            if l[1] != "cd" {
                continue;
            }

            let name = l[2];
            if name == ".." {
                stack.pop();
            } else {
                if name == "/" {
                    stack.push("".into());
                } else {
                    stack.push(name.into());
                }
            }
        } else if l[0].starts_with("dir") {
            // handle directory
            let name = l[1];
            let kp = stack.join("/");
            let k = kp.clone() + "/" + name;

            if !name_idx.contains_key(&k) {
                let kpi = name_idx.get(&kp).unwrap();

                vd.push(DirNode {
                    parent_index: *kpi,
                    _name: k.clone(),
                    size: 0,
                });

                name_idx.insert(k, vd.len() - 1);
            }
        } else {
            // handle file
            let name = l[1];
            let sz = l[0].parse::<u64>().unwrap();
            let kp = stack.join("/");
            let k = kp.clone() + "/" + name;

            if !name_idx.contains_key(&k) {
                let kpi = name_idx.get(&kp).unwrap();

                vf.push(FileNode {
                    dir_index: *kpi,
                    _name: k.clone(),
                    size: sz,
                });

                name_idx.insert(k, vd.len() - 1);
            }
        }
    }

    FsInfo {
        dirs: vd,
        files: vf,
    }
}
