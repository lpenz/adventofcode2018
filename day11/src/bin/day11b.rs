// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use std::collections::HashMap;
use std::io::{stdin, BufRead};

use rayon::prelude::*;

use day11::*;

fn process(bufin: impl BufRead) -> Result<(Xy, usize)> {
    let serial = parser::parse(bufin)?;
    let mut oldgrid2 = grid_coords_par_iter(0)
        .map(|xy| ((xy, 0), 0))
        .collect::<HashMap<(Xy, usize), i64>>();
    let mut oldgrid1 = grid_coords_par_iter(1)
        .map(|xy| ((xy, 1), cell_power_calc(xy, serial)))
        .collect::<HashMap<(Xy, usize), i64>>();
    let mut best: ((Xy, usize), i64) = oldgrid1
        .par_iter()
        .max_by_key(|(_, &v)| v)
        .map(|(k, v)| (*k, *v))
        .unwrap();
    for size in 2..300 {
        let newgrid = grid_coords_par_iter(size)
            .filter(|xy| xy.0 <= 300 - size && xy.1 <= 300 - size)
            .map(|xytl| {
                let xytr = (xytl.0 + size - 1, xytl.1);
                let xybl = (xytl.0, xytl.1 + size - 1);
                let xydiag = (xytl.0 + 1, xytl.1 + 1);
                (
                    (xytl, size),
                    oldgrid1.get(&(xytl, size - 1)).unwrap()
                        + oldgrid1.get(&(xydiag, size - 1)).unwrap()
                        + cell_power_calc(xytr, serial)
                        + cell_power_calc(xybl, serial)
                        - oldgrid2.get(&(xydiag, size - 2)).unwrap(),
                )
            })
            .collect::<HashMap<(Xy, usize), i64>>();
        best = newgrid.iter().fold(
            best,
            |best, (k, v)| if best.1 < *v { (*k, *v) } else { best },
        );
        // eprintln!("size {} best {:?}", size, best);
        oldgrid2 = oldgrid1;
        oldgrid1 = newgrid;
    }
    Ok(best.0)
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process("18\n".as_bytes())?, ((90, 269), 16));
    assert_eq!(process("42\n".as_bytes())?, ((232, 251), 12));
    Ok(())
}

fn main() -> Result<()> {
    println!("{:?}", process(stdin().lock())?);
    Ok(())
}
