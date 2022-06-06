// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::fmt;

use anyhow::anyhow;
use anyhow::Result;

// pub type Sqrid = sqrid::sqrid_create!(14, 8, false);
pub type Sqrid = sqrid::sqrid_create!(151, 151, false);
pub type Qa = sqrid::qa_create!(Sqrid);
pub type Grid = sqrid::grid_create!(Sqrid, Cell);
pub type GridChar = sqrid::grid_create!(Sqrid, char);
pub type Qr = sqrid::Qr;

pub const EXAMPLE: &str = r"/->-\        
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   
";

#[derive(Debug, Clone, Copy, Default)]
pub enum Cell {
    #[default]
    Empty,
    Verti,
    Horiz,
    Cross,
    Raise,
    Fall,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", char::from(self))
    }
}

impl From<&Cell> for char {
    fn from(c: &Cell) -> char {
        match c {
            Cell::Empty => ' ',
            Cell::Verti => '|',
            Cell::Horiz => '-',
            Cell::Cross => '+',
            Cell::Raise => '/',
            Cell::Fall => '\\',
        }
    }
}

impl From<char> for Cell {
    fn from(c: char) -> Cell {
        match c {
            ' ' => Cell::Empty,
            '|' => Cell::Verti,
            '-' => Cell::Horiz,
            '/' => Cell::Raise,
            '\\' => Cell::Fall,
            '+' => Cell::Cross,
            '^' => Cell::Verti,
            'v' => Cell::Verti,
            '>' => Cell::Horiz,
            '<' => Cell::Horiz,
            _ => panic!("could not convert char to cell"),
        }
    }
}

pub fn gridcarts(g: &Grid, carts: &[Cart]) -> GridChar {
    let mut h = g.iter().map(char::from).collect::<GridChar>();
    for c in carts {
        h[c.qa] = match c.qr {
            Qr::N => '^',
            Qr::E => '>',
            Qr::S => 'v',
            Qr::W => '<',
            _ => panic!("invalid qr"),
        };
    }
    h
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Cart {
    pub qa: Qa,
    pub qr: Qr,
    pub lastdir: Qr,
}

impl Cart {
    pub fn eval(&mut self, g: &Grid) -> Result<()> {
        let qr = match g[self.qa] {
            Cell::Verti => {
                if self.qr == Qr::N || self.qr == Qr::S {
                    Ok(self.qr)
                } else {
                    Err(anyhow!(
                        "invalid direction {:?} for position {:?} at {:?}",
                        self.qr,
                        g[self.qa],
                        self.qa
                    ))
                }
            }
            Cell::Horiz => {
                if self.qr == Qr::E || self.qr == Qr::W {
                    Ok(self.qr)
                } else {
                    Err(anyhow!(
                        "invalid direction {:?} for position {:?} at {:?}",
                        self.qr,
                        g[self.qa],
                        self.qa
                    ))
                }
            }
            Cell::Raise => {
                // /
                match self.qr {
                    Qr::N => Ok(Qr::E),
                    Qr::E => Ok(Qr::N),
                    Qr::S => Ok(Qr::W),
                    Qr::W => Ok(Qr::S),
                    _ => panic!("invalid qr"),
                }
            }
            Cell::Fall => {
                // \
                match self.qr {
                    Qr::N => Ok(Qr::W),
                    Qr::E => Ok(Qr::S),
                    Qr::S => Ok(Qr::E),
                    Qr::W => Ok(Qr::N),
                    _ => panic!("invalid qr"),
                }
            }
            Cell::Cross => {
                let dir = match self.lastdir {
                    Qr::W => Qr::N,
                    Qr::N => Qr::E,
                    Qr::E => Qr::W,
                    _ => panic!("invalid lastdir"),
                };
                self.lastdir = dir;
                Ok(self.qr + dir)
            }
            Cell::Empty => panic!("invalid cell"),
        }?;
        self.qa = sqrid::qaqr_resolve(self.qa, qr)?;
        self.qr = qr;
        Ok(())
    }
}

pub fn qr_from_char(c: char) -> Option<Qr> {
    match c {
        '^' => Some(Qr::N),
        'v' => Some(Qr::S),
        '>' => Some(Qr::E),
        '<' => Some(Qr::W),
        _ => None,
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

    use super::qr_from_char;
    use super::Cart;
    use super::Cell;
    use super::Grid;
    use super::Qa;
    use super::Qr;

    pub fn cell(input: &str) -> IResult<&str, (Cell, Option<Qr>)> {
        let (input, c) = character::one_of(" -|+\\/<>v^")(input)?;
        Ok((input, (Cell::from(c), qr_from_char(c))))
    }

    pub fn line(input: &str) -> IResult<&str, (Vec<Cell>, Vec<Option<Qr>>)> {
        let (input, cellscarts) = multi::many1(cell)(input)?;
        let (input, _) = character::newline(input)?;
        let cells = cellscarts.iter().map(|(cell, _)| *cell).collect();
        let carts = cellscarts.into_iter().map(|(_, cart)| cart).collect();
        Ok((input, (cells, carts)))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<(Grid, Vec<Cart>)> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        let (_, cellscarts) = combinator::all_consuming(multi::many1(line))(&input)
            .map_err(|e| anyhow!("error reading input: {:?}", e))?;
        let grid = Qa::iter()
            .map(|qa| {
                let t0 = qa.tuple();
                let t = (t0.0 as usize, t0.1 as usize);
                assert!(cellscarts.len() < Qa::HEIGHT as usize);
                assert!(cellscarts[0].0.len() < Qa::WIDTH as usize);
                if t.1 < cellscarts.len() && t.0 < cellscarts[t.1].0.len() {
                    cellscarts[t.1].0[t.0]
                } else {
                    Cell::Empty
                }
            })
            .collect::<Grid>();
        let carts = Qa::iter()
            .filter_map(|qa| {
                let t0 = qa.tuple();
                let t = (t0.0 as usize, t0.1 as usize);
                if t.1 < cellscarts.len() && t.0 < cellscarts[t.1].1.len() {
                    cellscarts[t.1].1[t.0].map(|qr| Cart {
                        qa,
                        qr,
                        lastdir: Qr::E,
                    })
                } else {
                    None
                }
            })
            .collect::<Vec<Cart>>();
        Ok((grid, carts))
    }
}

#[test]
fn test() -> Result<()> {
    let (g, c) = parser::parse(EXAMPLE.as_bytes())?;
    eprintln!("{}", gridcarts(&g, &c));
    assert_eq!(
        c,
        vec![
            Cart {
                qa: Qa::new_static::<2, 0>(),
                qr: Qr::E,
                lastdir: Qr::E
            },
            Cart {
                qa: Qa::new_static::<9, 3>(),
                qr: Qr::S,
                lastdir: Qr::E
            }
        ]
    );
    Ok(())
}
