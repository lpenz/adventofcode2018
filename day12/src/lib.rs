// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#[cfg(test)]
use anyhow::Result;

pub const EXAMPLE: &str = "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #
";

pub type Match = [bool; 5];
pub type Rule = (Match, bool);

pub mod parser {
    use anyhow::anyhow;
    use anyhow::Result;
    use nom::bytes::complete as bytes;
    use nom::character::complete as character;
    use nom::combinator;
    use nom::multi;
    use nom::IResult;
    use std::io::BufRead;

    use super::Rule;

    pub fn pot(input: &str) -> IResult<&str, bool> {
        let (input, c) = character::one_of(".#")(input)?;
        Ok((input, c == '#'))
    }

    pub fn rule(input: &str) -> IResult<&str, Rule> {
        let (input, c0) = pot(input)?;
        let (input, c1) = pot(input)?;
        let (input, c2) = pot(input)?;
        let (input, c3) = pot(input)?;
        let (input, c4) = pot(input)?;
        let (input, _) = bytes::tag(" => ")(input)?;
        let (input, result) = pot(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, ([c0, c1, c2, c3, c4], result)))
    }

    pub fn all(input: &str) -> IResult<&str, (Vec<bool>, Vec<Rule>)> {
        let (input, _) = bytes::tag("initial state: ")(input)?;
        let (input, state0) = multi::many1(pot)(input)?;
        let (input, _) = character::newline(input)?;
        let (input, _) = character::newline(input)?;
        let (input, rules) = multi::many1(rule)(input)?;
        Ok((input, (state0, rules)))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<(Vec<bool>, Vec<Rule>)> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        let result = combinator::all_consuming(all)(&input);
        Ok(result
            .map_err(|e| anyhow!("error reading input: {:?}", e))?
            .1)
    }
}

#[test]
fn test() -> Result<()> {
    let input = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(
        input.0,
        vec![
            true, false, false, true, false, true, false, false, true, true, false, false, false,
            false, false, false, true, true, true, false, false, false, true, true, true,
        ]
    );
    assert_eq!(input.1[0], ([false, false, false, true, true], true));
    assert_eq!(input.1[1], ([false, false, true, false, false], true));
    assert_eq!(input.1[2], ([false, true, false, false, false], true));
    assert_eq!(input.1[3], ([false, true, false, true, false], true));
    assert_eq!(input.1[4], ([false, true, false, true, true], true));
    assert_eq!(input.1[5], ([false, true, true, false, false], true));
    assert_eq!(input.1[6], ([false, true, true, true, true], true));
    assert_eq!(input.1[7], ([true, false, true, false, true], true));
    assert_eq!(input.1[8], ([true, false, true, true, true], true));
    assert_eq!(input.1[9], ([true, true, false, true, false], true));
    assert_eq!(input.1[10], ([true, true, false, true, true], true));
    assert_eq!(input.1[11], ([true, true, true, false, false], true));
    assert_eq!(input.1[12], ([true, true, true, false, true], true));
    assert_eq!(input.1[13], ([true, true, true, true, false], true));
    Ok(())
}
