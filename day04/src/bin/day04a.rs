// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use chrono::Duration;
use chrono::NaiveDateTime;
use chrono::Timelike;
use std::collections::HashMap;
use std::io::{stdin, BufRead};

use day04::*;

fn update(
    totalsleep: &mut HashMap<Guard, i64>,
    mostsleep: &mut HashMap<Guard, HashMap<u32, usize>>,
    guard: Guard,
    since: &mut Option<NaiveDateTime>,
    when: &NaiveDateTime,
) {
    if let Some(s) = since {
        // Update totalsleep
        let dur = (*when - *s).num_minutes();
        let e = totalsleep.entry(guard).or_insert(0);
        *e += dur;
        // Update mostsleep
        let e1 = mostsleep.entry(guard).or_default();
        let min1 = Duration::minutes(1);
        let mut now = *s;
        while now < *when {
            let e2 = e1.entry(now.minute()).or_insert(0);
            *e2 += 1;
            now += min1;
        }
        // Zero out since
        *since = None;
    }
}

fn process(bufin: impl BufRead) -> Result<u32> {
    let mut input = parser::parse(bufin)?;
    input.sort();
    let mut totalsleep = HashMap::new();
    let mut mostsleep = HashMap::new();
    let mut guard = 0;
    let mut since: Option<NaiveDateTime> = None;
    for entry in input.into_iter() {
        match entry.event {
            Event::Begin(iguard) => {
                update(
                    &mut totalsleep,
                    &mut mostsleep,
                    guard,
                    &mut since,
                    &entry.when,
                );
                guard = iguard;
            }
            Event::Wakes => {
                assert!(since.is_some());
                update(
                    &mut totalsleep,
                    &mut mostsleep,
                    guard,
                    &mut since,
                    &entry.when,
                );
            }
            Event::Sleep => {
                assert!(since.is_none());
                since = Some(entry.when);
            }
        }
    }
    let sleeper = totalsleep.into_iter().max_by_key(|(_, s)| *s).unwrap().0;
    let minutes = mostsleep
        .get(&sleeper)
        .unwrap()
        .iter()
        .max_by_key(|(_, freq)| *freq)
        .unwrap()
        .0;
    Ok(sleeper as u32 * minutes)
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 240);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
