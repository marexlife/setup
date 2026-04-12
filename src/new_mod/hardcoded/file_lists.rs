use crate::{
    file::FileData,
    utils::to_screaming_snake_case,
};

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

target_include_directories(${{PROJECT_NAME}} PRIVATE
    Private
)

target_include_directories(${{PROJECT_NAME}} PUBLIC
    Public
)

target_precompile_headers(${{PROJECT_NAME}} PUBLIC
    ${{CUSTOM_HEADER_PATH}}/{name}.h
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
    let header_guard =
        to_screaming_snake_case(format!(
            "{project_name}_{name}_{name}_H_",
        ));

    vec![FileData::new(
        format!("{name}.h"),
        format!(
            "#ifndef {header_guard}
#define {header_guard}
        
namespace {project_name}::{name}
{{
class {name} final
{{
  public:
    explicit {name}() = delete;
    ~{name}() = delete;
    {name}& operator=(const {name}&) = delete;
    {name}& operator=({name}&&) = delete;
    {name}(const {name}&) = delete;
    {name}({name}&&) = delete;
}};
}} // namespace {project_name}::{name}
#endif // {header_guard}",
        ),
    )]
}
