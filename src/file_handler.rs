///----------file-handlers.rs----------///

use crate::{DynResult, Task, TaskList};
use directories_next::ProjectDirs;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

/// Construct the paths to application data and configuration files.
fn get_project_paths() -> ProjPaths {
    let project_dirs = ProjectDirs::from("", "", "Task").expect("Failed to get project directories");
    fs::create_dir_all(project_dirs.data_dir()).expect("Failed to create data directory");
    fs::create_dir_all(project_dirs.config_dir()).expect("Failed to create config directory");

    let data_path = project_dirs.data_dir().join("data.json");

    ProjPaths {
        data_path,
    }
}

struct ProjPaths {
    data_path: PathBuf,  // Path to the data file where tasks are stored.
}

/// Load tasks from the stored JSON file. If the file does not exist, it creates a new empty file.
pub fn load_task() -> Result<Vec<Task>, io::Error> {
    let project_paths = get_project_paths();

    if !Path::new(&project_paths.data_path).exists() {
        fs::File::create(&project_paths.data_path)?;
    }

    let stringified_task = fs::read_to_string(&project_paths.data_path)?;
    let task_list: TaskList = serde_json::from_str(&stringified_task).unwrap_or_default();

    Ok(task_list.task)
}

/// Save all tasks to a JSON file. Uses `TaskList` structure for serialization.
pub fn save_task(task: &[Task]) -> DynResult {
    let project_paths = get_project_paths();
    let task_list = TaskList::new(task);
    let stringified_task = serde_json::to_string(&task_list)?;
    fs::write(&project_paths.data_path, stringified_task)?;

    Ok(())
}

/// Load the welcome message from a text file.
pub fn load_welcome_message() -> Result<String, io::Error> {
    fs::read_to_string("src/welcome.txt")
}


