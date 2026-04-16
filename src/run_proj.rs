use std::process::{Command, exit};

use crate::utils::get_parent_directory;

pub fn run_proj() {
    run_proj_in_dir(".");
}

pub fn run_proj_in_dir(dir: &str) {
    let mut child = Command::new("cmake")
        .arg(format!("./{dir}"))
        .arg("-B")
        .arg(format!("./{dir}/build"))
        .arg("-GNinja")
        .spawn()
        .unwrap_or_else(|e| {
            eprintln!(
                "Error when pre building: {e:?}"
            );

            exit(0);
        });

    child.wait().unwrap();

    let mut child = Command::new("cmake")
        .arg("--build")
        .arg(format!("./{dir}/build"))
        .spawn()
        .unwrap_or_else(|e| {
            eprintln!(
                "Error when building: {e:?}"
            );

            exit(0);
        });

    child.wait().unwrap();

    let execute_path = format!(
        "./{dir}/build/Source/Main/{}",
        get_parent_directory()
    );

    Command::new(&execute_path)
        .spawn()
        .unwrap_or_else(|e| {
            eprintln!(
                "Error when executing: {e:?}.
Path was: {execute_path}."
            );

            exit(0);
        });
}
