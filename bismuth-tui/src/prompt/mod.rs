#![allow(dead_code)]
mod input;
mod select;

pub struct Prompter<T: ToString> {
    title: T,
    description: Option<T>,
}

/// For selecting from a list of options
pub trait Select<T: ToString> {
    fn get_options(&self) -> Vec<Option<T>>;

    /// Selects a option by the index returned by `get_options`
    /// Returns a string that should be displayed
    fn select_option(&self, index: i32) -> Option<T>;

    fn get_prompter(&self) -> Prompter<T>;

    fn run(&self) {}
}

/// For accepting string inputs
pub trait Input<T: ToString> {
    /// Sets the result
    /// Returns a string that should be displayed
    fn set_result(&self, result: T) -> Option<T>;

    fn get_prompter(&self) -> Prompter<T>;

    fn run(&self) {}
}

pub struct Option<T: ToString> {
    promt_value: T,
    promt_description: T,
}

mod utils {}
