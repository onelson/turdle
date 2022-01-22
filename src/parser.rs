use nom::bytes::complete::take;
use nom::character::complete::alpha1;
use nom::combinator::map_parser;
use nom::sequence::{preceded, terminated};
use nom::{
    branch::alt,
    character::complete::char as char_,
    multi::{count, many1},
    sequence::separated_pair,
    IResult,
};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Column {
    pub fixed: Option<char>,
    pub displaced: Option<Vec<char>>,
}

fn char_parser(i: &str) -> IResult<&str, char> {
    let (i, c) = map_parser(take(1_usize), alpha1)(i)?;
    Ok((i, c.chars().next().expect("char").to_ascii_lowercase()))
}

fn fixed_and_displaced(i: &str) -> IResult<&str, Column> {
    let (i, (c, cs)) = separated_pair(count(char_parser, 1), char_(':'), many1(char_parser))(i)?;
    Ok((
        i,
        Column {
            fixed: Some(c[0]),
            displaced: Some(cs),
        },
    ))
}

fn fixed_only(i: &str) -> IResult<&str, Column> {
    let (i, c) = terminated(count(char_parser, 1), char_(':'))(i)?;
    Ok((
        i,
        Column {
            fixed: Some(c[0]),
            displaced: None,
        },
    ))
}

fn empty(i: &str) -> IResult<&str, Column> {
    let (i, _) = char_(':')(i)?;
    Ok((
        i,
        Column {
            fixed: None,
            displaced: None,
        },
    ))
}

fn displaced_only(i: &str) -> IResult<&str, Column> {
    let (i, cs) = preceded(char_(':'), many1(char_parser))(i)?;
    Ok((
        i,
        Column {
            fixed: None,
            displaced: Some(cs),
        },
    ))
}

fn column_parser(i: &str) -> IResult<&str, Column> {
    alt((fixed_and_displaced, fixed_only, displaced_only, empty))(i)
}

impl FromStr for Column {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (i, col) = column_parser(s).map_err(|e| format!("{}", e))?;
        if !i.is_empty() {
            return Err(format!(
                "expected all input to be consumed, got remainder: {}",
                i
            ));
        }
        return Ok(col);
    }
}

#[cfg(test)]
mod tests;
