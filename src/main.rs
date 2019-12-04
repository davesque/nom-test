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

fn str_token_hrtb<'a, E>(
    expected_string: String,
) -> impl for<'b> Fn(&'b str) -> IResult<&str, &str, E>
where
    E: ParseError<&'a str>,
{
    verify(one_token, move |actual_string| {
        actual_string == expected_string
    })
}

fn main() {
    println!(
        "{:?}",
        str_token::<VerboseError<_>>("foo".into())("foo bar")
    );
}
