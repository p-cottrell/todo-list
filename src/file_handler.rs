///----------file-handlers.rs----------///

use crate::{DynResult, Task, TaskList};
use directories_next::ProjectDirs;
use lazy_static::lazy_static;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

/// Holds paths to application data and configuration files.
struct ProjPaths {
    data_path: PathBuf,  // Path to the data file where tasks are stored.
}

lazy_static! {
    /// Static instance of `ProjPaths` to manage file paths throughout the application.
    /// Ensures directories are created at the start and paths are available statically.
    static ref PROJ_PATHS: ProjPaths = {
        let proj_dirs = ProjectDirs::from("", "", "Task").unwrap();
        fs::create_dir_all(proj_dirs.data_dir()).unwrap();
        fs::create_dir_all(proj_dirs.config_dir()).unwrap();

        let data_path = proj_dirs.data_dir().join("data.json");

        ProjPaths {
            data_path,
        }
    };
}

/// Load tasks from the stored JSON file. If the file does not exist, it creates a new empty file.
pub fn load_task() -> Result<Vec<Task>, io::Error> {
    if !Path::new(PROJ_PATHS.data_path.as_path()).exists() {
        fs::File::create(PROJ_PATHS.data_path.as_path())?;
    }

    let stringified_task = fs::read_to_string(PROJ_PATHS.data_path.as_path())?;
    let task_list: TaskList = serde_json::from_str(&stringified_task).unwrap_or_default();

    Ok(task_list.task)
}

/// Save all tasks to a JSON file. Uses `TaskList` structure for serialization.
pub fn save_task(task: &[Task]) -> DynResult {
    let task_list = TaskList::new(task);
    let stringified_task = serde_json::to_string(&task_list)?;
    fs::write(PROJ_PATHS.data_path.as_path(), stringified_task)?;

    Ok(())
}

/// Load the welcome message from a text file.
pub fn load_welcome_message() -> Result<String, io::Error> {
    fs::read_to_string("src/welcome.txt")
}