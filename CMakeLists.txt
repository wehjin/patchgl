cmake_minimum_required(VERSION 3.5)
project(patchgl)

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++11")

set(SOURCE_FILES
        src/patchgl/cpp/main.cpp)

INCLUDE_DIRECTORIES(/usr/local/include)
LINK_DIRECTORIES(/usr/local/lib /System/Library/Frameworks/OpenGL.framework/Libraries)

add_executable(patchgl ${SOURCE_FILES})
TARGET_LINK_LIBRARIES(patchgl glfw3 GL)
