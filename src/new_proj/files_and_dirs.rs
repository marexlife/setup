use crate::file_data::FileData;
use crate::utils::to_screaming_snake_case;
use crate::{
    shared,
    utils::{
        create_and_get_directory, create_files,
        create_sub_directory_and_files,
    },
};

const SOURCE_DIR_NAME: &str = "Source";
const THIRD_PARTY_DIR_NAME: &str = "ThirdParty";
const MAIN_MOD_DIR_NAME: &str = "Main";
const CMAKE_DIR_NAME: &str = "CMake";

#[must_use]
pub(crate) fn create_root_directory_and_files(
    name: &str,
) -> String {
    let path = create_and_get_directory(
        name.to_string(),
    );

    create_files(
        &name,
            vec![
        FileData::new(
            "CMakeLists.txt".to_string(),
            format!(
                r"cmake_minimum_required(VERSION 3.20)
project({name})

set(CMAKE_BUILD_TYPE Release)

add_subdirectory(Source)
add_subdirectory(ThirdParty)
"
            ),
        ),
        FileData::new(
            ".clang-tidy".to_string(),
"Checks: \"cppcoreguidelines-*,readability-*,modernization-*,performance-*\"
CheckOptions:
  - key: \"readability-identifier-naming.DefaultCase\"
    value: camelBack
  - key: \"readability-identifier-naming.ClassCase\"
    value: CamelCase
  - key: \"readability-identifier-naming.NamespaceCase\"
    value: lower_case
"
                .to_string(),
        ),
        FileData::new(
            ".clang-format".to_string(),
            r"---
BasedOnStyle: Google
PointerAlignment: Left
ColumnLimit: 70
IdentWidth: 4"
                .to_string(),
        ),
        FileData::new(
            ".gitignore".to_string(),
            r"/build
/.cache"
                .to_string(),
        ),
    ],
    );

    path
}

#[must_use]
pub(crate) fn create_third_party_directory_and_files(
    parent: &str,
) -> String {
    let dir = create_sub_directory_and_files(
        parent,
        THIRD_PARTY_DIR_NAME,
        vec![FileData::new(
            "CMakeLists.txt".to_string(),
            format!("cmake_minimum_required(VERSION 3.20)
project({THIRD_PARTY_DIR_NAME})

include(${{CMAKE_CURRENT_SOURCE_DIR}}/CMake/Dependencies.cmake)

Dependencies_Pull()
"),
        )],
    );

    let dir = create_sub_directory_and_files(
        &dir,
        CMAKE_DIR_NAME,
        vec![FileData::new(
            "Dependencies.cmake".to_string(),
            "cmake_minimum_required(VERSION 3.20)
project(Dependencies)

include(FetchContent)

function(Dependencies_Pull)
    FetchContent_Declare(ABSL
       GIT_REPOSITORY https://github.com/abseil/abseil-cpp.git
       GIT_TAG lts_2026_01_07
    )

   FetchContent_Declare(SPDLOG
       GIT_REPOSITORY https://github.com/gabime/spdlog.git
       GIT_TAG v1.17.0
    )

    FetchContent_MakeAvailable(ABSL SPDLOG)
endfunction()"
                .to_string(),
        )],
    );

    dir
}

#[must_use]
pub(crate) fn create_cmake_directory_and_files(
    parent: &str,
) -> String {
    let cmake_var_name_part =
        to_screaming_snake_case(
            parent.to_string(),
        );

    create_sub_directory_and_files(
        parent,
        CMAKE_DIR_NAME,
        vec![FileData::new(
            "Flags.cmake".to_string(),
            format!("cmake_minimum_required(VERSION 3.20)

set({cmake_var_name_part}_COMPILER_FLAGS)"
        ),
        )],
    )
}

#[must_use]
pub(crate) fn create_source_directory_and_files(
    parent: &str,
) -> String {
    create_sub_directory_and_files(
        parent,
        SOURCE_DIR_NAME,
        vec![FileData::new(
        "CMakeLists.txt".to_string(),
        r"cmake_minimum_required(VERSION 3.20)

add_subdirectory(Main)"
            .to_string(),
    )],
    )
}

#[must_use]
pub(crate) fn mod_directory_and_files(
    project_name: &str,
    parent: &str,
) -> String {
    create_sub_directory_and_files(
        parent,
        MAIN_MOD_DIR_NAME,
    vec![FileData::new(
        "CMakeLists.txt".to_string(),
        format!(
            "cmake_minimum_required(VERSION 3.20)
project({project_name})

set(CUSTOM_SOURCE_PATH ${{CMAKE_CURRENT_SOURCE_DIR}}/Private)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_STANDARD 20)

add_executable(${{PROJECT_NAME}}
    ${{CUSTOM_SOURCE_PATH}}/main.cpp
)
    
target_link_libraries(${{PROJECT_NAME}} PUBLIC
    spdlog
    absl::base
)"
        ),
    )],
    )
}

#[must_use]
pub(crate) fn create_private_directory_and_files(
    parent: &str,
) -> String {
    shared::create_private_and_files(
        parent,
        vec![FileData::new(
            "main.cpp".to_string(),
            "#include \"spdlog/spdlog.h\"

int main() { spdlog::info(\"Hello World!\"); }"
                .to_string(),
        )],
    )
}
