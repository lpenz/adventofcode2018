// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use std::collections::VecDeque;
use std::io::{stdin, BufRead};

use day05::*;

type Polymer = VecDeque<Unit>;

pub fn react(mut polymer: Polymer) -> Polymer {
    if let Some(u1) = polymer.pop_front() {
        let mut polymer2 = react(polymer);
        if let Some(u2) = polymer2.pop_front() {
            if u1.typ != u2.typ || u1.pol == u2.pol {
                polymer2.push_front(u2);
                polymer2.push_front(u1);
            }
            polymer2
        } else {
            VecDeque::from([u1])
        }
    } else {
        polymer
    }
}

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
        Unit::new('d'),
        Unit::new('a'),
        Unit::new('b'),
        Unit::new('C'),
        Unit::new('B'),
        Unit::new('A'),
        Unit::new('c'),
        Unit::new('a'),
        Unit::new('D'),
        Unit::new('A'),
    ]);
    assert_eq!(reacted, ans);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
