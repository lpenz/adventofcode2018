// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use std::collections::HashSet;
use std::io::{stdin, BufRead};
use std::iter;

use andex::*;

use day07::*;

enum IworkerMarker {}
type Iworker = Andex<IworkerMarker, 5>;
type WorkerTime = andex::andex_array!(Iworker, usize);
type WorkerStep = andex::andex_array!(Iworker, Option<Step>);

fn process(bufin: impl BufRead) -> Result<usize> {
    let deps = parser::parse(bufin)?;
    let steps = deps
        .iter()
        .flat_map(|&(s1, s2)| iter::once(s1).chain(iter::once(s2)))
        .collect::<HashSet<_>>();
    let mut done: Vec<Step> = vec![];
    let mut started = HashSet::<Step>::new();
    let mut now = 0;
    let mut wtime = WorkerTime::default();
    let mut wstep = WorkerStep::default();
    while done.len() < steps.len() {
        // Check who finished:
        for iw in Iworker::iter() {
            if wtime[iw] <= now {
                if let Some(step) = wstep[iw].take() {
                    done.push(step);
                    wtime[iw] = 0;
                }
            }
        }
        // New ready steps:
        let mut ready = steps
            .iter()
            .filter(|s| s.got_ready(&deps, &done) && !started.contains(s))
            .collect::<Vec<_>>();
        // Assign workers:
        for iw in Iworker::iter() {
            if wtime[iw] == 0 {
                if let Some(step) = ready.pop() {
                    wtime[iw] = now + step.cost() as usize;
                    wstep[iw] = Some(*step);
                    started.insert(*step);
                }
            }
        }
        // Next relevant time:
        if let Some(now_next) = wtime.iter().filter(|&t| *t > 0).min() {
            now = *now_next;
        }
    }
    Ok(now)
}

#[test]
fn test() -> Result<()> {
    // assert_eq!(process(EXAMPLE.as_bytes())?, 15);
    assert_eq!(process(EXAMPLE.as_bytes())?, 253);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
