use hardcoded::*;

mod hardcoded;

pub fn new_proj(name: &str) {
    create_root_directory_and_files(name);

    create_source_directory_and_files(name);

    create_main_directory_and_files(name);
}
