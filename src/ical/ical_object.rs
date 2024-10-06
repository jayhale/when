use winnow::ascii::{line_ending, till_line_ending};
use winnow::prelude::*;
use winnow::token::take_until;

use crate::ical::{ICalComponent, Input};

pub struct ICalObject {
    pub version: String,
    pub product_identifier: String,
    pub components: Vec<ICalComponent>,
}

fn ical_object(input: &mut Input) -> PResult<ICalObject> {
    let _ = "BEGIN:VCALENDAR".parse_next(input)?;
    let _ = line_ending.parse_next(input)?;
    let (version, product_identifier) = calprops(input)?;
    let _ = take_until(0.., "END:VCALENDAR").parse_next(input)?;
    let _ = "END:VCALENDAR".parse_next(input)?;
    let _ = line_ending(input)?;
    Ok(ICalObject {
        version,
        product_identifier,
        components: Vec::new(),
    })
}

fn calprops(input: &mut Input) -> PResult<(String, String)> {
    let _ = "VERSION:".parse_next(input)?;
    let version = till_line_ending(input)?;
    let _ = line_ending.parse_next(input)?;
    let _ = "PRODID:".parse_next(input)?;
    let product_identifier = till_line_ending(input)?;
    let _ = line_ending.parse_next(input)?;
    Ok((version.to_string(), product_identifier.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ical::new_input;

    #[test]
    fn test_parse_calprops() {
        let mut ical_data = new_input(
            "VERSION:2.0\r\n\
            PRODID:-//hacksw/handcal//NONSGML v1.0//EN\r\n",
        );
        let result = calprops(&mut ical_data);
        assert!(result.is_ok());
        let (version, product_identifier) = result.unwrap();
        assert_eq!(version, "2.0");
        assert_eq!(product_identifier, "-//hacksw/handcal//NONSGML v1.0//EN");
    }
}
