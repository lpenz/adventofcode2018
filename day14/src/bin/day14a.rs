// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::fmt;

use day14::*;

struct State {
    pub elf1: usize,
    pub elf2: usize,
    pub recipes: Vec<u8>,
}

impl Default for State {
    fn default() -> Self {
        State {
            elf1: 0,
            elf2: 1,
            recipes: vec![3, 7],
        }
    }
}

impl State {
    pub fn process(&mut self) {
        let n1 = self.recipes[self.elf1];
        let n2 = self.recipes[self.elf2];
        let base = n1 + n2;
        if base < 10 {
            self.recipes.push(base);
        } else {
            let d2 = base % 10;
            self.recipes.push((base - d2) / 10);
            self.recipes.push(d2);
        }
        self.elf1 = (self.elf1 + (n1 as usize + 1)) % self.recipes.len();
        self.elf2 = (self.elf2 + (n2 as usize + 1)) % self.recipes.len();
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.recipes.len() {
            if i == self.elf1 {
                write!(f, "({})", self.recipes[i])?;
            } else if i == self.elf2 {
                write!(f, "[{}]", self.recipes[i])?;
            } else {
                write!(f, " {} ", self.recipes[i])?;
            }
        }
        Ok(())
    }
}

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
