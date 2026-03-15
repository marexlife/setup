use std::fs::{File, create_dir};

pub fn create_files(
    path: &str,
    files: Vec<crate::file::File<'_>>,
) {
    eprintln!("path: {}", path);

    for file in files {
        File::create(format!("{}/{}", path, file.get_name()))
            .unwrap_or_else(|e| panic!("failed to create '{}' in path {} with error {}", file.get_name(), path, e));
    }
}

pub fn create_directory(name: &str) {
    create_dir(name).unwrap_or_else(|e| {
        panic!("Attempt to create directory '{}' failed with error {}!", name, e);
    });
}

pub fn create_and_get_directory(
    parent: &str,
    sub: &str,
) -> String {
    create_directory(sub);

    format!("{}/{}", parent, sub)
}

pub fn create_directory_and_files(
    parent: &str,
    sub: &str,
    files: Vec<crate::file::File>,
) {
    let path =
        create_and_get_directory(parent, sub);

    create_files(&path, files);
}
