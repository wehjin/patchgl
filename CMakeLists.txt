cmake_minimum_required(VERSION 3.5)
project(patchgl)

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++11")

set(SOURCE_FILES
        src/patchgl/cpp/main.cpp src/patchgl/cpp/patch.cpp src/patchgl/cpp/patch.h)

include_directories(/usr/local/include fern)

add_subdirectory(fern)
add_subdirectory(fern_tests)

link_directories(/usr/local/lib /System/Library/Frameworks/OpenGL.framework/Libraries)
add_executable(patchgl ${SOURCE_FILES})
target_link_libraries(patchgl glfw3 GL fern)
