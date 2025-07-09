//! Repository implementations

use anyhow::Result;
use async_trait::async_trait;
use fust_domain::{Project, ProjectId, Task, TaskId};
use fust_port::{ProjectRepository, TaskRepository};
use serde_json;
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs;

/// File-based project repository implementation
pub struct FileProjectRepository {
    data_path: PathBuf,
}

impl FileProjectRepository {
    /// Create a new file-based project repository
    pub fn new(data_path: PathBuf) -> Self {
        Self { data_path }
    }

    /// Get the projects file path
    fn projects_file(&self) -> PathBuf {
        self.data_path.join("projects.json")
    }

    /// Load projects from file
    async fn load_projects(&self) -> Result<HashMap<String, Project>> {
        let file_path = self.projects_file();
        if file_path.exists() {
            let content = fs::read_to_string(&file_path).await?;
            let projects: HashMap<String, Project> = serde_json::from_str(&content)?;
            Ok(projects)
        } else {
            Ok(HashMap::new())
        }
    }

    /// Save projects to file
    async fn save_projects(&self, projects: &HashMap<String, Project>) -> Result<()> {
        // Ensure directory exists
        if let Some(parent) = self.data_path.parent() {
            fs::create_dir_all(parent).await?;
        }
        
        let content = serde_json::to_string_pretty(projects)?;
        fs::write(&self.projects_file(), content).await?;
        Ok(())
    }
}

#[async_trait]
impl ProjectRepository for FileProjectRepository {
    async fn find_by_id(&self, id: &ProjectId) -> Result<Option<Project>> {
        let projects = self.load_projects().await?;
        Ok(projects.get(&id.0).cloned())
    }

    async fn find_all(&self) -> Result<Vec<Project>> {
        let projects = self.load_projects().await?;
        Ok(projects.values().cloned().collect())
    }

    async fn save(&self, project: &Project) -> Result<()> {
        let mut projects = self.load_projects().await?;
        projects.insert(project.id.0.clone(), project.clone());
        self.save_projects(&projects).await
    }

    async fn delete(&self, id: &ProjectId) -> Result<bool> {
        let mut projects = self.load_projects().await?;
        let removed = projects.remove(&id.0).is_some();
        if removed {
            self.save_projects(&projects).await?;
        }
        Ok(removed)
    }

    async fn find_by_tag(&self, tag: &str) -> Result<Vec<Project>> {
        let projects = self.load_projects().await?;
        let filtered: Vec<Project> = projects
            .values()
            .filter(|project| project.tags.contains(&tag.to_string()))
            .cloned()
            .collect();
        Ok(filtered)
    }

    async fn find_by_path(&self, path: &str) -> Result<Option<Project>> {
        let projects = self.load_projects().await?;
        Ok(projects.values().find(|p| p.path == path).cloned())
    }
}

/// File-based task repository implementation
pub struct FileTaskRepository {
    data_path: PathBuf,
}

impl FileTaskRepository {
    /// Create a new file-based task repository
    pub fn new(data_path: PathBuf) -> Self {
        Self { data_path }
    }

    /// Get the tasks file path
    fn tasks_file(&self) -> PathBuf {
        self.data_path.join("tasks.json")
    }

    /// Load tasks from file
    async fn load_tasks(&self) -> Result<HashMap<String, Task>> {
        let file_path = self.tasks_file();
        if file_path.exists() {
            let content = fs::read_to_string(&file_path).await?;
            let tasks: HashMap<String, Task> = serde_json::from_str(&content)?;
            Ok(tasks)
        } else {
            Ok(HashMap::new())
        }
    }

    /// Save tasks to file
    async fn save_tasks(&self, tasks: &HashMap<String, Task>) -> Result<()> {
        // Ensure directory exists
        if let Some(parent) = self.data_path.parent() {
            fs::create_dir_all(parent).await?;
        }
        
        let content = serde_json::to_string_pretty(tasks)?;
        fs::write(&self.tasks_file(), content).await?;
        Ok(())
    }
}

#[async_trait]
impl TaskRepository for FileTaskRepository {
    async fn find_by_id(&self, id: &TaskId) -> Result<Option<Task>> {
        let tasks = self.load_tasks().await?;
        Ok(tasks.get(&id.0).cloned())
    }

    async fn find_all(&self) -> Result<Vec<Task>> {
        let tasks = self.load_tasks().await?;
        Ok(tasks.values().cloned().collect())
    }

    async fn find_by_project(&self, project_id: &ProjectId) -> Result<Vec<Task>> {
        let tasks = self.load_tasks().await?;
        let filtered: Vec<Task> = tasks
            .values()
            .filter(|task| task.project_id.as_ref() == Some(project_id))
            .cloned()
            .collect();
        Ok(filtered)
    }

    async fn save(&self, task: &Task) -> Result<()> {
        let mut tasks = self.load_tasks().await?;
        tasks.insert(task.id.0.clone(), task.clone());
        self.save_tasks(&tasks).await
    }

    async fn delete(&self, id: &TaskId) -> Result<bool> {
        let mut tasks = self.load_tasks().await?;
        let removed = tasks.remove(&id.0).is_some();
        if removed {
            self.save_tasks(&tasks).await?;
        }
        Ok(removed)
    }

    async fn find_by_status(&self, status: &fust_domain::TaskStatus) -> Result<Vec<Task>> {
        let tasks = self.load_tasks().await?;
        let filtered: Vec<Task> = tasks
            .values()
            .filter(|task| std::mem::discriminant(&task.status) == std::mem::discriminant(status))
            .cloned()
            .collect();
        Ok(filtered)
    }

    async fn find_by_priority(&self, priority: &fust_domain::TaskPriority) -> Result<Vec<Task>> {
        let tasks = self.load_tasks().await?;
        let filtered: Vec<Task> = tasks
            .values()
            .filter(|task| std::mem::discriminant(&task.priority) == std::mem::discriminant(priority))
            .cloned()
            .collect();
        Ok(filtered)
    }
} 