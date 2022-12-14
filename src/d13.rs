use std::{
    cmp::Ordering,
    fmt::{Debug, Display},
    io::{BufRead, Lines},
    slice,
};

use crate::common;

#[derive(Clone, Eq)]
enum ListEl {
    Num(i32),
    Nest(Vec<ListEl>),
}

impl ListEl {
    /// Parses the input string iteratively instead of recursive by using additional `Vec<ListEl>`
    ///
    /// ## Warning
    /// The `input` is assumed to be properly formatted (no missing `[` or `]`), in case the `input` is not properly formatted,
    /// there will be some undefined-behavior or incorrect parsing result.
    ///
    /// ## Todo
    /// Add open/close validate
    fn parse(input: &str) -> Vec<ListEl> {
        let mut root: Vec<ListEl> = vec![];
        let chrs = input.as_bytes();
        let mut lvl = 0;

        let mut working_stack = vec![];
        let mut acc = vec![];
        let mut i = 0;
        while i < chrs.len() {
            let c = chrs[i];
            if c == b'[' {
                lvl += 1;
            } else if c == b',' || c == b']' {
                if acc.len() > 0 {
                    let s = String::from_utf8(acc).unwrap().parse::<i32>().unwrap();
                    acc = vec![];

                    if lvl > 1 {
                        // Not at the root level, store the result to the working stack
                        working_stack.push((lvl, ListEl::Num(s)));
                    } else {
                        // At root, just store the value as is
                        root.push(ListEl::Num(s));
                    }
                }

                if c == b']' {
                    if lvl > 1 {
                        // Not at the root level and about to leave this depth/level
                        // reduce the value into a single element
                        let mut nest = vec![];
                        while let Some(last_item) = working_stack.last() {
                            if last_item.0 != lvl {
                                break;
                            }
                            let last_item = working_stack.pop().unwrap();
                            nest.push(last_item.1);
                        }
                        nest.reverse();

                        if lvl > 2 {
                            // Currently at the level deeper than 2, store the reduced value to the working stack
                            // using the parent level (1-level above)
                            working_stack.push((lvl - 1, ListEl::Nest(nest)));
                        } else {
                            // At level 2
                            // parent is root, just commit/store the reduced value to the root since we are going back to root
                            root.push(ListEl::Nest(nest));
                        }
                    }
                    lvl -= 1;
                }
            } else {
                acc.push(c);
            }
            i += 1;
        }

        // if working_stack.len() > 0 {}

        // println!("root: {:?}", root);

        root
    }

    pub fn as_slice(&self) -> &[ListEl] {
        if let ListEl::Nest(v) = self {
            v
        } else {
            slice::from_ref(self)
        }
    }
}

impl Display for ListEl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ListEl::Num(n) => write!(f, "{}", n),
            ListEl::Nest(v) => {
                if let Err(err) = write!(f, "[") {
                    return Err(err);
                }

                for i in 0..v.len() {
                    let sep = if i > 0 { ", " } else { "" };
                    if let Err(err) = write!(f, "{}{}", sep, v[i]) {
                        return Err(err);
                    }
                }
                if let Err(err) = write!(f, "]") {
                    return Err(err);
                }
                Ok(())
            }
        }
    }
}

impl Debug for ListEl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Ord for ListEl {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (ListEl::Num(a), ListEl::Num(b)) => a.cmp(b),

            _ => self.as_slice().cmp(other.as_slice()),
        }
    }
}

impl PartialOrd for ListEl {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ListEl {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

pub fn run(mut input: Lines<impl BufRead>) {
    let mut pair_lines = common::parse_mut(&mut input);
    let mut pairs = vec![];
    let mut pn = 1;
    let mut sum = 0;
    while pair_lines.len() > 0 {
        let (p1, p2, ok) = process(&pair_lines);

        if ok {
            sum += pn;
        }

        println!("{:?}\n{:?}\n=> #{pn} {}", p1, p2, ok);

        pairs.push(p1);
        pairs.push(p2);

        pair_lines = common::parse_mut(&mut input);
        pn += 1;
    }

    // [Nest([Num(2)])]
    let div_1 = vec![ListEl::Nest(vec![ListEl::Num(2)])];
    // [Nest([Num(6)])]
    let div_2 = vec![ListEl::Nest(vec![ListEl::Num(6)])];

    let mut pos_1 = 1;
    let mut pos_2 = pos_1 + 1;
    for p in pairs {
        if let Ordering::Less | Ordering::Equal = p.cmp(&div_1) {
            pos_1 += 1;
        }
        if let Ordering::Less | Ordering::Equal = p.cmp(&div_2) {
            pos_2 += 1;
        }
    }

    // for p in pairs {
    //     println!("{:?}", p);
    // }

    println!("sum index: {}", sum);
    println!("divider: {} * {} = {}", pos_1, pos_2, pos_1 * pos_2);
}

fn process(lines: &[String]) -> (Vec<ListEl>, Vec<ListEl>, bool) {
    if lines.len() != 2 {
        return (vec![], vec![], false);
    }

    let p1 = ListEl::parse(lines[0].as_str());

    let p2 = ListEl::parse(lines[1].as_str());

    let ok = p1 <= p2;

    (p1, p2, ok)
}
