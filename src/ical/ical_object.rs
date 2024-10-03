use winnow::ascii::line_ending;
use winnow::prelude::*;
use winnow::token::take_until;

use crate::ical::Input;

// RFC2445 Section 4.4
// icalobject = 1*("BEGIN" ":" "VCALENDAR" CRLF
//                   icalbody
//                   "END" ":" "VCALENDAR" CRLF)

pub struct ICalObject {}

fn ical_object(input: &mut Input) -> PResult<ICalObject> {
    let _ = "BEGIN:VCALENDAR".parse_next(input)?;
    let _ = line_ending.parse_next(input)?;
    let _ = take_until(0.., "END:VCALENDAR").parse_next(input)?;
    let _ = "END:VCALENDAR".parse_next(input)?;
    let _ = line_ending(input)?;
    Ok(ICalObject {})
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ical::new_input;

    #[test]
    fn test_parse_ical() {
        let mut ical_data = new_input("BEGIN:VCALENDAR\r\nVERSION:2.0\r\nPRODID:-//hacksw/handcal//NONSGML v1.0//EN\r\nBEGIN:VEVENT\r\nDTSTART:19970714T170000Z\r\nDTEND:19970715T035959Z\r\nSUMMARY:Bastille Day Party\r\nEND:VEVENT\r\nEND:VCALENDAR\r\n");
        let result = ical_object(&mut ical_data);
        assert!(result.is_ok());
    }
}
