// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use std::io::BufRead;

pub const EXAMPLE: &str = "+1
-2
+3
+1
";

pub fn parse(bufin: impl BufRead) -> Result<Vec<i32>> {
    bufin
        .lines()
        .map(|line_opt| {
            let line = line_opt?;
            let i: i32 = line.parse()?;
            Ok(i)
        })
        .collect()
}

#[test]
fn test() -> Result<()> {
    assert_eq!(parse(EXAMPLE.as_bytes())?, &[1, -2, 3, 1]);
    Ok(())
}
