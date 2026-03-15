use super::file_lists;
use crate::utils::create_directory_and_files;

pub(crate) fn create_root_directory_and_files(
    name: &str,
) {
    create_directory_and_files(
        name,
        name,
        file_lists::get_root_files(name),
    )
}

pub(crate) fn create_source_directory_and_files(
    name: &str,
) {
    create_directory_and_files(
        name,
        "Source",
        file_lists::get_source_files(name),
    );
}

pub(crate) fn create_main_directory_and_files(
    name: &str,
) {
    create_directory_and_files(
        name,
        "Main",
        file_lists::get_main_files(name),
    );
}
