// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#[cfg(test)]
use anyhow::Result;

pub const EXAMPLE: &str = "9 players; last marble is worth 25 points\n";

pub mod parser {
    use anyhow::anyhow;
    use anyhow::Result;
    use nom::bytes::complete as bytes;
    use nom::character::complete as character;
    use nom::combinator;
    use nom::IResult;
    use std::io::BufRead;

    pub fn line(input: &str) -> IResult<&str, (usize, usize)> {
        let (input, players) = character::u32(input)?;
        let (input, _) = bytes::tag(" players; last marble is worth ")(input)?;
        let (input, lastmarble) = character::u32(input)?;
        let (input, _) = bytes::tag(" points")(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, (players as usize, lastmarble as usize)))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<(usize, usize)> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        let result = combinator::all_consuming(line)(&input);
        Ok(result
            .map_err(|e| anyhow!("error reading input: {:?}", e))?
            .1)
    }
}

#[test]
fn test() -> Result<()> {
    assert_eq!(parser::parse(EXAMPLE.as_bytes())?, (9, 25));
    Ok(())
}
