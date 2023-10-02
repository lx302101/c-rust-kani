cmake_minimum_required(VERSION 3.19)

project(C-Rust-Kani)

# ---------- Local CMake Modules ----------- #
set(CMAKE_MODULE_PATH ${CMAKE_SOURCE_DIR}/cmake)


# ---------- Install Corrosion ----------- #
include(FetchContent)
FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.3.5 # fixed tag, update manually
)

# Toolchain set for seahorn
set(Rust_TOOLCHAIN nightly-2022-08-01)

# Set any global configuration variables such as `Rust_TOOLCHAIN` before this line!
FetchContent_MakeAvailable(Corrosion)

include(CTest)
enable_testing()

# configure_file(${CMAKE_CURRENT_SOURCE_DIR}/cmake/extract_tests.cmake.in extract_tests.cmake @ONLY)
# set(EXTRACT_TEST_CMD ${CMAKE_CURRENT_BINARY_DIR}/extract_tests.cmake)

include_directories(BEFORE ${CMAKE_CURRENT_BINARY_DIR}/include)

add_custom_target(
    kani_build
    COMMAND cargo kani
)

add_subdirectory(src)
