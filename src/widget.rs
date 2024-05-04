///----------widget.rs----------///

use tui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Modifier,
    text::{Span, Spans, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
};

use crate::{config::keycode_to_string, App, InputMode, Task};

/// Divides the terminal window into main sections to organize the display of various UI components.
pub fn main_chunks(area: Rect) -> Vec<Rect> {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(3), // Fixed size for the input field.
                Constraint::Min(1),    // Minimum size for the task list.
                Constraint::Length(1), // Fixed size for the navigation hint.
            ]
            .as_ref(),
        )
        .split(area);

    chunks
}

/// Creates a list widget displaying all tasks, utilizing `ListItem` for individual tasks.
pub fn task_list(app: &App) -> List {
    // Converts each task in the application to a `ListItem` for rendering.
    let task: Vec<ListItem> = app
        .task
        .iter()
        .enumerate()
        .map(|q| indexed_task_item(app, q))
        .collect();

    // Constructs the list with a styled border and title.
    List::new(task).style(app.default_style()).block(
        Block::default()
            .title("Tasks")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(app.default_style()),
    )
}

/// Determines the visual representation of a task in the list based on its selection and completion status.
fn indexed_task_item<'a>(app: &'a App, (index, task): (usize, &Task)) -> ListItem<'a> {
    if let Some(selected_index) = app.selected_task {
        task_item(
            task.title.clone(),
            task.completed,
            selected_index == index,
            app,
        )
    } else {
        task_item(task.title.clone(), task.completed, false, app)
    }
}

/// Configures the appearance of a single task item in the list, adjusting style for completion and selection.
fn task_item(title: String, completed: bool, selected: bool, app: &App) -> ListItem {
    let style = if selected {
        app.selection_style()
    } else {
        app.default_style()
    };

    let task = if completed {
        ListItem::new(Spans::from(vec![
            Span::styled("✔  ", app.check_sign_style(selected)),
            Span::styled(title, app.checked_task_style(selected)),
        ]))
    } else {
        ListItem::new(Spans::from(vec![
            Span::styled("   ", style),
            Span::styled(title, style),
        ]))
    };

    task.style(style)
}

/// Provides an input field for adding new tasks with visual feedback on the current input mode.
pub fn task_input(app: &App) -> Paragraph {
    let style = match app.input_mode {
        InputMode::Normal => app.default_style(),
        InputMode::Adding => app.default_style().fg(app.config.colors.selection_bg),
        InputMode::ConfirmQuit => app.default_style(),
    };

    let input = Paragraph::new(app.input.as_ref()).style(style).block(
        Block::default()
            .borders(Borders::ALL)
            .title("New task")
            .border_type(BorderType::Rounded)
            .style(style),
    );

    input
}

/// Displays a help section at the bottom of the UI with keyboard shortcuts for various actions, styled differently based on the input mode.
pub fn navigation_hint(app: &App) -> Paragraph {
    let keybindings = &app.config.keybindings;

    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            // Keyboard shortcuts help text for normal mode, styled bold for keys and blinking for the text.
            vec![
                Span::styled(
                    keycode_to_string(keybindings.exit_app),
                    app.default_style().add_modifier(Modifier::BOLD),
                ),
                Span::styled(" exit | ", app.default_style()),
                Span::styled(
                    keycode_to_string(keybindings.new_task),
                    app.default_style().add_modifier(Modifier::BOLD),
                ),
                Span::styled(" new task | ", app.default_style()),
                Span::styled(
                    keycode_to_string(keybindings.check_and_uncheck_task),
                    app.default_style().add_modifier(Modifier::BOLD),
                ),
                Span::styled(" check/uncheck task | ", app.default_style()),
                Span::styled(
                    format!(
                        "{}/{}",
                        keycode_to_string(keybindings.list_up),
                        keycode_to_string(keybindings.list_down)
                    ),
                    app.default_style().add_modifier(Modifier::BOLD),
                ),
                Span::styled(" navigate list | ", app.default_style()),
                Span::styled(
                    keycode_to_string(keybindings.delete_task),
                    app.default_style().add_modifier(Modifier::BOLD),
                ),
                Span::styled(" delete task", app.default_style()),
            ],
            app.default_style(),
        ),
        InputMode::Adding => (
            // Keyboard shortcuts for adding mode
            vec![
                Span::styled(
                    keycode_to_string(keybindings.exit_adding),
                    app.default_style().add_modifier(Modifier::BOLD),
                ),
                Span::styled(" stop adding | ", app.default_style()),
                Span::styled(
                    keycode_to_string(keybindings.save_task),
                    app.default_style().add_modifier(Modifier::BOLD),
                ),
                Span::styled(" save task", app.default_style()),
            ],
            app.default_style(),
        ),

        InputMode::ConfirmQuit => (
            vec![
                Span::styled("Press Enter to confirm quitting, 'n' to cancel", app.default_style())

            ],
            app.default_style()
        ),
    };

    let mut help_text = Text::from(Spans::from(msg));
    help_text.patch_style(style);
    Paragraph::new(help_text).style(app.default_style())
}