use crate::ical::Input;
use winnow::combinator::{alt, not, repeat_till};
use winnow::error::{ErrMode, ParserError};
use winnow::prelude::*;
use winnow::stream::{Compare, FindSlice, Stream, StreamIsPartial};
use winnow::token::any;

pub fn line_ending<I>(input: &mut I) -> PResult<<I as Stream>::Slice>
where
    I: StreamIsPartial + Stream + Compare<&'static str>,
{
    let newline = winnow::ascii::line_ending(input)?;

    // Newlines followed by whitespace are considered part of the previous line
    let newline_location = input.checkpoint();
    let _ = not(alt((" ", "\t"))).parse_next(input)?;
    input.reset(&newline_location);

    return Ok(newline);
}

pub fn till_line_ending<I>(input: &mut I) -> PResult<<I as Stream>::Slice>
where
    I: StreamIsPartial + Stream + Compare<&'static str>,
{
    let start = input.checkpoint();
    loop {
        let start_of_candidate = input.checkpoint();
        match line_ending.parse_next(input) {
            Ok(_) => {
                input.reset(&start_of_candidate);
                let offset = input.offset_from(&start);
                input.reset(&start);
                return Ok(input.next_slice(offset));
            }
            Err(ErrMode::Backtrack(_)) => {
                input.reset(&start_of_candidate);
                let _ = any.parse_next(input)?;
            }
            Err(e) => return Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ical::new_input;

    #[test]
    fn test_line_ending() {
        let mut input = new_input("\n");
        assert_eq!(line_ending(&mut input).unwrap(), "\n");

        let mut input = new_input("\r\n");
        assert_eq!(line_ending(&mut input).unwrap(), "\r\n");
    }

    #[test]
    fn test_invalid_line_ending() {
        let mut input = new_input("\r\n ");
        assert!(line_ending(&mut input).is_err());
    }

    #[test]
    fn test_till_line_ending() {
        let mut input = new_input("Hello, world!\n");
        assert_eq!(till_line_ending(&mut input).unwrap(), "Hello, world!");

        let mut input = new_input("Hello, world!\r\n");
        assert_eq!(till_line_ending(&mut input).unwrap(), "Hello, world!");
    }

    #[test]
    fn test_till_line_ending_invalid() {
        let mut input = new_input("Hello, world!\n ");
        assert!(till_line_ending(&mut input).is_err());
    }
}
