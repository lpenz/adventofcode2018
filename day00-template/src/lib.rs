// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use std::io::BufRead;

pub const EXAMPLE: &str = "
";

pub fn parse(bufin: impl BufRead) -> Result<Vec<String>> {
    bufin
        .lines()
        .map(|line_opt| {
            let line = line_opt?;
            Ok(line)
        })
        .collect()
}

#[test]
fn test() -> Result<()> {
    assert_eq!(parse(EXAMPLE.as_bytes())?, &[""]);
    Ok(())
}
