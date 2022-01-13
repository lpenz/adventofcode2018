// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#[cfg(test)]
use anyhow::Result;

type Claim = (usize, (i32, i32), (i32, i32));

pub const EXAMPLE: &str = "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2
";

pub mod parser {
    use anyhow::anyhow;
    use anyhow::Result;
    use nom::bytes::complete as bytes;
    use nom::character::complete as character;
    use nom::combinator;
    use nom::multi;
    use nom::IResult;
    use std::io::BufRead;

    use super::Claim;

    pub fn line(input: &str) -> IResult<&str, Claim> {
        let (input, _) = bytes::tag("#")(input)?;
        let (input, id) = combinator::map(character::i32, |i| i as usize)(input)?;
        let (input, _) = bytes::tag(" @ ")(input)?;
        let (input, x) = character::i32(input)?;
        let (input, _) = bytes::tag(",")(input)?;
        let (input, y) = character::i32(input)?;
        let (input, _) = bytes::tag(": ")(input)?;
        let (input, w) = character::i32(input)?;
        let (input, _) = bytes::tag("x")(input)?;
        let (input, h) = character::i32(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, (id, (x, y), (w, h))))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Claim>> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        let result = combinator::all_consuming(multi::many1(line))(&input);
        Ok(result
            .map_err(|e| anyhow!("error reading input: {:?}", e))?
            .1)
    }
}

#[test]
fn test() -> Result<()> {
    assert_eq!(
        parser::parse(EXAMPLE.as_bytes())?,
        &[
            (1, (1, 3), (4, 4)),
            (2, (3, 1), (4, 4)),
            (3, (5, 5), (2, 2)),
        ]
    );
    Ok(())
}
