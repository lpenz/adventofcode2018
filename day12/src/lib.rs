// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::collections::BTreeMap;
use std::collections::BTreeSet;

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

pub type State = BTreeSet<i64>;
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
    use std::collections::BTreeSet;
    use std::io::BufRead;

    use super::Rule;
    use super::State;

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

    pub fn all(input: &str) -> IResult<&str, (State, Vec<Rule>)> {
        let (input, _) = bytes::tag("initial state: ")(input)?;
        let (input, state) = multi::many1(pot)(input)?;
        let (input, _) = character::newline(input)?;
        let (input, _) = character::newline(input)?;
        let (input, rules) = multi::many1(rule)(input)?;
        let state = state
            .into_iter()
            .enumerate()
            .filter_map(|(k, v)| if v { Some(k as i64) } else { None })
            .collect::<BTreeSet<_>>();
        Ok((input, (state, rules)))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<(State, Vec<Rule>)> {
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
        input.0.iter().cloned().collect::<Vec<_>>(),
        vec![0, 3, 5, 8, 9, 16, 17, 18, 22, 23, 24]
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

pub fn nextgen(state0: State, rules: &BTreeMap<Match, bool>) -> State {
    let mut keys = BTreeMap::<i64, Match>::new();
    for i in state0.into_iter() {
        for j in 0..5i64 {
            let key = keys.entry(i + j - 2i64).or_insert([false; 5]);
            key[4 - j as usize] = true;
        }
    }
    keys.into_iter()
        .filter_map(|(i, key)| {
            if rules.get(&key) == Some(&true) {
                Some(i)
            } else {
                None
            }
        })
        .collect::<BTreeSet<i64>>()
}
