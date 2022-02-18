// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#[cfg(test)]
use anyhow::Result;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use std::cmp;

#[derive(Debug, PartialEq, Eq)]
pub enum Event {
    Sleep,
    Begin(usize),
    Wakes,
}

impl Default for Event {
    fn default() -> Event {
        Event::Sleep
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Entry {
    pub when: NaiveDateTime,
    pub event: Event,
}

impl Default for Entry {
    fn default() -> Entry {
        Entry {
            when: NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0),
            event: Event::default(),
        }
    }
}

impl cmp::Ord for Entry {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.when.cmp(&other.when)
    }
}

impl cmp::PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Entry {
    pub fn new(dt: NaiveDateTime, ev: Event) -> Entry {
        Entry {
            when: dt,
            event: ev,
        }
    }
}

pub const EXAMPLE: &str = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up
";

pub mod parser {
    use anyhow::anyhow;
    use anyhow::Result;
    use chrono::NaiveDate;
    use nom::branch;
    use nom::bytes::complete as bytes;
    use nom::character::complete as character;
    use nom::combinator;
    use nom::multi;
    use nom::IResult;
    use std::io::BufRead;

    use super::Entry;
    use super::Event;

    pub fn sleep(input: &str) -> IResult<&str, Event> {
        combinator::map(bytes::tag("falls asleep"), |_| Event::Sleep)(input)
    }

    pub fn wakes(input: &str) -> IResult<&str, Event> {
        combinator::map(bytes::tag("wakes up"), |_| Event::Wakes)(input)
    }

    pub fn begin(input: &str) -> IResult<&str, Event> {
        let (input, _) = bytes::tag("Guard #")(input)?;
        let (input, g) = character::u32(input)?;
        let (input, _) = bytes::tag(" begins shift")(input)?;
        Ok((input, Event::Begin(g as usize)))
    }

    pub fn event(input: &str) -> IResult<&str, Event> {
        branch::alt((sleep, branch::alt((wakes, begin))))(input)
    }

    pub fn line(input: &str) -> IResult<&str, Entry> {
        let (input, _) = bytes::tag("[")(input)?;
        let (input, year) = character::i32(input)?;
        let (input, _) = bytes::tag("-")(input)?;
        let (input, month) = character::u32(input)?;
        let (input, _) = bytes::tag("-")(input)?;
        let (input, day) = character::u32(input)?;
        let (input, _) = character::space1(input)?;
        let (input, hour) = character::u32(input)?;
        let (input, _) = bytes::tag(":")(input)?;
        let (input, min) = character::u32(input)?;
        let (input, _) = bytes::tag("]")(input)?;
        let (input, _) = character::space1(input)?;
        let dt = NaiveDate::from_ymd(year, month, day).and_hms(hour, min, 0);
        let (input, ev) = event(input)?;
        let (input, _) = character::newline(input)?;
        let e = Entry::new(dt, ev);
        Ok((input, e))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Entry>> {
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
    let dat = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(dat.len(), 17);
    let d = |d, h, m| NaiveDate::from_ymd(1518, 11, d).and_hms(h, m, 0);
    let ans = vec![
        Entry::new(d(1, 0, 0), Event::Begin(10)),
        Entry::new(d(1, 0, 5), Event::Sleep),
        Entry::new(d(1, 0, 25), Event::Wakes),
        Entry::new(d(1, 0, 30), Event::Sleep),
        Entry::new(d(1, 0, 55), Event::Wakes),
        Entry::new(d(1, 23, 58), Event::Begin(99)),
        Entry::new(d(2, 0, 40), Event::Sleep),
        Entry::new(d(2, 0, 50), Event::Wakes),
        Entry::new(d(3, 0, 5), Event::Begin(10)),
        Entry::new(d(3, 0, 24), Event::Sleep),
        Entry::new(d(3, 0, 29), Event::Wakes),
        Entry::new(d(4, 0, 2), Event::Begin(99)),
        Entry::new(d(4, 0, 36), Event::Sleep),
        Entry::new(d(4, 0, 46), Event::Wakes),
        Entry::new(d(5, 0, 3), Event::Begin(99)),
        Entry::new(d(5, 0, 45), Event::Sleep),
        Entry::new(d(5, 0, 55), Event::Wakes),
    ];
    assert_eq!(dat, ans);
    Ok(())
}
