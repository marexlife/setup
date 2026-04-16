use crate::file_data::FileData;

pub fn get_mod_root_files(
    name: &str,
    project_name: &str,
) -> Vec<FileData> {
    vec![FileData::new(
        "CMakeLists.txt".to_string(),
        format!(
            "cmake_minimum_required(VERSION 3.20)
project({name})

set(CUSTOM_HEADER_PATH ${{CMAKE_CURRENT_SOURCE_DIR}}/Public/{project_name}/{name})
set(CUSTOM_SOURCE_PATH ${{CMAKE_CURRENT_SOURCE_DIR}}/Private)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_STANDARD 20)

add_library(${{PROJECT_NAME}}
    ${{CUSTOM_SOURCE_PATH}}/{name}.cpp
)

target_precompile_headers(${{PROJECT_NAME}} PUBLIC
    ${{CUSTOM_HEADER_PATH}}/{name}Export.pch
)

target_include_directories(${{PROJECT_NAME}} PUBLIC
    Public
)
    
target_link_libraries(${{PROJECT_NAME}} PUBLIC
    spdlog
    absl::base
)"
        ),
    )]
}

pub fn get_private_files(
    name: &str,
    project_name: &str,
) -> Vec<FileData> {
    vec![FileData::new(
        format!("{name}.cpp"),
        format!(
            "#include \"{project_name}/{name}/{name}.h\"

namespace {project_name}::{name}
{{
}} // namespace {project_name}::{name}"
        ),
    )]
}

pub fn get_public_files(
    name: &str,
    project_name: &str,
) -> Vec<FileData> {
    let class_name = format!("C{name}");

    vec![FileData::new(
        format!("{class_name}.h"),
        format!(
            "#pragma once

namespace {project_name}::{name}
{{
class {class_name} final
{{
  public:
    explicit {class_name}() = delete;
    ~{class_name}() = delete;

    {class_name}& operator=(const {class_name}&) = delete;
    {class_name}& operator=({class_name}&&) = delete;
    {class_name}(const {class_name}&) = delete;
    {class_name}({class_name}&&) = delete;
}};
}} // namespace {project_name}::{name}"
        ),
    ),
    FileData::new(format!("{class_name}Export.pch"), format!("#pragma once
#include \"{project_name}.h\""))]
}
