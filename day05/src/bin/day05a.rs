// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use std::collections::VecDeque;
use std::io::{stdin, BufRead};

use day05::*;

fn process(bufin: impl BufRead) -> Result<usize> {
    let input: VecDeque<Unit> = parser::parse(bufin)?.into();
    let polymer = react(input);
    Ok(polymer.len())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 10);
    Ok(())
}

#[test]
fn test_react() -> Result<()> {
    let reacted = react(VecDeque::from(parser::parse(EXAMPLE.as_bytes())?));
    let ans = VecDeque::from([
        Unit::from('d'),
        Unit::from('a'),
        Unit::from('b'),
        Unit::from('C'),
        Unit::from('B'),
        Unit::from('A'),
        Unit::from('c'),
        Unit::from('a'),
        Unit::from('D'),
        Unit::from('A'),
    ]);
    assert_eq!(reacted, ans);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
