// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

// use anyhow::anyhow;
use anyhow::Result;
use std::io::{stdin, BufRead};

use day09::*;

fn process(bufin: impl BufRead) -> Result<usize> {
    let _input = parser::parse(bufin)?;
    Ok(1)
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 1);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
