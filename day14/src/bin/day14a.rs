// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day14::*;

fn process(ignore: u64) -> Result<u64> {
    let mut st = State::default();
    while (st.recipes.len() as u64) < ignore + 10_u64 {
        st.process();
    }
    let mut num = 0_u64;
    for i in 0..10 {
        num = num * 10 + st.recipes[ignore as usize + i] as u64;
    }
    Ok(num)
}

#[test]
fn test5() -> Result<()> {
    assert_eq!(process(5)?, 124515891);
    Ok(())
}

#[test]
fn test9() -> Result<()> {
    assert_eq!(process(9)?, 5158916779);
    Ok(())
}

#[test]
fn test18() -> Result<()> {
    assert_eq!(process(18)?, 9251071085);
    Ok(())
}

#[test]
fn test2018() -> Result<()> {
    assert_eq!(process(2018)?, 5941429882);
    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;
    println!("{}", process(147061)?);
    Ok(())
}
