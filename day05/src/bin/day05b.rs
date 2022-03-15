// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::anyhow;
use anyhow::Result;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{stdin, BufRead};

use day05::*;

fn process(bufin: impl BufRead) -> Result<usize> {
    let input: VecDeque<Unit> = parser::parse(bufin)?.into();
    let typs = input.iter().map(|u| u.typ).collect::<HashSet<_>>();
    let best = typs
        .iter()
        .map(|typ| {
            let filtered = input
                .iter()
                .filter(|u| u.typ != *typ)
                .cloned()
                .collect::<VecDeque<_>>();
            react(filtered).len()
        })
        .min();
    best.ok_or_else(|| anyhow!("error calculating best"))
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
