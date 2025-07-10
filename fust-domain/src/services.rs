//! Domain services

use crate::{DomainError, Project, Task, TaskPriority, TaskStatus};

pub mod directory_service;

/// Domain service for project management
pub struct ProjectService;

impl ProjectService {
    /// Validate a project
    pub fn validate_project(project: &Project) -> Result<(), DomainError> {
        if project.name.is_empty() {
            return Err(DomainError::validation_error(
                "name",
                "Project name cannot be empty",
            ));
        }
        if project.path.is_empty() {
            return Err(DomainError::validation_error(
                "path",
                "Project path cannot be empty",
            ));
        }
        Ok(())
    }

    /// Check if a project can be deleted
    pub fn can_delete_project(_project: &Project) -> bool {
        // Business rule: projects can only be deleted if they have no active tasks
        // This would typically check against a task repository
        true // Simplified for now
    }
}

/// Domain service for task management
pub struct TaskService;

impl TaskService {
    /// Validate a task
    pub fn validate_task(task: &Task) -> Result<(), DomainError> {
        if task.title.is_empty() {
            return Err(DomainError::validation_error(
                "title",
                "Task title cannot be empty",
            ));
        }
        Ok(())
    }

    /// Check if a task can be completed
    pub fn can_complete_task(task: &Task) -> bool {
        matches!(task.status, TaskStatus::Todo | TaskStatus::InProgress)
    }

    /// Check if a task can be started
    pub fn can_start_task(task: &Task) -> bool {
        matches!(task.status, TaskStatus::Todo)
    }

    /// Calculate task priority score
    pub fn calculate_priority_score(task: &Task) -> u32 {
        match task.priority {
            TaskPriority::Low => 1,
            TaskPriority::Medium => 2,
            TaskPriority::High => 3,
            TaskPriority::Critical => 4,
        }
    }

    /// Check if task is overdue
    pub fn is_overdue(_task: &Task) -> bool {
        // This would typically check against a due date
        // For now, we'll use a simplified logic
        false
    }
}

/// Domain service for workspace management
pub struct WorkspaceService;

impl WorkspaceService {
    /// Validate workspace path
    pub fn validate_workspace_path(path: &str) -> Result<(), DomainError> {
        if path.is_empty() {
            return Err(DomainError::validation_error(
                "path",
                "Workspace path cannot be empty",
            ));
        }

        let path_obj = std::path::Path::new(path);
        if !path_obj.exists() {
            return Err(DomainError::validation_error(
                "path",
                "Workspace path does not exist",
            ));
        }

        if !path_obj.is_dir() {
            return Err(DomainError::validation_error(
                "path",
                "Workspace path must be a directory",
            ));
        }

        Ok(())
    }

    /// Check if path is a git repository
    pub fn is_git_repository(path: &str) -> bool {
        let git_dir = std::path::Path::new(path).join(".git");
        git_dir.exists() && git_dir.is_dir()
    }
}
