use std::process::{Command, exit};

use crate::utils::get_parent_directory;

pub fn run_proj() {
    let output = Command::new("cmake")
        .arg(".")
        .arg("-B")
        .arg("build")
        .arg("-GNinja")
        .output()
        .unwrap_or_else(|e| {
            eprintln!(
                "Error when pre building: {e:?}"
            );

            exit(0);
        });
    println!("{output:?}");

    let output = Command::new("cmake")
        .arg("--build")
        .arg("build")
        .output()
        .unwrap_or_else(|e| {
            eprintln!(
                "Error when building: {e:?}"
            );

            exit(0);
        });
    println!("{output:?}");

    let output = Command::new(format!(
        "./build/Source/Main/{}",
        get_parent_directory()
    ))
    .output()
    .unwrap_or_else(|e| {
        eprintln!("Error when executing: {e:?}");

        exit(0);
    });
    println!("{output:?}");
}
