use std::fs;
use std::path::Path;
use serde_json;
use uuid::Uuid;
use crate::model::Task;

pub struct TaskStorage {
    file_path: String,
}

impl TaskStorage {
    pub fn new(file_path: &str) -> Self {
        TaskStorage {
            file_path: file_path.to_string(),
        }
    }

    pub fn save_tasks(&self, tasks: &[Task]) -> Result<(), std::io::Error> {
        let json = serde_json::to_string_pretty(tasks)?;
        fs::write(&self.file_path, json)
    }

    pub fn load_tasks(&self) -> Result<Vec<Task>, std::io::Error> {
        if !Path::new(&self.file_path).exists() {
            return Ok(Vec::new());
        }

        let contents = fs::read_to_string(&self.file_path)?;
        let tasks: Vec<Task> = serde_json::from_str(&contents)?;
        Ok(tasks)
    }

    pub fn add_task(&self, task: Task) -> Result<(), std::io::Error> {
        let mut tasks = self.load_tasks()?;
        tasks.push(task);
        self.save_tasks(&tasks)
    }

    pub fn update_task(&self, task_id: Uuid, updated_task: Task) -> Result<(), std::io::Error> {
        let mut tasks = self.load_tasks()?;
        if let Some(task) = tasks.iter_mut().find(|t| t.id == task_id) {
            *task = updated_task;
        }
        self.save_tasks(&tasks)
    }

    pub fn delete_task(&self, task_id: Uuid) -> Result<(), std::io::Error> {
        let mut tasks = self.load_tasks()?;
        tasks.retain(|task| task.id != task_id);
        self.save_tasks(&tasks)
    }
}