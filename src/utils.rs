///----------utils.rs----------///

use crate::{App, InputMode, Task};

/// Sets the app to add a new task, changing the input mode to `Adding` and deselecting any currently selected task.
pub fn new_task(app: &mut App) {
    app.input_mode = InputMode::Adding;
    app.selected_task = None;
}

/// Sets the flag to exit the application.
pub fn exit_app(app: &mut App) {
    app.input_mode = InputMode::ConfirmQuit
}

/// Moves the selection up in the task list, if possible.
pub fn list_up(app: &mut App) {
    if let Some(index) = app.selected_task {
        if index > 0 {
            app.selected_task = Some(index - 1);
        }
    }
}

/// Moves the selection down in the task list, if possible.
pub fn list_down(app: &mut App) {
    if let Some(index) = app.selected_task {
        if index < app.task.len() - 1 {
            app.selected_task = Some(index + 1);
        }
    }
}

/// Toggles the completion status of the currently selected task.
pub fn check_and_uncheck_task(app: &mut App) {
    if let Some(index) = app.selected_task {
        app.task[index].completed = !app.task[index].completed;
    }
}

/// Deletes the currently selected task from the list. Adjusts the selection if needed.
pub fn delete_task(app: &mut App) {
    if let Some(index) = app.selected_task {
        app.task.remove(index);
        if app.task.is_empty() {
            app.selected_task = None;
        } else if app.selected_task.unwrap() == app.task.len() {
            app.selected_task = Some(app.task.len() - 1);
        }
    }
}

/// Saves the current input as a new task by taking the input, creating a new task, and adding it to the list.
pub fn save_task(app: &mut App) {
    let new_task = Task::new(app.input.drain(..).collect());
    app.task.push(new_task);
}

/// Exits the adding mode and sets the input mode back to `Normal`. Selects the first task if there is one.
pub fn exit_adding_mode(app: &mut App) {
    app.input_mode = InputMode::Normal;
    app.selected_task = Some(0);
}

/// Adds a character to the current input buffer, for when the user is typing to add a new task.
pub fn input_add_char(app: &mut App, c: char) {
    app.input.push(c);
}

/// Removes the last character from the current input buffer, for when the user presses backspace while typing.
pub fn input_del_char(app: &mut App) {
    app.input.pop();
}