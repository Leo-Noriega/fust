use fust_application::list_directories::ListDirectories;
use fust_domain::metadata::DirectoryMetadata;
use fust_port::file_system::{DirectoryRepository, FsError};
use std::path::Path;
use std::time::SystemTime;

// Mock implementation for testing
#[derive(Debug, Clone)]
struct MockDirectoryRepository {
    directories: Vec<DirectoryMetadata>,
}

impl MockDirectoryRepository {
    fn new(directories: Vec<DirectoryMetadata>) -> Self {
        Self { directories }
    }
}

impl DirectoryRepository for MockDirectoryRepository {
    fn list_directories(&self, _dir: &Path) -> Result<Vec<DirectoryMetadata>, FsError> {
        Ok(self.directories.clone())
    }
}

#[test]
fn test_list_directories_usecase_creation() {
    let directories = vec![
        DirectoryMetadata::new(std::path::PathBuf::from("/tmp/dir1"), SystemTime::now()),
        DirectoryMetadata::new(std::path::PathBuf::from("/tmp/dir2"), SystemTime::now()),
        DirectoryMetadata::new(std::path::PathBuf::from("/tmp/dir3"), SystemTime::now()),
    ];

    let mock_repo = MockDirectoryRepository::new(directories.clone());
    let use_case = ListDirectories::new(mock_repo);

    assert!(use_case.execute(Path::new("/tmp")).is_ok());
}

#[test]
fn test_list_directories_execute_returns_exact_3_elements() {
    let directories = vec![
        DirectoryMetadata::new(std::path::PathBuf::from("/tmp/dir1"), SystemTime::now()),
        DirectoryMetadata::new(std::path::PathBuf::from("/tmp/dir2"), SystemTime::now()),
        DirectoryMetadata::new(std::path::PathBuf::from("/tmp/dir3"), SystemTime::now()),
    ];

    let mock_repo = MockDirectoryRepository::new(directories.clone());
    let use_case = ListDirectories::new(mock_repo);

    let result = use_case.execute(Path::new("/tmp"));
    assert!(result.is_ok());

    let returned_directories = result.unwrap();
    assert_eq!(returned_directories.len(), 3);
    assert_eq!(returned_directories, directories);
}

#[test]
fn test_list_directories_with_empty_directory() {
    let directories = vec![];

    let mock_repo = MockDirectoryRepository::new(directories.clone());
    let use_case = ListDirectories::new(mock_repo);

    let result = use_case.execute(Path::new("/tmp"));
    assert!(result.is_ok());

    let returned_directories = result.unwrap();
    assert_eq!(returned_directories.len(), 0);
    assert_eq!(returned_directories, directories);
}

#[test]
fn test_list_directories_error_propagation() {
    // Mock that returns error
    #[derive(Debug, Clone)]
    struct ErrorMockRepository;

    impl DirectoryRepository for ErrorMockRepository {
        fn list_directories(&self, _dir: &Path) -> Result<Vec<DirectoryMetadata>, FsError> {
            Err(FsError::PermissionDenied("Access denied".to_string()))
        }
    }

    let use_case = ListDirectories::new(ErrorMockRepository);
    let result = use_case.execute(Path::new("/tmp"));

    assert!(result.is_err());
    match result {
        Err(FsError::PermissionDenied(_)) => {
            // Expected error
        }
        _ => panic!("Expected PermissionDenied error"),
    }
}

#[test]
fn test_list_directories_different_paths() {
    let directories = vec![
        DirectoryMetadata::new(
            std::path::PathBuf::from("/home/user/dir1"),
            SystemTime::now(),
        ),
        DirectoryMetadata::new(
            std::path::PathBuf::from("/home/user/dir2"),
            SystemTime::now(),
        ),
    ];

    let mock_repo = MockDirectoryRepository::new(directories.clone());
    let use_case = ListDirectories::new(mock_repo);

    // Test with different path
    let result = use_case.execute(Path::new("/home/user"));
    assert!(result.is_ok());

    let returned_directories = result.unwrap();
    assert_eq!(returned_directories.len(), 2);
    assert_eq!(returned_directories, directories);
}

#[test]
fn test_list_directories_usecase_clone() {
    let directories = vec![DirectoryMetadata::new(
        std::path::PathBuf::from("/tmp/dir1"),
        SystemTime::now(),
    )];

    let mock_repo = MockDirectoryRepository::new(directories.clone());
    let use_case1 = ListDirectories::new(mock_repo);
    let use_case2 = use_case1.clone();

    // Both should behave the same
    let result1 = use_case1.execute(Path::new("/tmp"));
    let result2 = use_case2.execute(Path::new("/tmp"));

    assert!(result1.is_ok());
    assert!(result2.is_ok());
    assert_eq!(result1.unwrap(), result2.unwrap());
}

#[test]
fn test_list_directories_usecase_debug() {
    let directories = vec![DirectoryMetadata::new(
        std::path::PathBuf::from("/tmp/dir1"),
        SystemTime::now(),
    )];

    let mock_repo = MockDirectoryRepository::new(directories);
    let use_case = ListDirectories::new(mock_repo);

    let debug_output = format!("{use_case:?}");
    assert!(debug_output.contains("ListDirectories"));
}

#[test]
fn test_list_directories_dependency_injection() {
    let directories = vec![
        DirectoryMetadata::new(std::path::PathBuf::from("/tmp/dir1"), SystemTime::now()),
        DirectoryMetadata::new(std::path::PathBuf::from("/tmp/dir2"), SystemTime::now()),
        DirectoryMetadata::new(std::path::PathBuf::from("/tmp/dir3"), SystemTime::now()),
    ];

    // Test that any DirectoryRepository can be injected
    let mock_repo = MockDirectoryRepository::new(directories.clone());
    let use_case: ListDirectories<MockDirectoryRepository> = ListDirectories::new(mock_repo);

    let result = use_case.execute(Path::new("/tmp"));
    assert!(result.is_ok());

    let returned_directories = result.unwrap();
    assert_eq!(returned_directories.len(), 3);
    assert_eq!(returned_directories, directories);
}
