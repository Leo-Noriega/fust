//! Application use cases

use anyhow::Result;
use fust_domain::{Project, ProjectId, Task, TaskId, TaskPriority};
use fust_port::{ProjectRepository, TaskRepository};

/// Use case for project management
pub struct ProjectUseCase<R>
where
    R: ProjectRepository,
{
    project_repo: R,
}

impl<R> ProjectUseCase<R>
where
    R: ProjectRepository,
{
    /// Create a new project use case
    pub fn new(project_repo: R) -> Self {
        Self { project_repo }
    }

    /// Create a new project
    pub async fn create_project(
        &self,
        name: String,
        path: String,
        description: Option<String>,
    ) -> Result<Project> {
        let project_id = ProjectId::new(uuid::Uuid::new_v4().to_string());
        let project = Project::new(project_id, name, path, description);

        // Validate project
        fust_domain::ProjectService::validate_project(&project)?;

        // Save project
        self.project_repo.save(&project).await?;

        Ok(project)
    }

    /// Get a project by ID
    pub async fn get_project(&self, id: &ProjectId) -> Result<Option<Project>> {
        self.project_repo.find_by_id(id).await
    }

    /// List all projects
    pub async fn list_projects(&self) -> Result<Vec<Project>> {
        self.project_repo.find_all().await
    }

    /// Delete a project
    pub async fn delete_project(&self, id: &ProjectId) -> Result<bool> {
        // Check if project can be deleted
        if let Some(project) = self.project_repo.find_by_id(id).await? {
            if !fust_domain::ProjectService::can_delete_project(&project) {
                return Err(anyhow::anyhow!("Project cannot be deleted"));
            }
        }

        self.project_repo.delete(id).await
    }

    /// Find projects by tag
    pub async fn find_projects_by_tag(&self, tag: &str) -> Result<Vec<Project>> {
        self.project_repo.find_by_tag(tag).await
    }
}

/// Use case for task management
pub struct TaskUseCase<R>
where
    R: TaskRepository,
{
    task_repo: R,
}

impl<R> TaskUseCase<R>
where
    R: TaskRepository,
{
    /// Create a new task use case
    pub fn new(task_repo: R) -> Self {
        Self { task_repo }
    }

    /// Create a new task
    pub async fn create_task(
        &self,
        title: String,
        description: Option<String>,
        priority: TaskPriority,
        _project_id: Option<ProjectId>,
    ) -> Result<Task> {
        let task_id = TaskId::new(uuid::Uuid::new_v4().to_string());
        let task = Task::new(task_id, title, description, priority);

        // Validate task
        fust_domain::TaskService::validate_task(&task)?;

        // Save task
        self.task_repo.save(&task).await?;

        Ok(task)
    }

    /// Get a task by ID
    pub async fn get_task(&self, id: &TaskId) -> Result<Option<Task>> {
        self.task_repo.find_by_id(id).await
    }

    /// List all tasks
    pub async fn list_tasks(&self) -> Result<Vec<Task>> {
        self.task_repo.find_all().await
    }

    /// Complete a task
    pub async fn complete_task(&self, id: &TaskId) -> Result<()> {
        if let Some(mut task) = self.task_repo.find_by_id(id).await? {
            if fust_domain::TaskService::can_complete_task(&task) {
                task.complete();
                self.task_repo.save(&task).await?;
                Ok(())
            } else {
                Err(anyhow::anyhow!("Task cannot be completed"))
            }
        } else {
            Err(anyhow::anyhow!("Task not found"))
        }
    }

    /// Start a task
    pub async fn start_task(&self, id: &TaskId) -> Result<()> {
        if let Some(mut task) = self.task_repo.find_by_id(id).await? {
            if fust_domain::TaskService::can_start_task(&task) {
                task.start();
                self.task_repo.save(&task).await?;
                Ok(())
            } else {
                Err(anyhow::anyhow!("Task cannot be started"))
            }
        } else {
            Err(anyhow::anyhow!("Task not found"))
        }
    }

    /// Find tasks by project
    pub async fn find_tasks_by_project(&self, project_id: &ProjectId) -> Result<Vec<Task>> {
        self.task_repo.find_by_project(project_id).await
    }
}
