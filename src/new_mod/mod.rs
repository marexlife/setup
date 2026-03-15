mod hardcoded;

use hardcoded::*;

use crate::utils::get_parent_directory;

pub fn new_mod(name: &str) {
    let project_name = get_parent_directory();

    let mod_path = create_mod_root_and_files(
        format!("Source/{}", project_name),
        name,
        name,
    );

    let _ = create_public_and_files(
        mod_path.clone(),
        name.to_string(),
        &project_name,
    );

    let _ = create_private_and_files(
        mod_path,
        name,
        &project_name,
    );
}
