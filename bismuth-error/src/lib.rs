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
