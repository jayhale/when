use winnow::Located;
use winnow::Parser;

mod ical_object;
#[cfg(test)]
mod tests;
mod tokens;

pub type Input<'i> = Located<&'i str>;

pub fn new_input(input: &str) -> Input {
    Located::new(input)
}

pub use ical_object::{ICalObject, ICalStream};
pub struct ICalComponent {}
pub struct ICalProperty {
    pub name: String,
    pub value: String,
}
