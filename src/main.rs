use nom::bytes::complete::is_not;
use nom::character::complete::multispace0;
use nom::combinator::verify;
use nom::error::{
    ParseError,
    VerboseError,
};
use nom::sequence::terminated;
use nom::IResult;

type Inp<'a> = &'a str;

fn one_token<'a, E>(input: Inp<'a>) -> IResult<Inp, Inp, E>
where
    E: ParseError<Inp<'a>>,
{
    terminated(is_not(" \t\r\n"), multispace0)(input)
}

fn str_token<'a, E>(string: String) -> impl Fn(Inp<'a>) -> IResult<Inp, Inp, E>
where
    E: ParseError<Inp<'a>>,
{
    verify(one_token, move |s| s == string)
}

fn main() {
    println!(
        "{:?}",
        str_token::<VerboseError<_>>("foo".into())("foo bar")
    );
}
