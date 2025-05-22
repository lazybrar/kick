# Minimum CMake version required
cmake_minimum_required(VERSION 3.10)

# Project name and version
project({{projectName}} VERSION 1.0 LANGUAGES CXX)

# Set CXX standard
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Export Compile Commands for auto complete using clangd
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# Include directories
include_directories(${PROJECT_SOURCE_DIR}/include)

# Add executable
file(GLOB SOURCES "${PROJECT_SOURCE_DIR}/src/*.cpp")
add_executable("build" ${SOURCES})

target_link_libraries(build m)

# Output the executable to the build directory
set(EXECUTABLE_OUTPUT_PATH ${PROJECT_SOURCE_DIR}/bin)
