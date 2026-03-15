use super::file_lists;
use crate::utils::{
    create_directory, create_directory_and_files,
    create_files,
};

#[must_use]
pub(crate) fn create_root_directory_and_files(
    name: &str,
) -> &str {
    let path = create_directory(name);

    create_files(
        &name,
        file_lists::get_root_files(name),
    );

    path
}

#[must_use]
pub(crate) fn create_source_directory_and_files(
    name: &str,
) -> String {
    create_directory_and_files(
        name,
        "Source",
        file_lists::get_source_files(name),
    )
}

#[must_use]
pub(crate) fn create_main_directory_and_files(
    name: &str,
) -> String {
    create_directory_and_files(
        name,
        "Main",
        file_lists::get_main_files(name),
    )
}
