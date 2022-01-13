// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::anyhow;
use anyhow::Result;
use std::collections;
use std::io::{stdin, BufRead};

use day03::*;

fn process(bufin: impl BufRead) -> Result<usize> {
    let claims = parser::parse(bufin)?;
    let mut map = collections::HashMap::new();
    let claimids = claims
        .iter()
        .cloned()
        .map(|(i, _, _)| i)
        .collect::<collections::HashSet<_>>();
    let mut overlaps = collections::HashSet::new();
    for (id, (x0, y0), (w, h)) in claims {
        for x in x0..(x0 + w) {
            for y in y0..(y0 + h) {
                let e = map.entry((x, y)).or_insert_with(Vec::new);
                e.push(id);
                if e.len() > 1 {
                    overlaps.extend(e.iter());
                }
            }
        }
    }
    let nonoverlap = claimids.difference(&overlaps);
    nonoverlap
        .into_iter()
        .cloned()
        .next()
        .ok_or_else(|| anyhow!("non-overlapping id not found"))
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 3);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
