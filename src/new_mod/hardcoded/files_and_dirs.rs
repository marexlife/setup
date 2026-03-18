use std::path::PathBuf;

use crate::{
    new_mod::hardcoded::file_lists::{
        get_mod_root_files, get_private_files,
        get_public_files,
    },
    shared,
    utils::{
        create_directory,
        create_directory_and_files,
    },
};

#[must_use]
pub fn create_mod_root_and_files(
    parent: String,
    sub: &str,
    name: &str,
) -> String {
    create_directory_and_files(
        &parent,
        sub,
        get_mod_root_files(name),
    )
}

pub fn update_source_cmake_lists_txt(
    mod_name: String,
) {
    const CMAKE_LISTS_TXT: &'static str =
        "CMakeLists.txt";
    let mut b = PathBuf::new();
    b.push("Source");
    b.push(CMAKE_LISTS_TXT);
    let path = b.as_path();

    let v = std::fs::read(path).unwrap_or_else(|e| {
        panic!("failed to read contents of '{CMAKE_LISTS_TXT}' in path {path:?} with error {e}")
    });

    std::fs::write(path, format!("{}
add_subdirectory({mod_name})", String::from_utf8(v).unwrap_or_else(|e| {
            panic!("'{CMAKE_LISTS_TXT}' in path doesn't contain valid utf8. The error is {e}")
        })))
        .unwrap_or_else(|e| {
            panic!("failed to update '{CMAKE_LISTS_TXT}' in path '{path:?}' with error {e}")
        });
}

#[must_use]
pub fn create_public_and_files(
    parent: String,
    name: String,
    project_name: &str,
) -> String {
    shared::create_public_and_files(
        &parent,
        &name,
        project_name,
        get_public_files(&name, project_name),
    )
}

#[must_use]
pub fn create_private_and_files(
    parent: String,
    name: &str,
    project_name: &str,
) -> String {
    shared::create_private_and_files(
        parent,
        get_private_files(name, project_name),
    )
}
