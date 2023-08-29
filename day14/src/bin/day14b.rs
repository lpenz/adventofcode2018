// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day14::*;

fn has_suffix(suffix: &[u8], st: &State) -> Option<usize> {
    let slen = suffix.len();
    let rlen = st.recipes.len();
    for i in 0..2 {
        if rlen >= slen + i && &st.recipes[(rlen - slen - i)..(rlen - i)] == suffix {
            return Some(rlen - slen - i);
        }
    }
    None
}

fn process(suffix: &[u8]) -> Result<usize> {
    let mut st = State::default();
    let suffix = suffix.iter().map(|c| c - b'0').collect::<Vec<u8>>();
    Ok(loop {
        if let Some(result) = has_suffix(&suffix, &st) {
            break result;
        }
        st.process();
    })
}

#[test]
fn test5() -> Result<()> {
    assert_eq!(process(b"01245")?, 5);
    Ok(())
}

#[test]
fn test9() -> Result<()> {
    assert_eq!(process(b"51589")?, 9);
    Ok(())
}

#[test]
fn test18() -> Result<()> {
    assert_eq!(process(b"92510")?, 18);
    Ok(())
}

#[test]
fn test2018() -> Result<()> {
    assert_eq!(process(b"59414")?, 2018);
    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;
    println!("{}", process(b"147061")?);
    Ok(())
}
