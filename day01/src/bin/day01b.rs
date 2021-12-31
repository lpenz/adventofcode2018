// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::anyhow;
use anyhow::Result;
use std::collections;
use std::io::{stdin, BufRead};

use day01::*;

fn process(bufin: impl BufRead) -> Result<i32> {
    let numbers = parse(bufin)?;
    let mut sum = 0;
    let mut visited = collections::HashSet::new();
    for i in 0..usize::MAX {
        sum += numbers[i % numbers.len()];
        if visited.contains(&sum) {
            return Ok(sum);
        }
        visited.insert(sum);
    }
    Err(anyhow!("no sum visited twice"))
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 2);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
