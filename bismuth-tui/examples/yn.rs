use bismuth_tui::prompt::{Input, Prompter, ResultType};

#[derive(Debug)]
struct YesNo {
    result: Option<bool>,
    prompter: Prompter,
}

impl YesNo {
    fn new() -> Self {
        let prompter = Prompter {
            title: String::from("YesNo"),
            description: Some(String::from("Simple Yes No app")),
        };
        Self {
            result: None,
            prompter,
        }
    }
}

impl Input for YesNo {
    fn set_result(&mut self, result: ResultType) -> Option<String> {
        if let ResultType::Bool(b) = result {
            self.result = b
        }
        Some(format!("{self:?}"))
    }
    fn set_default(&mut self) -> Option<String> {
        self.set_result(ResultType::Bool(Some(false)))
    }

    fn get_prompter(&self) -> &Prompter {
        &self.prompter
    }

    fn get_result_type(&self) -> ResultType {
        ResultType::Bool(None)
    }
}

fn main() {
    let mut yn = YesNo::new();
    yn.run();
    std::thread::sleep(std::time::Duration::from_secs(3))
    // println!("{yn:?}");
}
