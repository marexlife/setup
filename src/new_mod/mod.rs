mod hardcoded;

use hardcoded::*;

use crate::utils::get_parent_directory;

pub fn new_mod(name: &str) {
    let project_name = get_parent_directory();

    let mod_path = create_mod_root_and_files(
        "Source",
        name,
        name,
        &project_name,
    );

    let _ = create_public_and_files(
        &mod_path,
        name,
        &project_name,
    );

    let _ = create_private_and_files(
        &mod_path,
        name,
        &project_name,
    );

    update_source_cmake_lists_txt(name);

    println!(
        "# Link To {name}:
    
target_link_libraries(${{PROJECT_NAME}} PUBLIC
{name}
)"
    )
}
