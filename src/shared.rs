use crate::{
    file::FileData,
    utils::{
        create_directory,
        create_directory_and_files,
    },
};

#[must_use]
pub fn create_public_and_files(
    parent: &str,
    name: &str,
    project_name: &str,
    files: Vec<FileData>,
) -> String {
    let include_dir = create_directory(format!(
        "{parent}/Public/{project_name}/{name}"
    ));

    create_directory_and_files(
        &include_dir,
        &name,
        files,
    )
}

#[must_use]
pub fn create_private_and_files(
    parent: &str,
    files: Vec<FileData>,
) -> String {
    create_directory_and_files(
        &parent, "Private", files,
    )
}
