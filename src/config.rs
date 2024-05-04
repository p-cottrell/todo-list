use crossterm::event::KeyCode;
use serde::{Deserialize, Serialize};
use tui::style::Color;

/// Configuration structure for the application, storing color settings and key bindings.
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Config {
    pub colors: Colors,         // Color settings for the UI.
    pub keybindings: KeyBindings, // Key bindings for application actions.
}

/// Defines color settings used throughout the application.
#[derive(Serialize, Deserialize, Debug)]
pub struct Colors {
    pub foreground: Color,       // Default foreground color.
    pub background: Color,       // Default background color.
    pub selection_fg: Color,     // Foreground color for selected items.
    pub selection_bg: Color,     // Background color for selected items.
    pub check_sign: Color,       // Color for check marks in tasks.
    pub welcome_message: Color,       // Color for welcome message.
}

/// Provides default color values, useful for initial configuration or resets.
impl Default for Colors {
    fn default() -> Self {
        Self {
            foreground: Color::Rgb(242, 60, 147),
            background: Color::Rgb(0, 0, 0),
            selection_fg: Color::Rgb(255, 255, 255),
            selection_bg: Color::Rgb(0, 128, 128),
            check_sign: Color::Rgb(217, 200, 25),
            welcome_message: Color::Rgb(242, 60, 147),
        }
    }
}

/// Struct for mapping application actions to keyboard keys.
#[derive(Serialize, Deserialize, Debug)]
pub struct KeyBindings {
    pub exit_app: KeyCode,            // Key to exit the application.
    pub new_task: KeyCode,            // Key to initiate adding a new task.
    pub check_and_uncheck_task: KeyCode, // Key to mark a task as complete or incomplete.
    pub list_up: KeyCode,             // Key to navigate up in the task list.
    pub list_down: KeyCode,           // Key to navigate down in the task list.
    pub delete_task: KeyCode,         // Key to delete a task.
    pub exit_adding: KeyCode,         // Key to exit the task adding mode.
    pub save_task: KeyCode,           // Key to save a new task.
}

/// Provides default key bindings for actions.
impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            exit_app: KeyCode::Esc,
            new_task: KeyCode::Char('n'),
            check_and_uncheck_task: KeyCode::Enter,
            list_up: KeyCode::Up,
            list_down: KeyCode::Down,
            delete_task: KeyCode::Delete,
            exit_adding: KeyCode::Esc,
            save_task: KeyCode::Enter,
        }
    }
}

/// Converts a `KeyCode` into a string.
pub fn keycode_to_string(keycode: KeyCode) -> String {
    let temp;

    let stringified = match keycode {
        KeyCode::Backspace => "Backspace",
        KeyCode::Enter => "Enter",
        KeyCode::Left => "←",
        KeyCode::Right => "→",
        KeyCode::Up => "↑",
        KeyCode::Down => "↓",
        KeyCode::Home => "Home",
        KeyCode::End => "End",
        KeyCode::PageUp => "Page Up",
        KeyCode::PageDown => "Page Down",
        KeyCode::Tab => "Tab",
        KeyCode::BackTab => "Back Tab",
        KeyCode::Delete => "Delete",
        KeyCode::Insert => "Insert",
        KeyCode::F(n) => {
            temp = format!("F{}", n);
            temp.as_str()
        },
        KeyCode::Char(char) => {
            temp = char.to_string();
            temp.as_str()
        },
        KeyCode::Null => "Null",
        KeyCode::Esc => "Esc",
    }
    .to_string();

    stringified
}