use crate::{
    file_data::FileData,
    utils::{
        to_pascal_case, to_screaming_snake_case,
        to_snake_case,
    },
};

pub fn get_mod_root_files(
    name: &str,
    project_name: &str,
) -> Vec<FileData> {
    let class_name = to_pascal_case(name);

    let cmake_var_name_part =
        to_screaming_snake_case(project_name);

    vec![FileData::new(
        "CMakeLists.txt".to_string(),
        format!(
            "cmake_minimum_required(VERSION 3.20)
project({name})

include(${{CMAKE_SOURCE_DIR}}/CMake/Flags.cmake)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_STANDARD 20)

add_library(${{PROJECT_NAME}}
    ${{CMAKE_CURRENT_SOURCE_DIR}}/Private/{class_name}.cpp
)

target_precompile_headers(${{PROJECT_NAME}} PUBLIC
    ${{CMAKE_CURRENT_SOURCE_DIR}}/Public/{project_name}/{name}/{class_name}Export.pch
)

target_include_directories(${{PROJECT_NAME}} PUBLIC
    Public
)

target_link_libraries(${{PROJECT_NAME}} PUBLIC
    absl::base
)

target_compile_options(${{PROJECT_NAME}} PUBLIC
    ${{{cmake_var_name_part}_COMPILER_FLAGS}}
)"
        ),
    )]
}

pub fn get_private_files(
    name: &str,
    project_name: &str,
) -> Vec<FileData> {
    let class_name = to_pascal_case(name);
    let project_name_namespace_name =
        to_snake_case(project_name);
    let name_namespace_name = to_snake_case(name);

    vec![FileData::new(
        format!("{class_name}.cpp"),
        format!(
            "#include \"{project_name}/{name}/{class_name}.h\"

namespace {project_name_namespace_name}::{name_namespace_name} {{}}"
        ),
    )]
}

pub fn get_public_files(
    name: &str,
    project_name: &str,
) -> Vec<FileData> {
    let class_name = to_pascal_case(name);
    let project_name_namespace_name =
        to_snake_case(project_name);
    let name_namespace_name = to_snake_case(name);

    vec![FileData::new(
        format!("{class_name}.h"),
        format!(
            "#pragma once

namespace {project_name_namespace_name}::{name_namespace_name}
{{
class {class_name} final
{{
  public:
    explicit {class_name}() = default;
    ~{class_name}() = default;

    {class_name}& operator=(const {class_name}&) = delete;
    {class_name}& operator=({class_name}&&) = delete;
    {class_name}(const {class_name}&) = delete;
    {class_name}({class_name}&&) = delete;
}};
}} // namespace {project_name_namespace_name}::{name_namespace_name}"
        ),
    ),
    FileData::new(format!("{class_name}Export.pch"), format!("#pragma once

#include \"{class_name}.h\""))]
}
