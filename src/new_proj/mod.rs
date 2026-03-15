use hardcoded::*;

mod hardcoded;

pub fn new_proj(name: &str) {
    let project_path =
        create_root_directory_and_files(name);

    let source_path =
        create_source_directory_and_files(
            project_path,
        );

    let _ = create_main_directory_and_files(
        &source_path,
    );
}
