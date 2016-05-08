cmake_minimum_required(VERSION 3.5)
project(patchgl)
set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH} "${CMAKE_SOURCE_DIR}/cmake/Modules/")
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++11")

set(SOURCE_FILES
        lib/RxCpp-2.3.0/Rx/v2/src
        src/patchgl/cpp/main.cpp
        src/patchgl/cpp/font.cpp src/patchgl/cpp/font.h
        src/patchgl/cpp/patch.cpp src/patchgl/cpp/patch.h
        src/patchgl/cpp/screen.cpp src/patchgl/cpp/screen.h
        src/patchgl/cpp/patchgl.pb.cc src/patchgl/cpp/patchgl.pb.h
        src/patchgl/cpp/charon.cpp src/patchgl/cpp/charon.h)

include_directories(/usr/local/include fern lib/RxCpp-2.3.0/Rx/v2/src)

add_subdirectory(fern)
add_subdirectory(fern_tests)
add_subdirectory(lib/RxCpp-2.3.0)

link_directories(/usr/local/lib)
add_executable(patchgl ${SOURCE_FILES})
find_package(GLFW REQUIRED)
find_package(OpenGL REQUIRED)
find_package(Freetype REQUIRED)
find_package(Protobuf REQUIRED)

include_directories(${GLFW_INCLUDE_DIR}
                    ${OPENGL_INCLUDE_DIR}
                    ${FREETYPE_INCLUDE_DIRS}
                    ${PROTOBUF_INCLUDE_DIRS})

target_link_libraries(patchgl
    ${GLFW_LIBRARIES}
    ${OPENGL_LIBRARIES}
    ${FREETYPE_LIBRARIES}
    ${PROTOBUF_LIBRARIES}
    fern ftgl)

install(TARGETS patchgl DESTINATION bin)
