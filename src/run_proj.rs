use std::process::{Command, exit};

use crate::utils::get_parent_directory;

pub fn run_proj() {
    let mut child = Command::new("cmake")
        .arg(".")
        .arg("-B")
        .arg("build")
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
        .arg("build")
        .spawn()
        .unwrap_or_else(|e| {
            eprintln!(
                "Error when building: {e:?}"
            );

            exit(0);
        });

    child.wait().unwrap();

    Command::new(format!(
        "./build/Source/Main/{}",
        get_parent_directory()
    ))
    .spawn()
    .unwrap_or_else(|e| {
        eprintln!("Error when executing: {e:?}");

        exit(0);
    });
}
