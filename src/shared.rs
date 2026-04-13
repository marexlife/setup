use crate::{
    file_data::FileData,
    utils::{
        create_and_get_directory,
        create_directory_and_files,
        create_sub_directory_and_files,
    },
};

#[must_use]
pub fn create_public_and_files(
    parent: &str,
    name: &str,
    project_name: &str,
    files: Vec<FileData>,
) -> String {
    let dir = create_and_get_directory(format!(
        "{parent}/Public"
    ));

    let dir = create_and_get_directory(format!(
        "{dir}/{project_name}"
    ));

    create_directory_and_files(
        format!("{dir}/{name}"),
        files,
    )
}

#[must_use]
pub fn create_private_and_files(
    parent: &str,
    files: Vec<FileData>,
) -> String {
    create_sub_directory_and_files(
        &parent, "Private", files,
    )
}
