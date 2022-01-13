// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use std::collections::HashMap;
use std::io::{stdin, BufRead};

use day03::*;

fn process(bufin: impl BufRead) -> Result<usize> {
    let claims = parser::parse(bufin)?;
    let mut map = HashMap::new();
    for (_, (x0, y0), (w, h)) in claims {
        for x in x0..(x0 + w) {
            for y in y0..(y0 + h) {
                let e = map.entry((x, y)).or_insert(0_usize);
                *e += 1;
            }
        }
    }
    Ok(map.values().filter(|&&v| v > 1).count())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 4);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
