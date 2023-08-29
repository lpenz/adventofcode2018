// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::fmt;

pub use color_eyre::{eyre::eyre, Result};

pub struct State {
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
