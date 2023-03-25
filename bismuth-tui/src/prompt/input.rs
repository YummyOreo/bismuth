use crossterm::{cursor, event, execute, style, style::Stylize, terminal};
use std::io::stdout;

use crate::prompt::{utils::read_key, Input, ResultType};

fn render_info(title: String, description: String) -> Result<(), std::io::Error> {
    execute!(
        stdout(),
        cursor::MoveDown(1),
        style::SetForegroundColor(style::Color::Grey),
        style::Print(format!("{description}")),
        cursor::MoveUp(1),
        cursor::MoveToColumn(0),
        style::SetForegroundColor(style::Color::White),
        style::Print(format!("{title}: ")),
    )
}

fn handle_update(
    input: &str,
    c: Option<&char>,
    kind: &ResultType,
    delete: bool,
) -> Result<String, std::io::Error> {
    if input.len() != 0 || delete {
        let mut to_move = input.len() as u16;
        if delete {
            to_move += 1;
        }
        execute!(
            stdout(),
            cursor::MoveLeft(to_move),
            terminal::Clear(terminal::ClearType::UntilNewLine)
        )?;
    }

    let mut input = input.to_string();
    // println!("{input:?}");
    if let Some(c) = c {
        input.push(*c);
    }

    match kind {
        ResultType::Other(_) => {
            execute!(stdout(), style::Print(input.clone()))?;
            Ok(input)
        }
        ResultType::Bool(_) => {
            let print_input = match input.to_lowercase().as_str() {
                "yes" | "y" => input.clone().green(),
                "no" | "n" => input.clone().red(),
                _ => input.clone().white(),
            };
            execute!(stdout(), style::Print(print_input))?;
            Ok(input)
        }
    }
}

fn handle_key(
    key: event::KeyEvent,
    input: &str,
    kind: &ResultType,
) -> Result<(Option<String>, bool), std::io::Error> {
    Ok(match key.code {
        event::KeyCode::Enter => (None, false),
        event::KeyCode::Esc => (None, true),
        event::KeyCode::Char(c) => (Some(handle_update(input, Some(&c), kind, false)?), false),
        event::KeyCode::Backspace => {
            let mut input = input.to_string();
            if input.len() == 0 {
                (Some(input), false)
            } else {
                input.pop();
                (Some(handle_update(&input, None, kind, true)?), false)
            }
        }
        _ => (Some(input.to_string()), false),
    })
}

pub fn run<T: Input + ?Sized>(inputer: &mut T) -> Result<(), std::io::Error> {
    execute!(stdout(), cursor::SavePosition)?;

    let prompter = inputer.get_prompter();
    let (title, description) = (prompter.title.clone(), prompter.description.clone());
    render_info(title, description.unwrap_or_default())?;

    let kind = inputer.get_result_type();
    let mut select_default = false;
    let mut input = String::new();

    loop {
        if let Some(key) = read_key() {
            let (new_input, default) = handle_key(key, &input, &kind)?;
            if let Some(s) = new_input {
                if s == input {
                    continue;
                }
                input = s;
            } else {
                if default {
                    select_default = default;
                }
                break;
            }
        }
    }
    let res = match kind {
        ResultType::Bool(_) => ResultType::Bool(Some(match input.to_lowercase().as_str() {
            "yes" | "y" => true,
            _ => false,
        })),
        ResultType::Other(_) => ResultType::Other(Some(input)),
    };

    let text = if select_default {
        inputer.set_default()
    } else {
        inputer.set_result(res)
    };

    execute!(
        stdout(),
        cursor::RestorePosition,
        terminal::Clear(terminal::ClearType::FromCursorDown),
        style::Print(text.unwrap_or_default())
    )?;

    execute!(stdout(), cursor::Show)?;
    Ok(())
}
