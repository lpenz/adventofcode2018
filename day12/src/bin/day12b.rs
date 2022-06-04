// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use std::collections::BTreeMap;
use std::io::{stdin, BufRead};

use day12::*;

const GENERATIONS: i64 = 50000000000i64;

fn process(bufin: impl BufRead) -> Result<i64> {
    let input = parser::parse(bufin)?;
    let (mut state, rules) = input;
    let rules = rules.into_iter().collect::<BTreeMap<_, _>>();
    let offset = state.iter().min().cloned().unwrap();
    state = state.into_iter().map(|k| k - offset).collect();
    let mut offsetaccum = offset;
    for i in 0..GENERATIONS {
        let last = state.clone();
        state = nextgen(state, &rules);
        let offset = state.iter().min().cloned().unwrap();
        offsetaccum += offset;
        state = state.into_iter().map(|k| k - offset).collect();
        if state == last {
            let len = state.len() as i64;
            let sum = state.iter().sum::<i64>()
                + offsetaccum * state.len() as i64
                + (offset * len * (GENERATIONS - i - 1));
            return Ok(sum);
        }
    }
    Ok(state.into_iter().map(i64::from).sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 999999999374i64);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
