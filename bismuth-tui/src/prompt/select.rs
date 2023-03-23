use crossterm::{cursor, event, execute, style};
use std::io::stdout;

use crate::prompt::{utils::read_event, OptionElement, Select};

fn render_info(title: String, description: String) -> Result<(), std::io::Error> {
    execute!(
        stdout(),
        style::Print(format!("\n{title}\n")),
        style::SetForegroundColor(style::Color::Grey),
        style::Print(format!("{description}\n")),
        style::SetForegroundColor(style::Color::White),
    )
}

fn render_options(options: &[OptionElement]) -> Result<(), std::io::Error> {
    execute!(stdout(), cursor::MoveDown(1))?;
    let mut moves = 0;
    for element in options {
        moves += 1;
        execute!(
            stdout(),
            style::Print(format!("    {}", element.promt_value.clone()))
        )?;
        if let Some(description) = element.promt_description.clone() {
            moves += 1;
            execute!(
                stdout(),
                style::SetForegroundColor(style::Color::Grey),
                style::Print(format!("\n    {description}")),
                style::SetForegroundColor(style::Color::White),
            )?;
        }
        execute!(stdout(), style::Print("\n"))?;
    }

    execute!(stdout(), cursor::MoveUp(moves), cursor::MoveToColumn(4))
}

fn calc_move(from: usize, to: usize, options: &[OptionElement]) -> i32 {
    let between;
    if from > to {
        between = &options[to..from];
    } else {
        between = &options[from..to];
    }

    let mut lines = 0;
    for element in between {
        lines += 1;
        if element.promt_description.is_some() {
            lines += 1;
        }
    }
    if from < to {
        lines = -lines;
    }
    lines
}

fn handle_up(options: &[OptionElement], index: i32) -> Result<i32, std::io::Error> {
    let next_index;
    if index - 1 >= 0 {
        next_index = index - 1;
    } else {
        next_index = (options.len() as i32) - 1;
    }

    let moves = calc_move(index as usize, next_index as usize, options);
    if moves.is_positive() {
        execute!(stdout(), cursor::MoveUp(moves.try_into().unwrap()))?;
    } else {
        let moves = -moves;
        execute!(stdout(), cursor::MoveDown(moves.try_into().unwrap()))?;
    }
    return Ok(next_index);
}

fn handle_down(options: &[OptionElement], index: i32) -> Result<i32, std::io::Error> {
    let mut next_index = index + 1;
    if next_index >= options.len() as i32 {
        next_index = 0;
    }

    let moves = calc_move(index as usize, next_index as usize, options);
    if moves.is_positive() {
        execute!(stdout(), cursor::MoveUp(moves.try_into().unwrap()))?;
    } else {
        let moves = -moves;
        execute!(stdout(), cursor::MoveDown(moves.try_into().unwrap()))?;
    }
    return Ok(next_index);
}

fn handle_key(
    key: event::KeyEvent,
    options: &[OptionElement],
    index: i32,
) -> Result<Option<i32>, std::io::Error> {
    Ok(match key.code {
        event::KeyCode::Enter => None,
        event::KeyCode::Esc | event::KeyCode::Char('q') => Some(-1),
        event::KeyCode::Up | event::KeyCode::Char('k') => Some(handle_up(options, index)?),
        event::KeyCode::Down | event::KeyCode::Char('j') => Some(handle_down(options, index)?),
        _ => Some(index),
    })
}

pub fn run<T: Select + ?Sized>(selecter: &mut T) -> Result<(), std::io::Error> {
    let prompter = selecter.get_prompter();
    let (title, description) = (prompter.title.clone(), prompter.description.clone());
    render_info(title, description.unwrap_or_default())?;
    let options = selecter.get_options();
    render_options(&options)?;

    let mut current_index = 0_i32;
    loop {
        if current_index.is_negative() {
            break;
        }
        if let Some(event) = read_event() {
            match event {
                event::Event::Key(key) => {
                    if key.kind == event::KeyEventKind::Release {
                        continue;
                    }
                    if let Some(index) = handle_key(key, &options, current_index)? {
                        current_index = index;
                    } else {
                        break;
                    }
                }
                _ => (),
            }
        }
    }
    if current_index.is_negative() {
        selecter.select_default();
    } else {
        selecter.select_option(current_index);
    }

    Ok(())
}
