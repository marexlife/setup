use std::fs::{File, create_dir};

pub fn create_files(
    files: &[crate::file::File<'_>],
    path: &str,
) {
    for file in files {
        File::create_new(path)
            .unwrap_or_else(|e| panic!("failed to create '{}' in path {} with error {}", file.get_name(), path, e));
    }
}

pub fn get_and_make_path(
    name: &str,
    path: &str,
) -> String {
    create_dir(path).unwrap_or_else(|e| {
        panic!("failed to create directory with error {}", e)
    });

    format!("{}/{}", name, path)
}

pub fn create_directory_and_files(
    name: &str,
    path: &str,
    files: Vec<crate::file::File>,
) {
    let path = get_and_make_path(name, path);

    create_files(&files, &path);
}
