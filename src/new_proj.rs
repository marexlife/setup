use std::{
    env::join_paths,
    fs::{File, create_dir},
    usize,
};

use crate::utils::{
    create_files, get_and_make_path,
};

const SOURCE: &'static str = "Source";

fn get_root_files<'a>(
    name: &'a str,
) -> [crate::file::File<'a>; 3] {
    [
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
            ".clang-tidy",
            "Checks: 'cppcoreguidelines-*'"
                .to_string(),
        ),
        crate::file::File::new(
            ".clang-format",
            r"---
BasedOnStyle: Microsoft
PointerAlignment: Left
ColumnLimit: 70"
                .to_string(),
        ),
    ]
}

fn get_source_files<'a>(
    name: &'a str,
) -> [crate::file::File<'a>; 1] {
    [crate::file::File::new(
        name,
        r"cmake_minimum_required(VERSION 3.20)

add_subdirectory(Main)"
            .to_string(),
    )]
}

fn create_root_directory_and_files(name: &str) {
    create_dir(name).unwrap_or_else(|e| {
        panic!("failed to create project directory with error {e}");
    });

    let root_files = get_root_files(name);
    create_files(&root_files, name);
}

fn create_source_directory_and_files(name: &str) {
    let path = get_and_make_path(name, SOURCE);

    let source_files = get_source_files(name);
    create_files(&source_files, &path);
}

pub fn new_proj(name: &str) {
    create_root_directory_and_files(name);

    create_source_directory_and_files(name);

    "cmake_minimum_required(VERSION 3.20)
project({name})

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_STANDARD 23)

add_executable(${{PROJECT_NAME}}
    Private/main.cpp
)";
}
