//! Repository interfaces

use async_trait::async_trait;
use fust_domain::{Project, ProjectId, Task, TaskId};

/// Repository for project management
#[async_trait]
pub trait ProjectRepository: Send + Sync {
    /// Find a project by ID
    async fn find_by_id(&self, id: &ProjectId) -> anyhow::Result<Option<Project>>;

    /// Find all projects
    async fn find_all(&self) -> anyhow::Result<Vec<Project>>;

    /// Save a project
    async fn save(&self, project: &Project) -> anyhow::Result<()>;

    /// Delete a project
    async fn delete(&self, id: &ProjectId) -> anyhow::Result<bool>;

    /// Find projects by tag
    async fn find_by_tag(&self, tag: &str) -> anyhow::Result<Vec<Project>>;

    /// Find projects by path
    async fn find_by_path(&self, path: &str) -> anyhow::Result<Option<Project>>;
}

/// Repository for task management
#[async_trait]
pub trait TaskRepository: Send + Sync {
    /// Find a task by ID
    async fn find_by_id(&self, id: &TaskId) -> anyhow::Result<Option<Task>>;

    /// Find all tasks
    async fn find_all(&self) -> anyhow::Result<Vec<Task>>;

    /// Find tasks by project
    async fn find_by_project(&self, project_id: &ProjectId) -> anyhow::Result<Vec<Task>>;

    /// Save a task
    async fn save(&self, task: &Task) -> anyhow::Result<()>;

    /// Delete a task
    async fn delete(&self, id: &TaskId) -> anyhow::Result<bool>;

    /// Find tasks by status
    async fn find_by_status(&self, status: &fust_domain::TaskStatus) -> anyhow::Result<Vec<Task>>;

    /// Find tasks by priority
    async fn find_by_priority(
        &self,
        priority: &fust_domain::TaskPriority,
    ) -> anyhow::Result<Vec<Task>>;
}
