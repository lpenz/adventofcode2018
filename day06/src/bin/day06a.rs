// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::anyhow;
use anyhow::Result;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{stdin, BufRead};

use day06::*;

fn process(bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    let xmin = input.iter().map(|qa| qa.tuple().0).min().unwrap();
    let xmax = input.iter().map(|qa| qa.tuple().0).max().unwrap();
    let ymin = input.iter().map(|qa| qa.tuple().1).min().unwrap();
    let ymax = input.iter().map(|qa| qa.tuple().1).max().unwrap();
    // BF all initial points at the same time
    let mut qaiters = input
        .iter()
        .map(|qa| (qa, Sqrid::bf_iter(sqrid::qaqr_eval, qa)))
        .collect::<Vec<_>>();
    // Hold points with infinite area, which are skipped later
    let mut center_inf = HashSet::<Qa>::new();
    let mut center_done = HashSet::<Qa>::new();
    let mut nearest = input.iter().map(|&qa| (qa, qa)).collect::<HashMap<_, _>>();
    let mut visited = input.iter().cloned().collect::<HashSet<_>>();
    for _dist in 0..usize::MAX {
        // Create data indexed by coord, with a vec of the centers at
        // dist
        let mut data = HashMap::<Qa, Vec<Qa>>::new();
        for (&center, iter) in &mut qaiters {
            if center_done.contains(&center) {
                continue;
            }
            let mut done = true;
            if let Some(it) = iter.next() {
                for (qa, _) in it {
                    if visited.contains(&qa) {
                        continue;
                    }
                    done = false;
                    let e = data.entry(qa).or_default();
                    e.push(center);
                }
            }
            if done {
                center_done.insert(center);
            }
        }
        if data.is_empty() {
            // If we are done iterating
            break;
        }
        // Use data to update nearest
        for (qa, centers) in data {
            if centers.len() == 1 {
                nearest.insert(qa, centers[0]);
                // If we have touched the border, we are infinite
                let t = qa.tuple();
                if t.0 == xmin || t.0 == xmax || t.1 == ymin || t.1 == ymax {
                    for center in &centers {
                        center_inf.insert(*center);
                    }
                }
            }
            visited.insert(qa);
        }
        // If all pending centers are "infinte", we are done
        if center_done.union(&center_inf).count() == input.len() {
            break;
        }
    }
    let freqs = nearest
        .into_values()
        .fold(HashMap::new(), |mut freqs, center| {
            if !center_inf.contains(&center) {
                *freqs.entry(center).or_insert(0) += 1;
            }
            freqs
        });
    let ans = freqs
        .into_iter()
        .max_by_key(|(_, v)| *v)
        .ok_or_else(|| anyhow!("solution not found"))?;
    Ok(ans.1)
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 17);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
