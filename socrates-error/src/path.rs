#![allow(unused)]
use std::{io::stdout, path::PathBuf};

use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal, Result,
};

use crate::error_ui;

fn update_input(path: &mut String) -> Option<()> {
    if let Event::Key(k) = read().expect("Should be able to read") {
        match k.code {
            KeyCode::Char(x) => {
                path.push(x);
                execute!(stdout(), Print(x.to_string()));
            }
            KeyCode::Backspace => {
                path.pop();
                let print_path = format!("Enter new path: {}", path);
                execute!(
                    stdout(),
                    terminal::Clear(terminal::ClearType::CurrentLine),
                    cursor::MoveToColumn(0),
                    Print(print_path),
                );
            }
            KeyCode::Enter => return Some(()),
            _ => {}
        }
    }
    None
}

pub fn md_file_error(_description: &str) -> Option<PathBuf> {
    let options = vec![
        // "Make folder at path.".to_string(),
        "Chose another path".to_string(),
    ];

    let option = error_ui(&options)?;

    match option {
        // 1 => {}
        1 => {
            execute!(
                stdout(),
                cursor::MoveUp(u16::try_from(options.len()).unwrap() + 1_u16),
                terminal::Clear(terminal::ClearType::CurrentLine),
                cursor::SavePosition
            );
            for i in 0..options.len() + 1 {
                execute!(
                    stdout(),
                    cursor::MoveDown(1),
                    terminal::Clear(terminal::ClearType::CurrentLine),
                    cursor::RestorePosition,
                );
            }

            execute!(
                stdout(),
                ResetColor,
                Print("Enter new path: "),
                cursor::Show
            );
            let mut path = "".to_string();
            loop {
                if update_input(&mut path).is_some() {
                    break;
                }
            }
            return Some(PathBuf::from(path));
        }
        _ => panic!("Something has gone terribly wrong"),
    }
    None
}
