// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#[cfg(test)]
use anyhow::Result;

use andex::*;

pub enum InodeMarker {}
pub type Inode = Andex<InodeMarker, 0xFFFFFFFF>;

#[derive(Debug, Default)]
pub struct Node {
    pub id: Inode,
    pub children: Vec<Inode>,
    pub data: Vec<usize>,
}

pub const EXAMPLE: &str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2\n";
//                         A----------------------------------
//                             B----------- C-----------
//                                              D-----
//                         AAA BBB BB BB BB CCC DDD DD C A A A
//                         C-D C-D DD DD DD C-D C-D DD D D D D

pub mod parser {
    use anyhow::anyhow;
    use anyhow::Result;
    use nom::character::complete as character;
    use nom::combinator;
    use nom::IResult;
    use std::io::BufRead;

    use super::{Inode, Node};

    pub fn num(input: &str) -> IResult<&str, usize> {
        let (input, num) = character::u32(input)?;
        Ok((input, num as usize))
    }

    pub fn header(input: &str) -> IResult<&str, (usize, usize)> {
        let (input, nchildren) = num(input)?;
        let (input, _) = character::char(' ')(input)?;
        let (input, ndata) = num(input)?;
        Ok((input, (nchildren, ndata)))
    }

    pub fn node<'a, 'b>(allnodes: &'a mut Vec<Node>, input: &'b str) -> IResult<&'b str, Inode> {
        let (mut input, (nchildren, ndata)) = header(input)?;
        let inode = Inode::try_from(allnodes.len()).unwrap();
        allnodes.push(Node {
            id: inode,
            ..Default::default()
        });
        for _ in 0..nchildren {
            let (input2, _) = character::char(' ')(input)?;
            let (input2, childid) = node(allnodes, input2)?;
            allnodes[usize::from(inode)].children.push(childid);
            input = input2;
        }
        for _ in 0..ndata {
            let (input2, _) = character::char(' ')(input)?;
            let (input2, datum) = num(input2)?;
            allnodes[usize::from(inode)].data.push(datum);
            input = input2;
        }
        Ok((input, inode))
    }

    pub fn all(input: &str) -> IResult<&str, Vec<Node>> {
        let mut allnodes = vec![];
        let (input, _) = node(&mut allnodes, input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, allnodes))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Node>> {
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
    let allnodes = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(allnodes[0].id, Inode::FIRST);
    Ok(())
}
