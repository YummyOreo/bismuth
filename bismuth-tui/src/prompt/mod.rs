#![allow(dead_code)]
mod input;
mod select;

#[derive(Debug)]
pub struct Prompter {
    pub title: String,
    pub description: Option<String>,
}

/// For selecting from a list of options
pub trait Select {
    fn get_options(&self) -> Vec<OptionElement>;

    /// Selects a option by the index returned by `get_options`
    /// Returns a string that should be displayed
    fn select_option(&mut self, index: i32) -> Option<String>;
    /// Selects the default option
    fn select_default(&mut self) -> Option<String>;

    fn get_prompter(&self) -> &Prompter;

    fn run(&mut self) {
        select::run(self).unwrap()
    }
}

/// For accepting string inputs
pub trait Input {
    /// Sets the result
    /// Returns a string that should be displayed
    ///
    /// This indicates that the input has been chosen
    fn set_result(&mut self, result: ResultType) -> Option<String>;
    /// Sets the result to the default result
    /// Returns a string that should be displayed
    ///
    /// This indicates that the user quit
    fn set_default(&mut self) -> Option<String>;

    fn get_prompter(&self) -> &Prompter;
    fn get_result_type(&self) -> &ResultType;

    fn run(&self) {}
}

#[derive(Debug, Clone)]
pub struct OptionElement {
    pub promt_value: String,
    pub promt_description: Option<String>,
}

pub enum ResultType {
    Bool(Option<bool>),
    Other(Option<String>),
}

mod utils {
    #![allow(unused_imports)]
    use crossterm::{event, execute, style};
    use std::{
        io::{stdout, Write},
        time::Duration,
    };

    /// Reads not blocking
    /// If returns `None`. That means that there was no event within 250ms
    pub fn read_key() -> Option<event::KeyEvent> {
        if let Ok(true) = event::poll(Duration::from_millis(250)) {
            return match event::read().expect("Should not fail") {
                event::Event::Key(key) => {
                    if key.kind == event::KeyEventKind::Release {
                        return None;
                    }
                    Some(key)
                }
                _ => None,
            };
        }
        None
    }
}
