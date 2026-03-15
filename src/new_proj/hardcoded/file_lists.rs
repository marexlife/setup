use crate::file::File;

pub fn get_root_files(
    name: &str,
) -> Vec<File<'_>> {
    vec![
        File::new(
            "CMakeLists.txt",
            format!(
                r"cmake_minimum_required(VERSION 3.20)
project({name})

add_subdirectory(Source)
"
            ),
        ),
        File::new(
            ".clang-tidy",
            "Checks: 'cppcoreguidelines-*'"
                .to_string(),
        ),
        File::new(
            ".clang-format",
            r"---
BasedOnStyle: Microsoft
PointerAlignment: Left
ColumnLimit: 70"
                .to_string(),
        ),
        File::new(
            ".gitignore",
            r".cache
build"
                .to_string(),
        ),
    ]
}

pub fn get_source_files<'a>() -> Vec<File<'a>> {
    vec![File::new(
        "CMakeLists.txt",
        r"cmake_minimum_required(VERSION 3.20)

add_subdirectory(Main)"
            .to_string(),
    )]
}

pub fn get_main_mod_files<'a>(
    name: &str,
) -> Vec<File<'a>> {
    vec![File::new(
        "CMakeLists.txt",
        format!(
            "cmake_minimum_required(VERSION 3.20)
project({name})

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_STANDARD 20)

add_executable(${{PROJECT_NAME}}
    Private/main.cpp
)"
        ),
    )]
}

pub fn get_main_mod_private_files<'a>()
-> Vec<File<'a>> {
    vec![File::new(
            "main.cpp",
            "#include <iostream>

int main()
{
    std::operator<<(std::cout, \"Hello World!\\n\");
}"
                .to_string(),
        ),]
}
