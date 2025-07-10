use fust_domain::metadata::{DirectoryMetadata, FileMetadata};
use fust_port::file_system::{DirectoryRepository, FileRepository, FuzzyFinder};
use std::path::Path;

// Mock implementations for testing
struct MockFileRepository {
    files: Vec<FileMetadata>,
}

impl MockFileRepository {
    fn new(files: Vec<FileMetadata>) -> Self {
        Self { files }
    }
}

impl FileRepository for MockFileRepository {
    fn list_files(
        &self,
        _dir: &Path,
    ) -> Result<Vec<FileMetadata>, fust_port::file_system::FsError> {
        Ok(self.files.clone())
    }
}

struct MockDirectoryRepository {
    directories: Vec<DirectoryMetadata>,
}

impl MockDirectoryRepository {
    fn new(directories: Vec<DirectoryMetadata>) -> Self {
        Self { directories }
    }
}

impl DirectoryRepository for MockDirectoryRepository {
    fn list_directories(
        &self,
        _dir: &Path,
    ) -> Result<Vec<DirectoryMetadata>, fust_port::file_system::FsError> {
        Ok(self.directories.clone())
    }
}

struct MockFuzzyFinder;

impl FuzzyFinder<FileMetadata> for MockFuzzyFinder {
    fn find(
        &self,
        items: &[FileMetadata],
    ) -> Result<Vec<usize>, fust_port::file_system::FinderError> {
        // Simple mock that returns first item if it contains "test"
        let mut results = Vec::new();
        for (i, item) in items.iter().enumerate() {
            if let Some(name) = item.path.file_name() {
                if name.to_string_lossy().contains("test") {
                    results.push(i);
                }
            }
        }
        Ok(results)
    }
}

impl FuzzyFinder<DirectoryMetadata> for MockFuzzyFinder {
    fn find(
        &self,
        items: &[DirectoryMetadata],
    ) -> Result<Vec<usize>, fust_port::file_system::FinderError> {
        // Simple mock that returns first item if it contains "test"
        let mut results = Vec::new();
        for (i, item) in items.iter().enumerate() {
            if let Some(name) = item.path.file_name() {
                if name.to_string_lossy().contains("test") {
                    results.push(i);
                }
            }
        }
        Ok(results)
    }
}

#[test]
fn test_file_repository_trait() {
    use std::path::PathBuf;
    use std::time::SystemTime;

    let files = vec![
        FileMetadata::new(
            PathBuf::from("/tmp/test1.txt"),
            1024,
            0o644,
            SystemTime::now(),
        ),
        FileMetadata::new(
            PathBuf::from("/tmp/test2.txt"),
            2048,
            0o644,
            SystemTime::now(),
        ),
    ];

    let repository = MockFileRepository::new(files.clone());
    let result = repository.list_files(Path::new("/tmp"));

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), files);
}

#[test]
fn test_directory_repository_trait() {
    use std::path::PathBuf;
    use std::time::SystemTime;

    let directories = vec![
        DirectoryMetadata::new(PathBuf::from("/tmp/dir1"), SystemTime::now()),
        DirectoryMetadata::new(PathBuf::from("/tmp/dir2"), SystemTime::now()),
    ];

    let repository = MockDirectoryRepository::new(directories.clone());
    let result = repository.list_directories(Path::new("/tmp"));

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), directories);
}

#[test]
fn test_fuzzy_finder_with_files() {
    use std::path::PathBuf;
    use std::time::SystemTime;

    let files = vec![
        FileMetadata::new(
            PathBuf::from("/tmp/test1.txt"),
            1024,
            0o644,
            SystemTime::now(),
        ),
        FileMetadata::new(
            PathBuf::from("/tmp/other.txt"),
            2048,
            0o644,
            SystemTime::now(),
        ),
        FileMetadata::new(
            PathBuf::from("/tmp/test2.txt"),
            3072,
            0o644,
            SystemTime::now(),
        ),
    ];

    let finder = MockFuzzyFinder;
    let result = finder.find(&files);

    assert!(result.is_ok());
    let indices = result.unwrap();
    assert_eq!(indices, vec![0, 2]); // test1.txt and test2.txt
}

#[test]
fn test_fuzzy_finder_with_directories() {
    use std::path::PathBuf;
    use std::time::SystemTime;

    let directories = vec![
        DirectoryMetadata::new(PathBuf::from("/tmp/test_dir1"), SystemTime::now()),
        DirectoryMetadata::new(PathBuf::from("/tmp/other_dir"), SystemTime::now()),
        DirectoryMetadata::new(PathBuf::from("/tmp/test_dir2"), SystemTime::now()),
    ];

    let finder = MockFuzzyFinder;
    let result = finder.find(&directories);

    assert!(result.is_ok());
    let indices = result.unwrap();
    assert_eq!(indices, vec![0, 2]); // test_dir1 and test_dir2
}

#[test]
fn test_fuzzy_finder_no_matches() {
    use std::path::PathBuf;
    use std::time::SystemTime;

    let files = vec![
        FileMetadata::new(
            PathBuf::from("/tmp/other1.txt"),
            1024,
            0o644,
            SystemTime::now(),
        ),
        FileMetadata::new(
            PathBuf::from("/tmp/other2.txt"),
            2048,
            0o644,
            SystemTime::now(),
        ),
    ];

    let finder = MockFuzzyFinder;
    let result = finder.find(&files);

    assert!(result.is_ok());
    let indices = result.unwrap();
    assert_eq!(indices, vec![]); // No matches
}

#[test]
fn test_fuzzy_finder_empty_list() {
    let files: Vec<FileMetadata> = vec![];

    let finder = MockFuzzyFinder;
    let result = finder.find(&files);

    assert!(result.is_ok());
    let indices = result.unwrap();
    assert_eq!(indices, vec![]);
}
