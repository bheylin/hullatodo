extern crate nom;

use nom::{
    IResult,
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
    sequence::tuple
};

fn parse(_text: &str) -> Vec<Result<Todo, ParseError>> {
    vec![]
}