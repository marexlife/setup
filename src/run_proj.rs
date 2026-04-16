use std::process::{Command, Output, exit};

use crate::utils::get_parent_directory;

fn print_output(output: Output) {
    println!(
        "{}",
        String::from_utf8(output.stdout).unwrap()
    );
}

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
    print_output(output);

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
    print_output(output);

    let output = Command::new(format!(
        "./build/Source/Main/{}",
        get_parent_directory()
    ))
    .output()
    .unwrap_or_else(|e| {
        eprintln!("Error when executing: {e:?}");

        exit(0);
    });
    print_output(output);
}
