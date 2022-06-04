// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use std::collections::BTreeMap;
use std::io::{stdin, BufRead};

use day12::*;

fn process(bufin: impl BufRead) -> Result<i64> {
    let input = parser::parse(bufin)?;
    let (mut state, rules) = input;
    let rules = rules.into_iter().collect::<BTreeMap<_, _>>();
    for _ in 0..20 {
        state = nextgen(state, &rules);
    }
    Ok(state.into_iter().sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 325);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
