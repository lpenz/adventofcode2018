// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#[cfg(test)]
use anyhow::Result;

pub type Sqrid = sqrid::sqrid_create!(1000, 1000, false);
pub type Qa = sqrid::qa_create!(Sqrid);

pub const EXAMPLE: &str = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9
";

pub mod parser {
    use anyhow::anyhow;
    use anyhow::Result;
    use nom::bytes::complete as bytes;
    use nom::character::complete as character;
    use nom::combinator;
    use nom::multi;
    use nom::IResult;
    use std::io::BufRead;

    use super::Qa;

    pub fn u16tuple(input: &str) -> IResult<&str, (u16, u16)> {
        let (input, x) = character::u16(input)?;
        let (input, _) = bytes::tag(", ")(input)?;
        let (input, y) = character::u16(input)?;
        Ok((input, (x, y)))
    }

    pub fn qa(input: &str) -> IResult<&str, Qa> {
        combinator::map_res(u16tuple, Qa::try_from)(input)
    }

    pub fn line(input: &str) -> IResult<&str, Qa> {
        let (input, qa) = qa(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, qa))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Qa>> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        let result = combinator::all_consuming(multi::many1(line))(&input);
        Ok(result
            .map_err(|e| anyhow!("error reading input: {:?}", e))?
            .1)
    }
}

#[test]
fn test() -> Result<()> {
    assert_eq!(
        parser::parse(EXAMPLE.as_bytes())?,
        &[
            Qa::new_static::<1, 1>(),
            Qa::new_static::<1, 6>(),
            Qa::new_static::<8, 3>(),
            Qa::new_static::<3, 4>(),
            Qa::new_static::<5, 5>(),
            Qa::new_static::<8, 9>(),
        ]
    );
    Ok(())
}
