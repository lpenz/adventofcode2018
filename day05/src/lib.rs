// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#[cfg(test)]
use anyhow::Result;

pub const EXAMPLE: &str = "dabAcCaCBAcCcaDA\n";

#[derive(Debug, PartialEq, Eq)]
pub struct Unit {
    pub typ: char,
    pub pol: bool,
}

impl Unit {
    pub fn new(c: char) -> Unit {
        Unit {
            typ: c.to_ascii_lowercase(),
            pol: c.is_lowercase(),
        }
    }
}

pub mod parser {
    use anyhow::anyhow;
    use anyhow::Result;
    use nom::character::complete as character;
    use nom::combinator;
    use nom::multi;
    use nom::IResult;
    use std::io::BufRead;

    use super::Unit;

    pub fn unit(input: &str) -> IResult<&str, Unit> {
        combinator::map_opt(character::anychar, |c| {
            if c.is_ascii_alphabetic() {
                Some(Unit::new(c))
            } else {
                None
            }
        })(input)
    }

    pub fn line(input: &str) -> IResult<&str, Vec<Unit>> {
        let (input, units) = multi::many1(unit)(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, units))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Unit>> {
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
    let input = parser::parse(EXAMPLE.as_bytes())?;
    let ans = vec![
        Unit::new('d'),
        Unit::new('a'),
        Unit::new('b'),
        Unit::new('A'),
        Unit::new('c'),
        Unit::new('C'),
        Unit::new('a'),
        Unit::new('C'),
        Unit::new('B'),
        Unit::new('A'),
        Unit::new('c'),
        Unit::new('C'),
        Unit::new('c'),
        Unit::new('a'),
        Unit::new('D'),
        Unit::new('A'),
    ];
    assert_eq!(input, ans);
    Ok(())
}
