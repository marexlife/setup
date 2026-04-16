use std::process::{Command, exit};

use crate::utils::get_parent_directory;

pub fn run_proj() {
    run_proj_in_dir(".", &get_parent_directory());
}

pub fn run_proj_in_dir(dir: &str, name: &str) {
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

    let exit_code = child.wait().unwrap();

    if exit_code.code().unwrap() != 0 {
        exit(-1);
    }

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

    let exit_code = child.wait().unwrap();

    if exit_code.code().unwrap() != 0 {
        exit(-1);
    }

    let execute_path = format!(
        "./{dir}/build/Source/Main/{name}",
    );

    Command::new(&execute_path)
        .spawn()
        .unwrap_or_else(|e| {
            eprintln!(
                "Error when executing: {e:?}.
Path was: '{execute_path}'."
            );

            exit(0);
        });
}
