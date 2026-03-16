use crate::file::File;

const BUILD_DIR_NAME: &str = "build";

pub fn get_root_files(name: &str) -> Vec<File> {
    vec![
        File::new(
            "CMakeLists.txt".to_string(),
            format!(
                r"cmake_minimum_required(VERSION 3.20)
project({name})

add_subdirectory(Source)
"
            ),
        ),
        File::new(
            ".clang-tidy".to_string(),
            "Checks: 'cppcoreguidelines-*'"
                .to_string(),
        ),
        File::new(
            ".clang-format".to_string(),
            r"---
BasedOnStyle: Microsoft
PointerAlignment: Left
ColumnLimit: 70"
                .to_string(),
        ),
        File::new(
            ".gitignore".to_string(),
            r".cache
build"
                .to_string(),
        ),
        File::new(
            "run.py".to_string(),
            format!(
                "from subprocess import run

run([\"cmake\", \".\", \"-B\", \"{BUILD_DIR_NAME}\"])
run([\"cmake\", \"--build\", \"{BUILD_DIR_NAME}\"])
run([\"./{BUILD_DIR_NAME}/Source/Main/{name}\"])"
            ),
        ),
    ]
}

pub fn get_source_files<'a>() -> Vec<File> {
    vec![File::new(
        "CMakeLists.txt".to_string(),
        r"cmake_minimum_required(VERSION 3.20)

add_subdirectory(Main)"
            .to_string(),
    )]
}

pub fn get_main_mod_files<'a>(
    name: &str,
) -> Vec<File> {
    vec![File::new(
        "CMakeLists.txt".to_string(),
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
-> Vec<File> {
    vec![File::new(
            "main.cpp".to_string(),
            "#include <iostream>

int main()
{
    std::operator<<(std::cout, \"Hello World!\\n\");
}"
                .to_string(),
        ),]
}
