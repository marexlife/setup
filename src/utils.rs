use std::{
    fs::{File, create_dir},
    io::Write,
};

pub fn create_files(
    path: &str,
    files: Vec<crate::file::File>,
) {
    eprintln!("path: {}", path);

    for file in files {
        File::create(format!("{}/{}", path, file.get_name()))
            .unwrap_or_else(|e| 
                panic!("failed to create '{}' in path {} with error {}", file.get_name(), path, e)
            )
            .write_all(file.get_contents().as_bytes())
            .unwrap_or_else(|e| 
                panic!("failed to write contents in file '{}' with error {}", file.get_name(), e)
            );
    }
}

pub fn create_directory(name: &str) -> &str {
    println!("CREATING: {name}");

    create_dir(name).unwrap_or_else(|e| {
        panic!("Attempt to create directory '{}' failed with error {}!", name, e);
    });

    name
}

pub fn create_and_get_directory(
    parent: &str,
    sub: &str,
) -> String {
    let new = format!("{}/{}", parent, sub);

    create_directory(&new);

    new
}

#[must_use]
pub fn create_directory_and_files(
    parent: &str,
    sub: &str,
    files: Vec<crate::file::File>,
) -> String {
    let path =
        create_and_get_directory(parent, sub);

    create_files(&path, files);

    path
}
