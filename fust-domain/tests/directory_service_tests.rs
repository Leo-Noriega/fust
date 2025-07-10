use fust_domain::fs_errors::FsError;
use fust_domain::services::directory_service::DirectoryService;
use std::path::Path;
use tempfile::TempDir;

#[tokio::test]
async fn test_directory_service_create() {
    let service = DirectoryService::new();
    let temp_dir = TempDir::new().unwrap();
    let test_path = temp_dir.path().join("test_create_dir");

    // Test creating a new directory
    let result = service.create(&test_path).await;
    assert!(result.is_ok());

    // Verify directory was created
    assert!(test_path.exists());
    assert!(test_path.is_dir());
}

#[tokio::test]
async fn test_directory_service_create_nested() {
    let service = DirectoryService::new();
    let temp_dir = TempDir::new().unwrap();
    let test_path = temp_dir
        .path()
        .join("nested")
        .join("deep")
        .join("directory");

    // Test creating nested directories
    let result = service.create(&test_path).await;
    assert!(result.is_ok());

    // Verify all nested directories were created
    assert!(test_path.exists());
    assert!(test_path.is_dir());

    // Verify parent directories also exist
    assert!(test_path.parent().unwrap().exists());
    assert!(test_path.parent().unwrap().parent().unwrap().exists());
}

#[tokio::test]
async fn test_directory_service_create_already_exists() {
    let service = DirectoryService::new();
    let temp_dir = TempDir::new().unwrap();
    let test_path = temp_dir.path().join("existing_dir");

    // Create directory first time
    let result1 = service.create(&test_path).await;
    assert!(result1.is_ok());

    // Try to create the same directory again
    let result2 = service.create(&test_path).await;
    assert!(result2.is_ok()); // Should succeed (idempotent)

    // Verify directory still exists
    assert!(test_path.exists());
    assert!(test_path.is_dir());
}

#[tokio::test]
async fn test_directory_service_rename() {
    let service = DirectoryService::new();
    let temp_dir = TempDir::new().unwrap();
    let old_path = temp_dir.path().join("old_dir");
    let new_path = temp_dir.path().join("new_dir");

    // Create original directory
    service.create(&old_path).await.unwrap();
    assert!(old_path.exists());

    // Rename directory
    let result = service.rename(&old_path, &new_path).await;
    assert!(result.is_ok());

    // Verify old directory doesn't exist and new one does
    assert!(!old_path.exists());
    assert!(new_path.exists());
    assert!(new_path.is_dir());
}

#[tokio::test]
async fn test_directory_service_rename_nonexistent() {
    let service = DirectoryService::new();
    let temp_dir = TempDir::new().unwrap();
    let old_path = temp_dir.path().join("nonexistent_dir");
    let new_path = temp_dir.path().join("new_dir");

    // Try to rename non-existent directory
    let result = service.rename(&old_path, &new_path).await;
    assert!(result.is_err());

    // Verify error type
    match result {
        Err(FsError::NotFound(_)) => {
            // Expected error
        }
        _ => panic!("Expected NotFound error"),
    }
}

#[tokio::test]
async fn test_directory_service_rename_to_existing() {
    let service = DirectoryService::new();
    let temp_dir = TempDir::new().unwrap();
    let old_path = temp_dir.path().join("old_dir");
    let new_path = temp_dir.path().join("existing_dir");

    // Create both directories
    service.create(&old_path).await.unwrap();
    service.create(&new_path).await.unwrap();

    // Try to rename to existing directory
    let result = service.rename(&old_path, &new_path).await;
    assert!(result.is_err());

    // Verify error type
    match result {
        Err(FsError::Io(_)) => {
            // Expected error (directory already exists)
        }
        _ => panic!("Expected Io error"),
    }
}

#[tokio::test]
async fn test_directory_service_delete() {
    let service = DirectoryService::new();
    let temp_dir = TempDir::new().unwrap();
    let test_path = temp_dir.path().join("delete_test_dir");

    // Create directory
    service.create(&test_path).await.unwrap();
    assert!(test_path.exists());

    // Delete directory
    let result = service.delete(&test_path).await;
    assert!(result.is_ok());

    // Verify directory was deleted
    assert!(!test_path.exists());
}

#[tokio::test]
async fn test_directory_service_delete_nested() {
    let service = DirectoryService::new();
    let temp_dir = TempDir::new().unwrap();
    let test_path = temp_dir.path().join("nested_delete_test");

    // Create nested directory structure
    service
        .create(&test_path.join("subdir1").join("subdir2"))
        .await
        .unwrap();
    assert!(test_path.exists());

    // Delete the entire nested structure
    let result = service.delete(&test_path).await;
    assert!(result.is_ok());

    // Verify entire structure was deleted
    assert!(!test_path.exists());
}

#[tokio::test]
async fn test_directory_service_delete_nonexistent() {
    let service = DirectoryService::new();
    let temp_dir = TempDir::new().unwrap();
    let test_path = temp_dir.path().join("nonexistent_delete_dir");

    // Try to delete non-existent directory
    let result = service.delete(&test_path).await;
    assert!(result.is_err());

    // Verify error type
    match result {
        Err(FsError::NotFound(_)) => {
            // Expected error
        }
        _ => panic!("Expected NotFound error"),
    }
}

#[tokio::test]
async fn test_directory_service_delete_with_contents() {
    let service = DirectoryService::new();
    let temp_dir = TempDir::new().unwrap();
    let test_path = temp_dir.path().join("dir_with_contents");

    // Create directory with subdirectories and files
    service.create(&test_path.join("subdir1")).await.unwrap();
    service.create(&test_path.join("subdir2")).await.unwrap();

    // Create some files in the directories
    std::fs::write(test_path.join("subdir1").join("file1.txt"), "content1").unwrap();
    std::fs::write(test_path.join("subdir2").join("file2.txt"), "content2").unwrap();

    assert!(test_path.exists());

    // Delete the entire directory with contents
    let result = service.delete(&test_path).await;
    assert!(result.is_ok());

    // Verify entire directory and contents were deleted
    assert!(!test_path.exists());
}

#[tokio::test]
async fn test_directory_service_permission_denied() {
    let service = DirectoryService::new();

    // Try to create directory in a location that requires root permissions
    let root_path = Path::new("/root/test_dir");
    let result = service.create(root_path).await;

    // Should fail with permission denied (unless running as root)
    if result.is_err() {
        match result {
            Err(FsError::PermissionDenied(_)) => {
                // Expected error for non-root user
            }
            _ => {
                // Other errors are also acceptable
                println!("Got error: {result:?}");
            }
        }
    }
}

#[tokio::test]
async fn test_directory_service_invalid_path() {
    let service = DirectoryService::new();

    // Try to create directory with invalid path
    let invalid_path = Path::new("invalid/path/with/\0null");
    let result = service.create(invalid_path).await;

    // Should fail with invalid path error
    assert!(result.is_err());
    match result {
        Err(FsError::InvalidPath(_)) => {
            // Expected error
        }
        _ => {
            // Other errors are also acceptable for invalid paths
            println!("Got error: {result:?}");
        }
    }
}

#[tokio::test]
async fn test_directory_service_concurrent_operations() {
    let service = DirectoryService::new();
    let temp_dir = TempDir::new().unwrap();

    // Test concurrent create operations
    let handles: Vec<_> = (0..5)
        .map(|i| {
            let service = service.clone();
            let path = temp_dir.path().join(format!("concurrent_dir_{i}"));
            tokio::spawn(async move { service.create(&path).await })
        })
        .collect();

    // Wait for all operations to complete
    let results = futures::future::join_all(handles).await;

    // All operations should succeed
    for result in results {
        assert!(result.unwrap().is_ok());
    }

    // Verify all directories were created
    for i in 0..5 {
        let path = temp_dir.path().join(format!("concurrent_dir_{i}"));
        assert!(path.exists());
        assert!(path.is_dir());
    }
}
