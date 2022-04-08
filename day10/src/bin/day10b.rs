// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

// use anyhow::anyhow;
use anyhow::Result;
use std::io::{stdin, BufRead};

use day10::*;

fn process(bufin: impl BufRead) -> Result<usize> {
    let posvel = parser::parse(bufin)?;
    let (seconds, _word) = get_word(posvel);
    Ok(seconds)
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
