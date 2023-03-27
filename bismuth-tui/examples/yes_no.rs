use bismuth_tui::prompt::{OptionElement, Prompter, Select};

#[derive(Debug)]
struct YesNo {
    result: Option<i32>,
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

impl Select for YesNo {
    fn get_options(&self) -> Vec<OptionElement> {
        let yes = OptionElement {
            promt_value: String::from("Yes"),
            promt_description: None,
        };
        let no = OptionElement {
            promt_value: String::from("No"),
            promt_description: Some(String::from("Nope")),
        };
        let idk = OptionElement {
            promt_value: String::from("IDK"),
            promt_description: Some(String::from("You don't know")),
        };
        vec![yes, no, idk]
    }
    fn get_prompter(&self) -> &Prompter {
        &self.prompter
    }

    fn select_option(&mut self, index: i32) -> Option<String> {
        self.result = Some(index);
        Some(format!("{self:?}"))
    }
    fn select_default(&mut self) -> Option<String> {
        self.select_option(0)
    }
}

fn main() {
    let mut yn = YesNo::new();
    yn.run();
    std::thread::sleep(std::time::Duration::from_secs(3))
    // println!("{yn:?}");
}
