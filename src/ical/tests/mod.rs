use std::str::FromStr;

use super::*;

macro_rules! fixture {
    ($filename:expr) => {
        std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/ical/tests/fixtures/",
            $filename
        ))
        .unwrap()
    };
}

// basic.ics
// From RFC2445 Section 4.4: https://www.ietf.org/rfc/rfc2445.txt
#[test]
fn test_parse_basic_ics() {
    ICalStream::from_str(fixture!("basic.ics").as_str()).unwrap();
}
