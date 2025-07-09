use fust_domain::metadata::{DirectoryMetadata, FileMetadata};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

#[test]
fn test_file_metadata_creation() {
    let path = PathBuf::from("/tmp/test.txt");
    let size = 1024;
    let permissions = 0o644;
    let modified = SystemTime::now();

    let metadata = FileMetadata::new(path.clone(), size, permissions, modified);

    assert_eq!(metadata.path, path);
    assert_eq!(metadata.size, size);
    assert_eq!(metadata.permissions, permissions);
    assert_eq!(metadata.modified, modified);
}

#[test]
fn test_file_metadata_equality() {
    let path = PathBuf::from("/tmp/test.txt");
    let size = 1024;
    let permissions = 0o644;
    let modified = SystemTime::now();

    let metadata1 = FileMetadata::new(path.clone(), size, permissions, modified);
    let metadata2 = FileMetadata::new(path, size, permissions, modified);

    assert_eq!(metadata1, metadata2);
}

#[test]
fn test_file_metadata_extension() {
    let path = PathBuf::from("/tmp/test.txt");
    let metadata = FileMetadata::new(path, 1024, 0o644, SystemTime::now());

    assert_eq!(metadata.extension(), Some("txt"));
}

#[test]
fn test_file_metadata_extension_no_extension() {
    let path = PathBuf::from("/tmp/test");
    let metadata = FileMetadata::new(path, 1024, 0o644, SystemTime::now());

    assert_eq!(metadata.extension(), None);
}

#[test]
fn test_file_metadata_permissions() {
    let path = PathBuf::from("/tmp/test.txt");
    let metadata = FileMetadata::new(path, 1024, 0o755, SystemTime::now());

    assert!(metadata.is_executable());
    assert!(metadata.is_readable());
    assert!(metadata.is_writable());
}

#[test]
fn test_file_metadata_permissions_readonly() {
    let path = PathBuf::from("/tmp/test.txt");
    let metadata = FileMetadata::new(path, 1024, 0o444, SystemTime::now());

    assert!(!metadata.is_executable());
    assert!(metadata.is_readable());
    assert!(!metadata.is_writable());
}

#[test]
fn test_directory_metadata_creation() {
    let path = PathBuf::from("/tmp/test_dir");
    let created = SystemTime::now();

    let metadata = DirectoryMetadata::new(path.clone(), created);

    assert_eq!(metadata.path, path);
    assert_eq!(metadata.created, created);
}

#[test]
fn test_directory_metadata_equality() {
    let path = PathBuf::from("/tmp/test_dir");
    let created = SystemTime::now();

    let metadata1 = DirectoryMetadata::new(path.clone(), created);
    let metadata2 = DirectoryMetadata::new(path, created);

    assert_eq!(metadata1, metadata2);
}

#[test]
fn test_directory_metadata_name() {
    let path = PathBuf::from("/tmp/test_dir");
    let metadata = DirectoryMetadata::new(path, SystemTime::now());

    assert_eq!(metadata.name(), Some("test_dir"));
}

#[test]
fn test_directory_metadata_parent() {
    let path = PathBuf::from("/tmp/test_dir");
    let metadata = DirectoryMetadata::new(path, SystemTime::now());

    assert_eq!(metadata.parent(), Some(Path::new("/tmp")));
}

#[test]
fn test_directory_metadata_parent_root() {
    let path = PathBuf::from("/tmp");
    let metadata = DirectoryMetadata::new(path, SystemTime::now());

    assert_eq!(metadata.parent(), Some(Path::new("/")));
}

#[test]
fn test_serialization_deserialization() {
    let file_path = PathBuf::from("/tmp/test.txt");
    let dir_path = PathBuf::from("/tmp/test_dir");
    let now = SystemTime::now();

    let file_metadata = FileMetadata::new(file_path, 1024, 0o644, now);
    let dir_metadata = DirectoryMetadata::new(dir_path, now);

    // Test FileMetadata serialization
    let file_json = serde_json::to_string(&file_metadata).unwrap();
    let file_deserialized: FileMetadata = serde_json::from_str(&file_json).unwrap();
    assert_eq!(file_metadata, file_deserialized);

    // Test DirectoryMetadata serialization
    let dir_json = serde_json::to_string(&dir_metadata).unwrap();
    let dir_deserialized: DirectoryMetadata = serde_json::from_str(&dir_json).unwrap();
    assert_eq!(dir_metadata, dir_deserialized);
}

#[test]
fn test_clone() {
    let file_path = PathBuf::from("/tmp/test.txt");
    let dir_path = PathBuf::from("/tmp/test_dir");
    let now = SystemTime::now();

    let file_metadata = FileMetadata::new(file_path, 1024, 0o644, now);
    let dir_metadata = DirectoryMetadata::new(dir_path, now);

    let file_clone = file_metadata.clone();
    let dir_clone = dir_metadata.clone();

    assert_eq!(file_metadata, file_clone);
    assert_eq!(dir_metadata, dir_clone);
} 