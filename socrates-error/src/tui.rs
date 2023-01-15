#![allow(unused)]
use std::io::stdout;

use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal, Result,
};

use crate::State;

pub fn init_options(options: &[String]) -> Result<()> {
    // init cursor
    execute!(stdout(), cursor::Hide, cursor::SavePosition,)?;

    // set the foreground
    execute!(
        stdout(),
        Print("There was a error:\n"),
        SetForegroundColor(Color::Green),
        Print(format!("> {}\n", options.get(0).unwrap())),
        ResetColor,
        // Print("Option 2"),
        // cursor::RestorePosition
    )?;

    for (i, option) in options.iter().enumerate() {
        if i == 0 {
            continue;
        }
        let formated_option = format!("{}\n", option);
        execute!(stdout(), Print(formated_option))?;
    }
    execute!(stdout(), cursor::RestorePosition)?;

    Ok(())
}

pub fn go_up(state: &mut State) {
    if state.max > state.position {
        execute!(
            stdout(),
            ResetColor,
            terminal::Clear(terminal::ClearType::CurrentLine)
        );

        execute!(stdout(), Print(state.get_option(state.position)));
        execute!(stdout(), cursor::MoveToColumn(1), cursor::MoveUp(1));

        execute!(stdout(), terminal::Clear(terminal::ClearType::CurrentLine));

        let option_formated = format!("> {}", state.get_option(state.position - 1));
        execute!(
            stdout(),
            SetForegroundColor(Color::Green),
            Print(option_formated)
        );
        execute!(stdout(), cursor::MoveToColumn(1));
        state.position += 1;
    }
}

pub fn go_down(state: &mut State) {
    if state.max < state.position {
        execute!(
            stdout(),
            ResetColor,
            terminal::Clear(terminal::ClearType::CurrentLine)
        );

        execute!(stdout(), Print(state.get_option(state.position - 2)));
        execute!(stdout(), cursor::MoveToColumn(1), cursor::MoveDown(1));

        execute!(stdout(), terminal::Clear(terminal::ClearType::CurrentLine));

        let option_formated = format!("> {}", state.get_option(state.position - 1));
        execute!(
            stdout(),
            SetForegroundColor(Color::Green),
            Print(option_formated)
        );
        execute!(stdout(), cursor::MoveToColumn(1));
        state.position -= 1;
    }
}

pub enum ReturnType {
    Quit,
    RunOption(usize),
}

pub fn update_options(state: &mut State) -> Option<ReturnType> {
    if let Event::Key(k) = read().expect("Should be able to read") {
        match k.code {
            KeyCode::Char('q') => return Some(ReturnType::Quit),
            KeyCode::Char('k') | KeyCode::Up => go_up(state),
            KeyCode::Char('j') | KeyCode::Down => go_down(state),
            KeyCode::Enter => {
                return Some(ReturnType::RunOption(state.position));
            }
            _ => {}
        }
    }
    None
}
