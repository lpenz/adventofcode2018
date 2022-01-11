// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::anyhow;
use anyhow::Result;
use std::io::{stdin, BufRead};

use day02::*;

fn process(bufin: impl BufRead) -> Result<String> {
    let codes = parse(bufin)?;
    for (i, code1) in codes.iter().enumerate() {
        for code2 in &codes[i + 1..] {
            let code = code1
                .chars()
                .zip(code2.chars())
                .filter_map(|(c1, c2)| if c1 == c2 { Some(c1) } else { None })
                .collect::<String>();
            if code.len() == code1.len() - 1 {
                return Ok(code);
            }
        }
    }
    Err(anyhow!("no correct code found"))
}

#[test]
fn test() -> Result<()> {
    const EXAMPLE: &str = "abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz
";
    assert_eq!(process(EXAMPLE.as_bytes())?, "fgij");
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
