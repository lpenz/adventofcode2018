// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use std::io::{stdin, BufRead};

use day08::*;

fn node_value(nodes: &[Node], inode: Inode) -> usize {
    let node = &nodes[usize::from(inode)];
    if node.children.is_empty() {
        node.data.iter().sum()
    } else {
        node.data
            .iter()
            .filter_map(|&i| {
                if 0 < i && i <= node.children.len() {
                    Some(node_value(nodes, node.children[i - 1]))
                } else {
                    None
                }
            })
            .sum()
    }
}

fn process(bufin: impl BufRead) -> Result<usize> {
    let allnodes = parser::parse(bufin)?;
    Ok(node_value(&allnodes, Inode::FIRST))
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 66);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
