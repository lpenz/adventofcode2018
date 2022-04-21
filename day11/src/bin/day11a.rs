// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use std::collections::HashMap;
use std::io::{stdin, BufRead};

use rayon::prelude::*;

use day11::*;

fn process(bufin: impl BufRead) -> Result<(usize, usize)> {
    let serial = parser::parse(bufin)?;
    let grid = grid_coords_par_iter()
        .map(|xy| (xy, cell_power_calc(xy, serial)))
        .collect::<HashMap<_, _>>();
    let maxpower = grid_coords_par_iter()
        .filter(|xy| xy.0 <= 298 && xy.1 <= 298)
        .map(|xy| {
            let power = square_coords_iter(xy).map(|xy| grid[&xy]).sum::<i64>();
            (power, xy)
        })
        .max();
    Ok(maxpower.unwrap().1)
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process("18\n".as_bytes())?, (33, 45));
    assert_eq!(process("42\n".as_bytes())?, (21, 61));
    Ok(())
}

fn main() -> Result<()> {
    println!("{:?}", process(stdin().lock())?);
    Ok(())
}
