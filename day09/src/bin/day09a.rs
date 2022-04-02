// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use std::io::{stdin, BufRead};

use day09::*;

fn process(bufin: impl BufRead) -> Result<usize> {
    let (players, lastmarble) = parser::parse(bufin)?;
    let mut state = State::new(players, lastmarble);
    Ok(state.resolve().max_score())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 32);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
