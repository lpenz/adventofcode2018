// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::fmt;

#[cfg(test)]
use anyhow::Result;

pub type Sqrid = sqrid::sqrid_create!(13, 6, false);
pub type Qa = sqrid::qa_create!(Sqrid);
pub type Grid = sqrid::grid_create!(Sqrid, Cell);
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
        write!(
            f,
            "{}",
            match self {
                Cell::Empty => ' ',
                Cell::Verti => '|',
                Cell::Horiz => '-',
                Cell::Cross => '+',
                Cell::Raise => '/',
                Cell::Fall => '\\',
            }
        )
    }
}

impl From<char> for Cell {
    fn from(c: char) -> Cell {
        match c {
            ' ' => Cell::Empty,
            '|' => Cell::Verti,
            '-' => Cell::Horiz,
            '+' => Cell::Cross,
            '/' => Cell::Raise,
            '\\' => Cell::Fall,
            '^' => Cell::Verti,
            'v' => Cell::Verti,
            '>' => Cell::Horiz,
            '<' => Cell::Horiz,
            _ => panic!("could not convert char to cell"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Cart {
    pub qa: Qa,
    pub qr: Qr,
    pub lastdir: Qr,
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
        eprintln!("got {:?}", cellscarts);
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
    eprintln!("{}", g);
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
