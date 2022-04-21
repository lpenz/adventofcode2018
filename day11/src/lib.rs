// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#[cfg(test)]
use anyhow::Result;

use rayon::prelude::*;

pub type Xy = (usize, usize);

pub const EXAMPLE: &str = "4172\n";

pub mod parser {
    use anyhow::anyhow;
    use anyhow::Result;
    use nom::character::complete as character;
    use nom::combinator;
    use nom::IResult;
    use std::io::BufRead;

    pub fn line(input: &str) -> IResult<&str, i64> {
        let (input, num) = character::i64(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, num))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<i64> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        let result = combinator::all_consuming(line)(&input);
        Ok(result
            .map_err(|e| anyhow!("error reading input: {:?}", e))?
            .1)
    }
}

pub fn cell_power_calc(xy: Xy, serial: i64) -> i64 {
    let rackid = xy.0 as i64 + 10;
    let p = (rackid * xy.1 as i64 + serial) * rackid;
    ((p - p % 100) / 100) % 10 - 5
}

pub fn grid_coords_iter() -> impl Iterator<Item = Xy> {
    (1..=300)
        .into_iter()
        .flat_map(|x| (1..=300).into_iter().map(move |y| (x, y)))
}

pub fn square_coords_iter(xy: Xy) -> impl Iterator<Item = Xy> {
    (0..=2)
        .into_iter()
        .flat_map(move |dx| (0..=2).into_iter().map(move |dy| (xy.0 + dx, xy.1 + dy)))
}

pub fn grid_coords_par_iter() -> impl ParallelIterator<Item = Xy> {
    (1..=300)
        .into_par_iter()
        .flat_map(|x| (1..=300).into_par_iter().map(move |y| (x, y)))
}

pub fn square_coords_par_iter(xy: Xy) -> impl ParallelIterator<Item = Xy> {
    (0..=2).into_par_iter().flat_map(move |dx| {
        (0..=2)
            .into_par_iter()
            .map(move |dy| (xy.0 + dx, xy.1 + dy))
    })
}

#[test]
fn test_parse() -> Result<()> {
    assert_eq!(parser::parse(EXAMPLE.as_bytes())?, 4172);
    Ok(())
}

#[test]
fn test_calc() -> Result<()> {
    assert_eq!(cell_power_calc((3, 5), 8), 4);
    assert_eq!(cell_power_calc((122, 79), 57), -5);
    assert_eq!(cell_power_calc((217, 196), 39), 0);
    assert_eq!(cell_power_calc((101, 153), 71), 4);
    Ok(())
}
