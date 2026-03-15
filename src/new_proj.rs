use std::fs::{File, create_dir};

pub fn new_proj(name: &str) {
    create_dir(name).unwrap_or_else(|e| {
        panic!("failed to create project directory with error {e}");
    });

    let root_files = [
        crate::file::File::new(
            "CMakeLists.txt",
            format!(
                r"cmake_minimum_required(VERSION 3.20)
project({name})

add_subdirectory(Source)
"
            ),
        ),
        crate::file::File::new(
            "clang-tidy",
            "Checks: 'cppcoreguidelines-*'"
                .to_string(),
        ),
        crate::file::File::new(
            "clang-format",
            r"---
BasedOnStyle: Microsoft
PointerAlignment: Left
ColumnLimit: 70"
                .to_string(),
        ),
    ];

    for root_file in root_files {
        File::create_new(name)
            .unwrap_or_else(|e| panic!("failed to create '{}' in path {} with error {}", root_file.get_name(), name, e));
    }
}
