// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#[cfg(test)]
use anyhow::Result;
use std::collections::VecDeque;

pub const EXAMPLE: &str = "dabAcCaCBAcCcaDA\n";

type Polymer = VecDeque<Unit>;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Unit {
    pub typ: char,
    pub pol: bool,
}

impl From<char> for Unit {
    fn from(c: char) -> Unit {
        Unit {
            typ: c.to_ascii_lowercase(),
            pol: c.is_lowercase(),
        }
    }
}

impl From<(char, bool)> for Unit {
    fn from(cb: (char, bool)) -> Unit {
        Unit {
            typ: cb.0.to_ascii_lowercase(),
            pol: cb.1,
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
                Some(Unit::from(c))
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
        Unit::from('d'),
        Unit::from('a'),
        Unit::from('b'),
        Unit::from('A'),
        Unit::from('c'),
        Unit::from('C'),
        Unit::from('a'),
        Unit::from('C'),
        Unit::from('B'),
        Unit::from('A'),
        Unit::from('c'),
        Unit::from('C'),
        Unit::from('c'),
        Unit::from('a'),
        Unit::from('D'),
        Unit::from('A'),
    ];
    assert_eq!(input, ans);
    Ok(())
}

pub fn react(mut polymer: Polymer) -> Polymer {
    if let Some(u1) = polymer.pop_front() {
        let mut polymer2 = react(polymer);
        if let Some(u2) = polymer2.pop_front() {
            if u1.typ != u2.typ || u1.pol == u2.pol {
                polymer2.push_front(u2);
                polymer2.push_front(u1);
            }
            polymer2
        } else {
            VecDeque::from([u1])
        }
    } else {
        polymer
    }
}
