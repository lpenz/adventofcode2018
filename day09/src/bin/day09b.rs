// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

// use anyhow::anyhow;
use anyhow::Result;
use std::io::{stdin, BufRead};

use day09::*;

fn process(bufin: impl BufRead) -> Result<usize> {
    let (players, lastmarble) = parser::parse(bufin)?;
    let mut state = State::new(players, 100 * lastmarble);
    Ok(state.resolve().max_score())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
