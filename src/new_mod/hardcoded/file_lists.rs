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

include(${{CMAKE_SOURCE_DIR}}/cmake/flags.cmake)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_STANDARD 20)

add_library(${{PROJECT_NAME}}
    ${{CMAKE_CURRENT_SOURCE_DIR}}/Private/{class_name}.cc
)

target_include_directories(${{PROJECT_NAME}} PUBLIC
    Public
)

target_link_libraries(${{PROJECT_NAME}} PUBLIC
    absl::base
    absl::statusor
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
        format!("{class_name}.cc"),
        format!(
            "#include \"{project_name}/{name}/{class_name}.h\"

namespace {project_name_namespace_name}::{name_namespace_name} {{}}"
        ),
    )]
}

pub fn get_public_files(
    mode_name: &str,
    project_name: &str,
) -> Vec<FileData> {
    let class_name = to_pascal_case(mode_name);
    let project_name_namespace_name =
        to_snake_case(project_name);
    let name_namespace_name =
        to_snake_case(mode_name);

    let uncased_include_guard =
        format!("{project_name}_{mode_name}_H_");
    let include_guard = to_screaming_snake_case(
        &uncased_include_guard,
    );

    vec![FileData::new(
        format!("{class_name}.h"),
        format!(
            "#ifndef {include_guard}
#define {include_guard}
namespace {project_name_namespace_name}::{name_namespace_name} {{
}} // namespace {project_name_namespace_name}::{name_namespace_name}
#endif // {include_guard}"
        ),
    ),
    FileData::new(format!("{class_name}Export.pch"), format!("#pragma once

#include \"{class_name}.h\""))]
}
