// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#[cfg(test)]
use anyhow::Result;
use std::collections::VecDeque;
use std::fmt;

pub const EXAMPLE: &str = "9 players; last marble is worth 25 points\n";

pub type Marble = usize;
pub type Player = usize;

#[derive(Debug)]
pub struct State {
    pub players: usize,
    pub lastmarble: Marble,
    pub nextplayer: Player,
    pub nextmarble: Marble,
    pub icurrmarble: usize,
    pub turn: Marble,
    pub marbles: VecDeque<Marble>,
    pub scores: Vec<Player>,
}

impl State {
    pub fn new(players: usize, lastmarble: Marble) -> State {
        State {
            players,
            lastmarble,
            nextplayer: 0,
            nextmarble: 0,
            icurrmarble: 0,
            turn: 0,
            marbles: vec![0].into_iter().collect(),
            scores: vec![0].repeat(players),
        }
    }

    pub fn play(&mut self) {
        self.turn += 1;
        if self.turn % 23 == 0 {
            self.scores[self.nextplayer] += self.turn;
            let imarble = if self.icurrmarble >= 7 {
                self.icurrmarble
            } else {
                self.icurrmarble + self.marbles.len()
            } - 7;
            self.scores[self.nextplayer] += self.marbles.remove(imarble).unwrap();
            self.icurrmarble = imarble;
        } else if self.marbles.len() == 1 {
            self.marbles.push_back(self.turn);
            self.icurrmarble = 1;
        } else if self.icurrmarble == self.marbles.len() - 2 {
            self.icurrmarble = self.marbles.len();
            self.marbles.push_back(self.turn);
        } else {
            self.icurrmarble = (self.icurrmarble + 2) % self.marbles.len();
            self.marbles.insert(self.icurrmarble, self.turn);
        }
        if self.turn > 1 {
            self.nextplayer = (self.nextplayer + 1) % self.players;
        }
    }

    pub fn resolve(&mut self) -> &State {
        for _ in 0..=self.lastmarble {
            self.play();
        }
        self
    }

    pub fn max_score(&self) -> usize {
        *self.scores.iter().max().unwrap()
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:3} / {:3} [{}] ",
            self.turn, self.lastmarble, self.nextplayer
        )?;
        for (i, m) in self.marbles.iter().enumerate() {
            if i == self.icurrmarble {
                write!(f, "({}) ", m)?;
            } else {
                write!(f, " {}  ", m)?;
            }
        }
        Ok(())
    }
}

pub mod parser {
    use anyhow::anyhow;
    use anyhow::Result;
    use nom::bytes::complete as bytes;
    use nom::character::complete as character;
    use nom::combinator;
    use nom::IResult;
    use std::io::BufRead;

    use super::Marble;

    pub fn line(input: &str) -> IResult<&str, (usize, Marble)> {
        let (input, players) = character::u32(input)?;
        let (input, _) = bytes::tag(" players; last marble is worth ")(input)?;
        let (input, lastmarble) = character::u32(input)?;
        let (input, _) = bytes::tag(" points")(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, (players as usize, lastmarble as usize)))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<(usize, Marble)> {
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

#[test]
fn test_resolve() -> Result<()> {
    assert_eq!(State::new(5, 25).resolve().max_score(), 32);
    assert_eq!(State::new(10, 1618).resolve().max_score(), 8317);
    assert_eq!(State::new(13, 7999).resolve().max_score(), 146373);
    assert_eq!(State::new(17, 1104).resolve().max_score(), 2764);
    assert_eq!(State::new(21, 6111).resolve().max_score(), 54718);
    assert_eq!(State::new(30, 5807).resolve().max_score(), 37305);
    Ok(())
}
