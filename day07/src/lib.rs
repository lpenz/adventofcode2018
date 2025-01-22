// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#[cfg(test)]
use anyhow::Result;
use std::fmt;
use std::fmt::Write;

pub const EXAMPLE: &str = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.
";

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub struct Step(char);

impl Step {
    pub fn got_ready(&self, deps: &[(Step, Step)], done: &[Step]) -> bool {
        !done.contains(self)
            && deps
                .iter()
                .all(|(blocker, dep)| dep != self || done.contains(blocker))
    }

    pub fn cost(&self) -> u8 {
        char::from(*self) as u8 - b'A' + 61
    }
}

impl From<Step> for char {
    fn from(s: Step) -> char {
        s.0
    }
}

impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

pub mod parser {
    use anyhow::anyhow;
    use anyhow::Result;
    use nom::bytes::complete as bytes;
    use nom::character::complete as character;
    use nom::combinator;
    use nom::multi;
    use nom::IResult;
    use std::io::BufRead;

    use super::Step;

    pub fn step(input: &str) -> IResult<&str, Step> {
        combinator::map(character::one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ"), Step)(input)
    }

    pub fn line(input: &str) -> IResult<&str, (Step, Step)> {
        let (input, _) = bytes::tag("Step ")(input)?;
        let (input, step1) = step(input)?;
        let (input, _) = bytes::tag(" must be finished before step ")(input)?;
        let (input, step2) = step(input)?;
        let (input, _) = bytes::tag(" can begin.")(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, (step1, step2)))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<(Step, Step)>> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        let result = combinator::all_consuming(multi::many1(line))(&input);
        Ok(result
            .map_err(|e| anyhow!("error reading input: {:?}", e))?
            .1)
    }
}

pub fn render(steps: &[Step]) -> String {
    steps.iter().fold(String::new(), |mut s, c| {
        write!(&mut s, "{}", c).unwrap();
        s
    })
}

#[test]
fn test_parse() -> Result<()> {
    assert_eq!(
        parser::parse(EXAMPLE.as_bytes())?,
        &[
            (Step('C'), Step('A')),
            (Step('C'), Step('F')),
            (Step('A'), Step('B')),
            (Step('A'), Step('D')),
            (Step('B'), Step('E')),
            (Step('D'), Step('E')),
            (Step('F'), Step('E'))
        ]
    );
    Ok(())
}

#[test]
fn test_render() -> Result<()> {
    assert_eq!(
        render(&[
            Step('C'),
            Step('C'),
            Step('A'),
            Step('A'),
            Step('B'),
            Step('D'),
            Step('F')
        ]),
        "CCAABDF"
    );
    Ok(())
}
