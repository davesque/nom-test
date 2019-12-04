use nom::bytes::complete::is_not;
use nom::character::complete::multispace0;
use nom::combinator::verify;
use nom::error::VerboseError;
use nom::sequence::terminated;
use nom::IResult;

fn one_token(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    terminated(is_not(" \t\r\n"), multispace0)(input)
}

fn str_token(
    expected_string: String,
) -> impl for<'a> Fn(&'a str) -> IResult<&str, &str, VerboseError<&str>> {
    verify(one_token, move |actual_string| {
        actual_string == expected_string
    })
}

fn main() {
    let parser_1 = str_token("foo".into());
    let string = "foo bar".to_string();
    let input = &string[..];
    let parser_2 = str_token("foo".into());

    println!("{:?} {:?}", parser_1(input), parser_2(input),);
}
