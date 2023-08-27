// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use color_eyre::{eyre::eyre, Result};

pub const EXAMPLE: &str = "37\n";

pub mod parser {
    use aoc::parser::*;

    // use super::*;

    fn num(input: &str) -> IResult<&str, u64> {
        character::u64(input)
    }

    fn line(input: &str) -> IResult<&str, u64> {
        let (input, num) = num(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, num))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<u64> {
        aoc::parse_with!(line, bufin)
    }
}

#[test]
fn test() -> Result<()> {
    assert_eq!(parser::parse(EXAMPLE.as_bytes())?, 37);
    Ok(())
}
