// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

// use anyhow::anyhow;
use anyhow::Result;
use std::io::{stdin, BufRead};

use day06::*;

fn process(maxdist: usize, bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    let xsum: usize = input.iter().map(|qa| qa.tuple().0 as usize).sum();
    let ysum: usize = input.iter().map(|qa| qa.tuple().1 as usize).sum();
    let len = input.len();
    let center = Qa::try_from(((xsum / len) as u16, (ysum / len) as u16))?;
    let mut ans = 1;
    for front in Sqrid::bf_iter(sqrid::qaqr_eval, &center) {
        let mut done = true;
        for (qa, _) in front {
            let dist: usize = input.iter().map(|center| Qa::manhattan(center, qa)).sum();
            if dist < maxdist {
                ans += 1;
                done = false;
            }
        }
        if done {
            break;
        }
    }
    Ok(ans)
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(32, EXAMPLE.as_bytes())?, 16);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(10000, stdin().lock())?);
    Ok(())
}
