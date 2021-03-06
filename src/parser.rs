use nom::{
    bytes::complete::take_till,
    character::complete::{digit1, satisfy},
    character::is_alphabetic,
    combinator::{map_res, recognize},
    IResult,
};

pub fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(recognize(digit1), str::parse)(input)
}

pub fn parse_usize_in_range(
    range: std::ops::RangeInclusive<usize>,
) -> Box<dyn Fn(&str) -> IResult<&str, usize>> {
    Box::new(move |input| {
        map_res(parse_usize, |v| {
            if range.contains(&v) {
                Ok(v)
            } else {
                Err("broken")
            }
        })(input)
    })
}

pub fn to_newline(input: &str) -> IResult<&str, &str> {
    take_till(|c| c == '\n')(input)
}

pub fn single_char(input: &str) -> IResult<&str, char> {
    satisfy(|c| is_alphabetic(c as u8))(input)
}
