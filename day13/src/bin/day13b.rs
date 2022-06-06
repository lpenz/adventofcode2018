// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use std::cmp::Reverse;
use std::collections::BTreeSet;
use std::io::{stdin, BufRead};

use day13::*;

pub const EXAMPLE: &str = r"/>-<\  
|   |  
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/
";

fn process(bufin: impl BufRead) -> Result<(u16, u16)> {
    let (g, mut carts) = parser::parse(bufin)?;
    while carts.len() > 1 {
        carts.sort_by_key(|c| {
            let t = c.qa.tuple();
            (t.1, t.0)
        });
        let mut dead = BTreeSet::new();
        for i1 in 0..carts.len() {
            let ri1 = Reverse(i1);
            if dead.contains(&ri1) {
                continue;
            }
            carts[i1].eval(&g)?;
            for (i2, c2) in carts.iter().enumerate() {
                let ri2 = Reverse(i2);
                if i1 == i2 || dead.contains(&ri2) {
                    continue;
                }
                if carts[i1].qa == c2.qa {
                    dead.insert(ri1);
                    dead.insert(ri2);
                }
            }
        }
        for i in dead.into_iter() {
            carts.remove(i.0);
        }
    }
    Ok(carts[0].qa.tuple())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, (6, 4));
    Ok(())
}

fn main() -> Result<()> {
    println!("{:?}", process(stdin().lock())?);
    Ok(())
}
