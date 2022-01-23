use nom::{
    bytes::complete::take,
    character::complete::alpha1,
    character::complete::char as char_,
    combinator::{map_parser, opt},
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

fn column_parser(i: &str) -> IResult<&str, Column> {
    let (i, (c, cs)) = separated_pair(
        opt(count(char_parser, 1)),
        char_(':'),
        opt(many1(char_parser)),
    )(i)?;
    Ok((
        i,
        Column {
            fixed: c.map(|x| x[0]),
            displaced: cs,
        },
    ))
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
        Ok(col)
    }
}

#[cfg(test)]
mod tests;
