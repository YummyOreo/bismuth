use crossterm::{cursor, event, execute, style, style::Stylize, terminal};
use std::io::stdout;

use crate::prompt::{utils::read_key, OptionElement, Select};

fn render_info(title: String, description: String) -> Result<(), std::io::Error> {
    execute!(
        stdout(),
        style::Print(format!("\n{title}\n")),
        style::SetForegroundColor(style::Color::Grey),
        style::Print(format!("{description}\n")),
        style::SetForegroundColor(style::Color::White),
    )
}

fn render_option(
    element: &OptionElement,
    return_to_pos: bool,
    selected: bool,
) -> Result<u16, std::io::Error> {
    let mut moves = 1;

    let mut title = format!("    {}", element.promt_value.clone()).white();
    if selected {
        title = title.italic().bold().blue();
    }
    execute!(stdout(), cursor::MoveToColumn(0), style::Print(title))?;
    if let Some(description) = element.promt_description.clone() {
        let mut description = format!("\n    {}", description).grey();
        if selected {
            description = description.italic().blue();
        }
        moves += 1;
        execute!(stdout(), style::Print(description),)?;
    }

    execute!(stdout(), style::Print("\n"))?;

    if return_to_pos {
        execute!(stdout(), cursor::MoveUp(moves), cursor::MoveToColumn(4))?;
    }
    Ok(moves)
}

fn render_options(options: &[OptionElement]) -> Result<(), std::io::Error> {
    execute!(stdout(), cursor::MoveDown(1))?;
    let mut moves = 0;
    for element in options {
        let mut selected = false;
        if moves == 0 {
            selected = true;
        }
        moves += render_option(&element, false, selected)?;
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
    render_option(&options[index as usize], true, false)?;
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
    render_option(&options[next_index as usize], true, true)?;
    return Ok(next_index);
}

fn handle_down(options: &[OptionElement], index: i32) -> Result<i32, std::io::Error> {
    render_option(&options[index as usize], true, false)?;
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
    render_option(&options[next_index as usize], true, true)?;
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
    execute!(stdout(), cursor::Hide, cursor::SavePosition)?;

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
        if let Some(key) = read_key() {
            if let Some(index) = handle_key(key, &options, current_index)? {
                current_index = index;
            } else {
                break;
            }
        }
    }
    let text = if current_index.is_negative() {
        selecter.select_default()
    } else {
        selecter.select_option(current_index)
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
