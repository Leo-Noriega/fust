use fust_adapter::filesystem_adapter::FileSystemAdapter;
use fust_port::file_system::{DirectoryRepository, FileRepository, FsError};
use std::path::Path;

// Dummy use case for testing dependency injection
struct DummyUseCase<FR, DR>
where
    FR: FileRepository,
    DR: DirectoryRepository,
{
    file_repository: FR,
    directory_repository: DR,
}

impl<FR, DR> DummyUseCase<FR, DR>
where
    FR: FileRepository,
    DR: DirectoryRepository,
{
    fn new(file_repository: FR, directory_repository: DR) -> Self {
        Self {
            file_repository,
            directory_repository,
        }
    }

    fn list_files_in_directory(
        &self,
        dir: &Path,
    ) -> Result<Vec<fust_domain::metadata::FileMetadata>, FsError> {
        self.file_repository.list_files(dir)
    }

    fn list_directories_in_directory(
        &self,
        dir: &Path,
    ) -> Result<Vec<fust_domain::metadata::DirectoryMetadata>, FsError> {
        self.directory_repository.list_directories(dir)
    }
}

#[test]
fn test_filesystem_adapter_creation() {
    let adapter = FileSystemAdapter::new();
    assert!(adapter.is_ok());
}

#[test]
fn test_file_repository_trait_implementation() {
    let adapter = FileSystemAdapter::new().unwrap();
    let result = adapter.list_files(Path::new("/tmp"));

    assert!(result.is_err());
    match result {
        Err(FsError::NotImplemented(_)) => {
            // Expected behavior - not implemented yet
        }
        _ => panic!("Expected NotImplemented error"),
    }
}

#[test]
fn test_directory_repository_trait_implementation() {
    let adapter = FileSystemAdapter::new().unwrap();
    let result = adapter.list_directories(Path::new("/tmp"));

    assert!(result.is_err());
    match result {
        Err(FsError::NotImplemented(_)) => {
            // Expected behavior - not implemented yet
        }
        _ => panic!("Expected NotImplemented error"),
    }
}

#[test]
fn test_dependency_injection_with_filesystem_adapter() {
    let adapter = FileSystemAdapter::new().unwrap();
    let use_case = DummyUseCase::new(adapter.clone(), adapter);

    // Test file repository injection
    let file_result = use_case.list_files_in_directory(Path::new("/tmp"));
    assert!(file_result.is_err());
    match file_result {
        Err(FsError::NotImplemented(_)) => {
            // Expected behavior
        }
        _ => panic!("Expected NotImplemented error for files"),
    }

    // Test directory repository injection
    let dir_result = use_case.list_directories_in_directory(Path::new("/tmp"));
    assert!(dir_result.is_err());
    match dir_result {
        Err(FsError::NotImplemented(_)) => {
            // Expected behavior
        }
        _ => panic!("Expected NotImplemented error for directories"),
    }
}

#[test]
fn test_filesystem_adapter_send_sync() {
    let adapter = FileSystemAdapter::new().unwrap();

    // Test that adapter can be sent to another thread
    let handle = std::thread::spawn(move || {
        let _result = adapter.list_files(Path::new("/tmp"));
    });

    assert!(handle.join().is_ok());
}

#[test]
fn test_filesystem_adapter_clone() {
    let adapter1 = FileSystemAdapter::new().unwrap();
    let adapter2 = adapter1.clone();

    // Both adapters should behave the same way
    let result1 = adapter1.list_files(Path::new("/tmp"));
    let result2 = adapter2.list_files(Path::new("/tmp"));

    assert!(result1.is_err());
    assert!(result2.is_err());

    match (result1, result2) {
        (Err(FsError::NotImplemented(_)), Err(FsError::NotImplemented(_))) => {
            // Both should return NotImplemented
        }
        _ => panic!("Both adapters should return NotImplemented error"),
    }
}

#[test]
fn test_filesystem_adapter_debug() {
    let adapter = FileSystemAdapter::new().unwrap();
    let debug_output = format!("{adapter:?}");

    // Should contain adapter name
    assert!(debug_output.contains("FileSystemAdapter"));
}

#[test]
fn test_error_propagation() {
    let adapter = FileSystemAdapter::new().unwrap();

    // Test that errors are properly propagated
    let file_result = adapter.list_files(Path::new("/nonexistent"));
    let dir_result = adapter.list_directories(Path::new("/nonexistent"));

    // Both should return NotImplemented, not other error types
    assert!(matches!(file_result, Err(FsError::NotImplemented(_))));
    assert!(matches!(dir_result, Err(FsError::NotImplemented(_))));
}
