use nom::bytes::complete::is_not;
use nom::character::complete::multispace0;
use nom::combinator::verify;
use nom::error::{
    ParseError,
    VerboseError,
};
use nom::sequence::terminated;
use nom::IResult;

fn one_token<'a, E>(input: &'a str) -> IResult<&str, &str, E>
where
    E: ParseError<&'a str>,
{
    terminated(is_not(" \t\r\n"), multispace0)(input)
}

fn str_token<'a, E>(expected_string: String) -> impl Fn(&'a str) -> IResult<&str, &str, E>
where
    E: ParseError<&'a str>,
{
    verify(one_token, move |actual_string| {
        actual_string == expected_string
    })
}

fn main() {
    let parser_1 = str_token::<VerboseError<_>>("foo".into());
    let string = "foo bar".to_string();
    let input = &string[..];
    let parser_2 = str_token::<VerboseError<_>>("foo".into());

    println!("{:?} {:?}", parser_1(input), parser_2(input),);
}
