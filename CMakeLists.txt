
cmake_minimum_required(VERSION 3.9.0)

project(localizer)

set(PLATFORM OS64)
set(ENABLE_BITCODE OFF)
set(Rust_CARGO_TARGET aarch64-apple-ios)
set(CMAKE_INTERPROCEDURAL_OPTIMIZATION_Release TRUE)

if (NOT DEFINED ENV{THEOS})
    message(FATAL_ERROR "Please check for your theos installation at https://theos.dev")
endif()

link_directories($ENV{THEOS}/vendor/lib)
include_directories($ENV{THEOS}/vendor/include)

include(FetchContent)

FetchContent_Declare(
  ios_cmake
  GIT_REPOSITORY https://github.com/leetal/ios-cmake
  GIT_TAG        master
)

FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.2.1
)

FetchContent_MakeAvailable(ios_cmake)
FetchContent_MakeAvailable(Corrosion)


include(${ios_cmake_SOURCE_DIR}/ios.toolchain.cmake)
corrosion_import_crate(MANIFEST_PATH localizer-rs/Cargo.toml)

add_custom_command(
    OUTPUT localizer.m
    COMMAND bash -c "$ENV{THEOS}/bin/logos.pl ${CMAKE_CURRENT_SOURCE_DIR}/src/localizer.x > localizer.m"
    COMMENT "Running logos..."
    VERBATIM
    DEPENDS ${CMAKE_CURRENT_SOURCE_DIR}/src/localizer.x
)

add_library(Localizer SHARED localizer.m)
target_link_libraries(Localizer PUBLIC
    localizer-rs
    substrate
    "-framework CoreFoundation"
    "-framework Foundation"
    "-framework Security"
    "-framework UIKit"
)

add_custom_command(
    TARGET Localizer
    POST_BUILD
    COMMAND strip -x $<TARGET_FILE:Localizer>
    COMMENT "Stripping..."
)
