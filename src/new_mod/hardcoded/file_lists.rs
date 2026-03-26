use crate::file::FileData;

pub fn get_mod_root_files<'a>(
    name: &str,
) -> Vec<FileData> {
    vec![FileData::new(
        "CMakeLists.txt".to_string(),
        format!(
            "cmake_minimum_required(VERSION 3.20)
project({name})

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_STANDARD 20)

add_library(${{PROJECT_NAME}}
    Private/{name}.cpp
)

target_include_directories(${{PROJECT_NAME}} PUBLIC
    Public
)"
        ),
    )]
}

pub fn get_private_files<'a>(
    name: &'a str,
    project_name: &'a str,
) -> Vec<FileData> {
    vec![FileData::new(
        format!("{name}.cpp"),
        format!(
            "#include \"{name}/{name}.h\"

namespace {project_name}
{{
namespace {name}
{{
}} // namespace {name}
}} // namespace {project_name}"
        ),
    )]
}

pub fn get_public_files<'a>(
    name: &'a str,
    project_name: &'a str,
) -> Vec<FileData> {
    vec![FileData::new(
        format!("{name}.h"),
        format!(
            "#pragma once
        
namespace {project_name}
{{
namespace {name}
{{
class {name} 
{{
  public:
    explicit {name}() = default;
    virtual ~{name}() = default;

    {name}& operator=(const {name}&) = delete;
    {name}& operator=({name}&&) = delete;
    {name}(const {name}&) = delete;
    {name}({name}&&) = delete;
}};
}} // namespace {name}
}} // namespace {project_name}"
        ),
    )]
}
