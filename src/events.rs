///----------events.rs----------///


use crate::{utils, App, InputMode, TerminalFrame};
use crossterm::event::{KeyCode, KeyEvent};
use tui::layout::Rect;

/// Input events handler
pub fn handle_events(event: KeyEvent, app: &mut App) {
    match app.input_mode {
        InputMode::Normal => handle_normal_events(app, event.code),
        InputMode::Adding => handle_adding_events(app, event.code),
        InputMode::ConfirmQuit => handle_confirm_quit_events(app, event.code),
    }
}

/// When user is viewing tasks
fn handle_normal_events(app: &mut App, keycode: KeyCode) {
    let keybindings = &app.config.keybindings;

    if keycode == keybindings.new_task {
        utils::new_task(app);
    } else if keycode == keybindings.exit_app {
        utils::exit_app(app);
    } else if keycode == keybindings.list_up {
        utils::list_up(app);
    } else if keycode == keybindings.list_down {
        utils::list_down(app);
    } else if keycode == keybindings.check_and_uncheck_task {
        utils::check_and_uncheck_task(app);
    } else if keycode == keybindings.delete_task {
        utils::delete_task(app);
    }
}

/// When user adding a new task
fn handle_adding_events(app: &mut App, keycode: KeyCode) {
    let keybindings = &app.config.keybindings;

    if keycode == keybindings.save_task && !app.input.trim().is_empty() {
        utils::save_task(app);
    } else if keycode == keybindings.exit_adding {
        utils::exit_adding(app);
    } else if let KeyCode::Char(c) = keycode {
        utils::input_add_char(app, c);
    } else if let KeyCode::Backspace = keycode {
        utils::input_del_char(app);
    }
}

fn handle_confirm_quit_events(app: &mut App, keycode: KeyCode) {
    match keycode {
        KeyCode::Enter => app.should_exit = true,  // Confirm and exit
        KeyCode::Char('n') => {
            app.input_mode = InputMode::Normal;  // Cancel and return to normal mode
        },
        _ => {}  // Ignore other keys in this mode
    }
}

/// Handle cursor when typing
pub fn handle_input_cursor(app: &App, frame: &mut TerminalFrame, chunks: &[Rect]) {
    match app.input_mode {
        InputMode::Normal => {
            // No need to handle cursor in normal mode
        },
        InputMode::Adding => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            frame.set_cursor(
                // Put cursor past the end of the input text
                chunks[0].x + app.input.len() as u16 + 1,
                // Move one line down, from the border to the input line
                chunks[0].y + 1,
            )
        },
        InputMode::ConfirmQuit => {
            // No need to handle cursor in confirm quit mode
        }
    }
}