// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::io::{stdin, BufRead};

use day12::*;

pub type State = BTreeSet<i32>;

fn nextgen(state0: State, rules: &BTreeMap<Match, bool>) -> State {
    let mut keys = BTreeMap::<i32, Match>::new();
    for i in state0.into_iter() {
        for j in 0..5 {
            let key = keys.entry(i + j - 2).or_insert([false; 5]);
            key[4 - j as usize] = true;
        }
    }
    keys.into_iter()
        .filter_map(|(i, key)| {
            if rules.get(&key) == Some(&true) {
                Some(i)
            } else {
                None
            }
        })
        .collect::<BTreeSet<i32>>()
}

fn process(bufin: impl BufRead) -> Result<i32> {
    let input = parser::parse(bufin)?;
    let (state, rules) = input;
    let mut state = state
        .into_iter()
        .enumerate()
        .filter_map(|(k, v)| if v { Some(k as i32) } else { None })
        .collect::<BTreeSet<_>>();
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
