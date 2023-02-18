pub mod path;
mod tui;

pub struct State {
    pub position: usize,
    pub max: usize,
    pub options: Vec<String>,
}

impl State {
    pub fn new(options: Vec<String>) -> Self {
        State {
            position: options.len(),
            max: options.len(),
            options,
        }
    }

    pub fn get_option(&self, index: usize) -> String {
        self.options
            .get(index)
            .unwrap_or(&"".to_string())
            .to_string()
    }

    pub fn handle_option(&self, _index: usize) {}
}

pub fn error_ui(options: &[String], description: &str) -> Option<usize> {
    let mut state = State::new(options.to_vec());

    tui::init_options(&state.options, description).unwrap();

    loop {
        if let Some(s) = tui::update_options(&mut state) {
            return match s {
                tui::ReturnType::Quit => None,
                tui::ReturnType::RunOption(u) => Some(u),
            };
        }
    }
}

mod test {
    #![allow(unused, dead_code)]
    use std::error::Error;
    // ME TESTING SHIT

    pub trait Recover<T> {
        fn try_recover(self) -> T;
    }

    pub trait Recoverable<T> {
        fn get_recoverd(&self) -> T;
    }

    pub enum TestError<T> {
        Recoverable(Box<dyn Recoverable<T>>),
        Unrecoverable(Box<dyn Error>),
    }

    impl<T> TestError<T> {
        pub fn recover<E: Recoverable<T> + 'static>(error: E) -> Self {
            Self::Recoverable(Box::new(error))
        }
    }

    impl<T> Recover<T> for TestError<T> {
        fn try_recover(self) -> T {
            match self {
                Self::Recoverable(e) => e.get_recoverd(),
                Self::Unrecoverable(e) => panic!("{e}"),
            }
        }
    }

    pub struct TestError2 {}

    type Result<T, E = TestError<T>> = core::result::Result<T, E>;

    impl<T> Recover<T> for Result<T, TestError<T>> {
        fn try_recover(self) -> T {
            match self {
                Ok(t) => t,
                Err(e) => e.try_recover(),
            }
        }
    }

    impl Recoverable<bool> for TestError2 {
        fn get_recoverd(&self) -> bool {
            true
        }
    }

    pub fn test_2() {
        let t = test().unwrap_err();
        let t = match t {
            TestError::Recoverable(e) => e.get_recoverd(),
            TestError::Unrecoverable(e) => panic!("{e}"),
        };
    }

    pub fn test() -> Result<bool> {
        Err(TestError::recover(TestError2 {}))
    }
}
