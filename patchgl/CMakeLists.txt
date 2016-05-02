cmake_minimum_required(VERSION 3.5)
project(patchgl)

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++11")

set(SOURCE_FILES
        src/patchgl/cpp/main.cpp
        src/patchgl/cpp/patch.cpp src/patchgl/cpp/patch.h
        src/patchgl/cpp/screen.cpp src/patchgl/cpp/screen.h
        src/patchgl/cpp/patchgl.pb.cc src/patchgl/cpp/patchgl.pb.h
        lib/RxCpp-2.3.0/Rx/v2/src src/patchgl/cpp/charon.cpp src/patchgl/cpp/charon.h)

include_directories(/usr/local/include fern lib/RxCpp-2.3.0/Rx/v2/src)

add_subdirectory(fern)
add_subdirectory(fern_tests)
add_subdirectory(lib/RxCpp-2.3.0)

link_directories(/usr/local/lib /System/Library/Frameworks/OpenGL.framework/Libraries)
add_executable(patchgl ${SOURCE_FILES})
target_link_libraries(patchgl glfw3 GL fern protobuf)

install(TARGETS patchgl DESTINATION bin)
