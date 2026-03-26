use super::file_lists;
use crate::{
    new_proj::hardcoded::file_lists::get_main_mod_private_files,
    shared,
    utils::{
        create_and_get_directory, create_files,
        create_sub_directory_and_files,
    },
};

#[must_use]
pub(crate) fn create_root_directory_and_files(
    name: &str,
) -> String {
    let path = create_and_get_directory(name.to_string());

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

    create_sub_directory_and_files(
        parent,
        name,
        file_lists::get_source_files(),
    )
}

#[must_use]
pub(crate) fn mod_directory_and_files(
    project_name: &str,
    parent: &str,
) -> String {
    let name = "Main";

    create_sub_directory_and_files(
        parent,
        name,
        file_lists::get_main_mod_files(
            project_name,
        ),
    )
}

#[must_use]
pub(crate) fn create_private_directory_and_files(
    parent: &str,
) -> String {
    shared::create_private_and_files(
        parent,
        get_main_mod_private_files(),
    )
}
