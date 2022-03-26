// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

// use anyhow::anyhow;
use anyhow::Result;
use std::collections::HashSet;
use std::io::{stdin, BufRead};
use std::iter;

use day07::*;

fn is_ready(deps: &[(Step, Step)], done: &[Step], step: &Step) -> bool {
    deps.iter()
        .all(|(blocker, dep)| dep != step || done.contains(blocker))
}

fn process(bufin: impl BufRead) -> Result<String> {
    let deps = parser::parse(bufin)?;
    let steps = deps
        .iter()
        .flat_map(|&(s1, s2)| iter::once(s1).chain(iter::once(s2)))
        .collect::<HashSet<_>>();
    let mut done: Vec<Step> = vec![];
    while done.len() < steps.len() {
        let mut ready = steps
            .iter()
            .filter(|s| !done.contains(s) && is_ready(&deps, &done, s))
            .collect::<Vec<_>>();
        ready.sort();
        done.push(*ready[0]);
    }
    Ok(done.into_iter().map(char::from).collect::<String>())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, "CABDFE");
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
