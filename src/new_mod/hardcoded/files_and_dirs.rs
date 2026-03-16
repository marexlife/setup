use crate::{
    new_mod::hardcoded::file_lists::{
        get_mod_root_files, get_private_files,
        get_public_files,
    },
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

#[must_use]
pub fn create_public_and_files(
    parent: String,
    name: String,
    project_name: &str,
) -> String {
    let include_dir = create_directory(format!(
        "{}/Public",
        parent,
    ));

    create_directory_and_files(
        &include_dir,
        &name,
        get_public_files(&name, project_name),
    )
}

#[must_use]
pub fn create_private_and_files(
    parent: String,
    name: &str,
    project_name: &str,
) -> String {
    create_directory_and_files(
        &parent,
        "Private",
        get_private_files(name, project_name),
    )
}
