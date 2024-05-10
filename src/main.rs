///----------main.rs----------///

use crossterm::{
    event::{read, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io::stdout};
use tui::{backend::CrosstermBackend, Terminal};
use todo_list::{
    events::{handle_events, handle_input_cursor},
    file_handler::{load_task, save_task},
    widget, App, CrossTerminal, DynResult, TerminalFrame, config::Config
};

fn main() -> DynResult {
    // Initialize the terminal for UI display, enable raw mode, and switch to alternate screen.
    let mut terminal = initialise_terminal()?;

    // Load task data and configuration from files.
    let task = load_task()?;
    let config = Config::default();
    let mut app = App::new(&task, config);

    // Start the main UI loop for the application.
    draw_ui(&mut terminal, &mut app)?;

    // Cleanup the terminal settings and return to normal terminal after the app closes.
    cleanup_terminal(terminal)?;

    // Save the task data back to the file.
    save_task(&app.task)?;

    Ok(())
}

/// Initialises and returns a terminal object
fn initialise_terminal() -> Result<CrossTerminal, Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    Ok(terminal)
}

/// Cleans up the terminal by disabling raw mode and exiting the alternate screen.
fn cleanup_terminal(mut terminal: CrossTerminal) -> DynResult {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}

/// Manages the main UI loop, rendering the UI and handling input events until the application should exit.
fn draw_ui(terminal: &mut CrossTerminal, app: &mut App) -> DynResult {
    while !app.should_exit {
        // Draw the UI in the terminal using the provided application state.
        terminal.draw(|f| {
            app_view(f, app);
        })?;

        // Read and handle user key events.
        if let Ok(Event::Key(event)) = read() {
            handle_events(event, app);
        }
    }

    Ok(())
}

/// Renders the current view of the application in the terminal frame.
fn app_view(frame: &mut TerminalFrame, app: &App) {
    let main_chunks = widget::main_chunks(frame.size());

    // Render the task input widget in the first section and manage the input cursor.
    let taskinput = widget::task_input(app);
    frame.render_widget(taskinput, main_chunks[0]);
    handle_input_cursor(&app, frame, &main_chunks);


    // Render the task list widget in the second section.
    let tasklist = widget::task_list(app);
    frame.render_widget(tasklist, main_chunks[1]);

    // Render the navigation hint widget in the third section.
    let navigation_hint = widget::navigation_hint(app);
    frame.render_widget(navigation_hint, main_chunks[2]);
}