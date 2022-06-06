// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use std::collections::BTreeSet;
use std::io::{stdin, BufRead};

use day13::*;

pub const EXAMPLE: &str = r"/->-\        
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   
";

fn process(bufin: impl BufRead) -> Result<(u16, u16)> {
    let (g, mut carts) = parser::parse(bufin)?;
    loop {
        carts.sort_by_key(|c| c.qa.tuple());
        for c in &mut carts {
            c.eval(&g)?;
        }
        let mut seen = BTreeSet::new();
        for c in &carts {
            if seen.contains(&c.qa) {
                return Ok(c.qa.tuple());
            }
            seen.insert(c.qa);
        }
    }
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, (7, 3));
    Ok(())
}

fn main() -> Result<()> {
    println!("{:?}", process(stdin().lock())?);
    Ok(())
}
