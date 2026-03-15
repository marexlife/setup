use super::file_lists;
use crate::utils::{
    create_directory, create_directory_and_files,
    create_files,
};

#[must_use]
pub(crate) fn create_root_directory_and_files(
    name: &str,
) -> String {
    let path = create_directory(name.to_string());

    create_files(
        &name,
        file_lists::get_root_files(name),
    );

    path
}

#[must_use]
pub(crate) fn create_source_directory_and_files(
    parent: &str,
) -> String {
    let name = "Source";

    create_directory_and_files(
        parent,
        name,
        file_lists::get_source_files(),
    )
}

#[must_use]
pub(crate) fn create_main_mod_directory_and_files(
    project_name: &str,
    parent: &str,
) -> String {
    let name = "Main";

    create_directory_and_files(
        parent,
        name,
        file_lists::get_main_mod_files(
            project_name,
        ),
    )
}

#[must_use]
pub(crate) fn create_main_mod_private_directory_and_files(
    parent: &str,
) -> String {
    let name = "Private";

    create_directory_and_files(
        parent,
        name,
        file_lists::get_main_mod_private_files(),
    )
}
