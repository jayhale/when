use winnow::Located;

mod ical_object;
#[cfg(test)]
mod tests;

pub type Input<'i> = Located<&'i str>;

pub fn new_input(input: &str) -> Input {
    Located::new(input)
}

pub use ical_object::ICalObject;
pub struct ICalComponent {}
