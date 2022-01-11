// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use std::collections;
use std::io::{stdin, BufRead};

use day02::*;

fn process(bufin: impl BufRead) -> Result<i32> {
    let codes = parse(bufin)?;
    let mut twos = 0;
    let mut threes = 0;
    for code in codes {
        let freqs = code
            .chars()
            .fold(collections::HashMap::new(), |mut freqs, c| {
                let e = freqs.entry(c).or_insert(0);
                *e += 1;
                freqs
            });
        if freqs.values().any(|i| *i == 2) {
            twos += 1;
        }
        if freqs.values().any(|i| *i == 3) {
            threes += 1;
        }
    }
    Ok(twos * threes)
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE1.as_bytes())?, 12);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
