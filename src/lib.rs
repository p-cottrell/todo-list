///----------lib.rs----------///

pub mod utils;
pub mod config;
pub mod events;
pub mod file_handler;
pub mod widget;

use config::Config;
use serde::{Deserialize, Serialize};
use std::{error::Error, io::Stdout};
use tui::{
    backend::CrosstermBackend,
    style::{Modifier, Style},
    Frame, Terminal,
};

pub type DynResult = Result<(), Box<dyn Error>>;
pub type CrossTerminal = Terminal<CrosstermBackend<Stdout>>;
pub type TerminalFrame<'a> = Frame<'a, CrosstermBackend<Stdout>>;

/// Represents a single task in the task list.
#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub title: String,
    pub completed: bool,
}

impl Task {
    /// Constructor for creating a new task with a given title.
    pub fn new(title: String) -> Self {
        Self {
            title,
            completed: false, // Tasks start as not completed by default.
        }
    }
}

/// Represents the complete list of tasks in the application.
#[derive(Serialize, Deserialize, Default)]
pub struct TaskList {
    pub task: Vec<Task>,
}

impl TaskList {
    /// Constructor for creating a new list of tasks from a slice of Task references.
    pub fn new(task: &[Task]) -> Self {
        Self {
            task: task.to_vec(), // Converts the slice into a Vec for storage.
        }
    }
}

/// Enum representing the possible states of the input field in the application.
pub enum InputMode {
    /// Normal browsing through the task list.
    Normal,
    /// Adding a new task to the list.
    Adding,
    /// To confirm the user want's to quit
    ConfirmQuit,
}

/// Main application state structure holding all data about the application's runtime state.
pub struct App {
    /// Buffer for user input when adding new tasks.
    pub input: String,
    /// Current mode of input handling, either adding a task or browsing.
    pub input_mode: InputMode,
    /// Dynamic list of tasks currently managed by the application.
    pub task: Vec<Task>,
    /// Flag indicating whether the application should exit.
    pub should_exit: bool,
    /// Index of the currently selected task in the list, if any.
    pub selected_task: Option<usize>,
    /// Configuration settings for the application.
    pub config: Config,
}

impl App {
    /// Constructor for creating a new App instance from a list of tasks and configuration settings.
    pub fn new(task: &[Task], config: Config) -> Self {
        Self {
            task: task.to_vec(),
            selected_task: Some(0), // Start with the first task selected.
            input: String::new(),
            input_mode: InputMode::Normal,
            should_exit: false,
            config,
        }
    }

    /// Returns the default style for the UI based on application configuration.
    pub fn default_style(&self) -> Style {
        Style::default()
            .fg(self.config.colors.foreground)
            .bg(self.config.colors.background)
    }

    /// Returns the style for selected items in the UI.
    pub fn selection_style(&self) -> Style {
        self.default_style()
            .fg(self.config.colors.selection_fg)
            .bg(self.config.colors.selection_bg)
    }

    /// Returns the style for the check sign, adjusted based on whether the task is selected.
    pub fn check_sign_style(&self, selected: bool) -> Style {
        if selected {
            self.selection_style().fg(self.config.colors.check_sign)
        } else {
            self.default_style().fg(self.config.colors.check_sign)
        }
    }

    /// Returns the style for completed tasks, applying a crossed-out modifier.
    pub fn checked_task_style(&self, selected: bool) -> Style {
        if selected {
            self.selection_style().add_modifier(Modifier::CROSSED_OUT)
        } else {
            self.default_style().add_modifier(Modifier::CROSSED_OUT)
        }
    }
}