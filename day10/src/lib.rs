// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::fmt;
use std::ops;

#[cfg(test)]
use anyhow::Result;

pub const EXAMPLE: &str = "position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>
";

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Pos(i32, i32);

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Vel(i32, i32);

impl ops::Add<Vel> for Pos {
    type Output = Pos;
    fn add(self, other: Vel) -> Self::Output {
        Pos(self.0 + other.0, self.1 + other.1)
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Posvel {
    pv: Vec<(Pos, Vel)>,
}

impl Posvel {
    pub fn step(&mut self) {
        for (p, v) in self.pv.iter_mut() {
            *p = *p + *v;
        }
    }

    pub fn valid(&self) -> bool {
        let left_max_x = self
            .pv
            .iter()
            .filter_map(|(p, v)| if v.0 < 0 { Some(p.0) } else { None })
            .max();
        let right_min_x = self
            .pv
            .iter()
            .filter_map(|(p, v)| if v.0 > 0 { Some(p.0) } else { None })
            .min();
        if let Some(max) = left_max_x {
            if let Some(min) = right_min_x {
                if min < max {
                    return true;
                }
            }
        }
        false
    }

    pub fn contains(&self, p0: &Pos) -> bool {
        self.pv.iter().any(|(p, _)| p == p0)
    }

    pub fn verticality(&self) -> u32 {
        self.pv
            .iter()
            .map(|(p, _)| {
                let down = Pos(p.0, p.1 + 1);
                if self.contains(&down) {
                    1
                } else {
                    0
                }
            })
            .sum()
    }
}

impl From<Vec<(Pos, Vel)>> for Posvel {
    fn from(pv: Vec<(Pos, Vel)>) -> Posvel {
        Posvel { pv }
    }
}

impl fmt::Display for Posvel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let xmin = self.pv.iter().map(|(p, _)| p.0).min().unwrap();
        let xmax = self.pv.iter().map(|(p, _)| p.0).max().unwrap();
        let ymin = self.pv.iter().map(|(p, _)| p.1).min().unwrap();
        let ymax = self.pv.iter().map(|(p, _)| p.1).max().unwrap();
        for y in ymin..=ymax {
            for x in xmin..=xmax {
                let pos = Pos(x, y);
                let lit = self.pv.iter().any(|(p, _)| *p == pos);
                if lit {
                    write!(f, "#")?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn get_word(mut posvel: Posvel) -> (usize, Posvel) {
    let mut i = 0;
    let mut best = (posvel.verticality(), i, posvel.clone());
    while posvel.valid() {
        i += 1;
        posvel.step();
        let v = posvel.verticality();
        if v > best.0 {
            best = (v, i, posvel.clone());
        }
    }
    (best.1, best.2)
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

    use super::{Pos, Posvel, Vel};

    pub fn tuple(input: &str) -> IResult<&str, (i32, i32)> {
        let (input, _) = bytes::tag("<")(input)?;
        let (input, _) = character::space0(input)?;
        let (input, x) = character::i32(input)?;
        let (input, _) = bytes::tag(",")(input)?;
        let (input, _) = character::space0(input)?;
        let (input, y) = character::i32(input)?;
        let (input, _) = bytes::tag(">")(input)?;
        Ok((input, (x, y)))
    }

    pub fn line(input: &str) -> IResult<&str, (Pos, Vel)> {
        let (input, _) = bytes::tag("position=")(input)?;
        let (input, pos) = tuple(input)?;
        let (input, _) = bytes::tag(" velocity=")(input)?;
        let (input, vel) = tuple(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, (Pos(pos.0, pos.1), Vel(vel.0, vel.1))))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Posvel> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        let result = combinator::all_consuming(multi::many0(line))(&input);
        Ok(Posvel::from(
            result
                .map_err(|e| anyhow!("error reading input: {:?}", e))?
                .1,
        ))
    }
}

#[test]
fn test() -> Result<()> {
    assert_eq!(
        parser::parse(EXAMPLE.as_bytes())?,
        Posvel {
            pv: vec![
                (Pos(9, 1), Vel(0, 2)),
                (Pos(7, 0), Vel(-1, 0)),
                (Pos(3, -2), Vel(-1, 1)),
                (Pos(6, 10), Vel(-2, -1)),
                (Pos(2, -4), Vel(2, 2)),
                (Pos(-6, 10), Vel(2, -2)),
                (Pos(1, 8), Vel(1, -1)),
                (Pos(1, 7), Vel(1, 0)),
                (Pos(-3, 11), Vel(1, -2)),
                (Pos(7, 6), Vel(-1, -1)),
                (Pos(-2, 3), Vel(1, 0)),
                (Pos(-4, 3), Vel(2, 0)),
                (Pos(10, -3), Vel(-1, 1)),
                (Pos(5, 11), Vel(1, -2)),
                (Pos(4, 7), Vel(0, -1)),
                (Pos(8, -2), Vel(0, 1)),
                (Pos(15, 0), Vel(-2, 0)),
                (Pos(1, 6), Vel(1, 0)),
                (Pos(8, 9), Vel(0, -1)),
                (Pos(3, 3), Vel(-1, 1)),
                (Pos(0, 5), Vel(0, -1)),
                (Pos(-2, 2), Vel(2, 0)),
                (Pos(5, -2), Vel(1, 2)),
                (Pos(1, 4), Vel(2, 1)),
                (Pos(-2, 7), Vel(2, -2)),
                (Pos(3, 6), Vel(-1, -1)),
                (Pos(5, 0), Vel(1, 0)),
                (Pos(-6, 0), Vel(2, 0)),
                (Pos(5, 9), Vel(1, -2)),
                (Pos(14, 7), Vel(-2, 0)),
                (Pos(-3, 6), Vel(2, -1))
            ]
        }
    );

    Ok(())
}
