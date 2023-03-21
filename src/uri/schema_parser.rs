use nom::{branch::alt, bytes::streaming::tag_no_case, error::context};
use rujson_parser::Res;

use super::Scheme;

fn parse_scheme(input: &str) -> Res<&str, Scheme> {
    context(
        "scheme",
        alt((tag_no_case("HTTP://"), tag_no_case("HTTPS://"))),
    )(input)
    .map(|(next_input, res)| (next_input, Scheme::from(res)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{
        error::{ErrorKind, VerboseError, VerboseErrorKind},
        Err as NomErr,
    };

    #[test]
    fn test_scheme() {
        assert_eq!(parse_scheme("https://yay"), Ok(("yay", Scheme::HTTPS)));
        assert_eq!(parse_scheme("http://yay"), Ok(("yay", Scheme::HTTP)));
        assert_eq!(
            parse_scheme("bla://yay"),
            Err(NomErr::Error(VerboseError {
                errors: vec![
                    ("bla://yay", VerboseErrorKind::Nom(ErrorKind::Tag)),
                    ("bla://yay", VerboseErrorKind::Nom(ErrorKind::Alt)),
                    ("bla://yay", VerboseErrorKind::Context("scheme")),
                ]
            }))
        );
    }
}
