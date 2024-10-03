use winnow::{BStr, Located};

mod ical_object;
#[cfg(test)]
mod tests;

pub type Input<'b> = Located<&'b BStr>;

pub fn new_input(input: &str) -> Input {
    Located::new(BStr::new(input))
}
