use winnow::combinator::{alt, repeat, repeat_till, separated};
use winnow::prelude::*;
use winnow::token::{any, take_until};

use crate::ical::tokens::{line_ending, till_line_ending};
use crate::ical::{ical_object, new_input, ICalComponent, ICalProperty, Input};

pub struct ICalObject {
    pub properties: Vec<ICalProperty>,
    pub components: Vec<ICalComponent>,
}

pub struct ICalStream(pub Vec<ICalObject>);

impl std::str::FromStr for ICalStream {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let objects = repeat(1.., ical_object::ical_object)
            .parse(new_input(s))
            .map_err(|e| anyhow::format_err!("{e}"))?;
        Ok(ICalStream(objects))
    }
}

pub fn ical_stream(input: &mut Input) -> PResult<ICalStream> {
    let objects = separated(1.., ical_object, line_ending).parse_next(input)?;
    Ok(ICalStream(objects))
}

pub fn ical_object(input: &mut Input) -> PResult<ICalObject> {
    let _ = "BEGIN:VCALENDAR".parse_next(input)?;
    let _ = line_ending(input)?;
    let properties = repeat(1.., calprop).parse_next(input)?;
    let _ = take_until(0.., "END:VCALENDAR").parse_next(input)?;
    let _ = "END:VCALENDAR".parse_next(input)?;
    Ok(ICalObject {
        properties,
        components: Vec::new(),
    })
}

fn calprop(input: &mut Input) -> PResult<ICalProperty> {
    let name = alt((
        "VERSION",
        "PRODID",
        "CALSCALE",
        "METHOD",
        // Non-standard properties
        ("X-", take_until(1.., ":")).take(),
    ))
    .parse_next(input)?;
    let _ = ":".parse_next(input)?;
    let value = till_line_ending(input)?;
    let _ = line_ending(input)?;
    return Ok(ICalProperty {
        name: name.to_string(),
        value: value.to_string(),
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ical::new_input;

    #[test]
    fn test_parse_calprop_version() {
        let mut prop_line = new_input("VERSION:2.0\r\n");
        let result = calprop(&mut prop_line);
        assert!(result.is_ok());
        let prop = result.unwrap();
        assert_eq!(prop.name, "VERSION");
        assert_eq!(prop.value, "2.0");
    }

    #[test]
    fn test_parse_calprop_prodid() {
        let mut prop_line = new_input("PRODID:-//hacksw/handcal//NONSGML v1.0//EN\r\n");
        let result = calprop(&mut prop_line);
        assert!(result.is_ok());
        let prop = result.unwrap();
        assert_eq!(prop.name, "PRODID");
        assert_eq!(prop.value, "-//hacksw/handcal//NONSGML v1.0//EN");
    }

    #[test]
    fn test_parse_calprop_calscale() {
        let mut prop_line = new_input("CALSCALE:GREGORIAN\r\n");
        let result = calprop(&mut prop_line);
        assert!(result.is_ok());
        let prop = result.unwrap();
        assert_eq!(prop.name, "CALSCALE");
        assert_eq!(prop.value, "GREGORIAN");
    }

    #[test]
    fn test_parse_calprop_method() {
        let mut prop_line = new_input("METHOD:PUBLISH\r\n");
        let result = calprop(&mut prop_line);
        assert!(result.is_ok());
        let prop = result.unwrap();
        assert_eq!(prop.name, "METHOD");
        assert_eq!(prop.value, "PUBLISH");
    }

    #[test]
    fn test_parse_calprop_xprop() {
        let mut prop_line = new_input(
            "X-ABC-MMSUBJ;VALUE=URI;FMTTYPE=audio/basic:http://www.example.org/mysubj.au\r\n",
        );
        let result = calprop(&mut prop_line);
        assert!(result.is_ok());
        let prop = result.unwrap();
        assert_eq!(prop.name, "X-ABC-MMSUBJ;VALUE=URI;FMTTYPE=audio/basic");
        assert_eq!(prop.value, "http://www.example.org/mysubj.au");
    }
}
