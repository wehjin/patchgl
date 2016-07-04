cmake_minimum_required(VERSION 3.5)
project(patchgl)
set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH} "${CMAKE_SOURCE_DIR}/cmake/Modules/")
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++11")

set(SOURCE_FILES
        lib/RxCpp-2.3.0/Rx/v2/src
        src/patchgl/cpp/main.cpp
        src/patchgl/cpp/font.cpp src/patchgl/cpp/font.h
        src/patchgl/cpp/patch.cpp src/patchgl/cpp/patch.h
        src/patchgl/cpp/data/patchgl.pb.cc src/patchgl/cpp/data/patchgl.pb.h
        src/patchgl/cpp/charon.cpp src/patchgl/cpp/charon.h
        src/patchgl/cpp/removable/Removable.h
        src/patchgl/cpp/removable/BooleanRemovable.cpp src/patchgl/cpp/removable/BooleanRemovable.h
        src/patchgl/cpp/gx/Display.h
        src/patchgl/cpp/gx/Shape.h
        src/patchgl/cpp/gx/Frame.h
        src/patchgl/cpp/gx/Argb.h
        src/patchgl/cpp/gx/GlfwDisplay.cpp src/patchgl/cpp/gx/GlfwDisplay.h
        src/patchgl/cpp/removable/EmptyRemovable.h
        src/patchgl/cpp/gx/ShiftDisplay.cpp src/patchgl/cpp/gx/ShiftDisplay.h
        src/patchgl/cpp/gx/Shader.h
        src/patchgl/cpp/gx/Scribe.cpp src/patchgl/cpp/gx/Scribe.h
        src/patchgl/cpp/gx/Span.cpp src/patchgl/cpp/gx/Span.h
        )

include_directories(
        fern
)

add_subdirectory(fern)
add_subdirectory(lib/RxCpp-2.3.0)
add_subdirectory(lib/freetype-2.6.3)
add_subdirectory(lib/glm)
add_executable(patchgl ${SOURCE_FILES})
find_package(OpenGL REQUIRED)
find_package(GLFW REQUIRED)
find_package(Protobuf REQUIRED)
find_package(GLEW REQUIRED)

include_directories(
        ${OPENGL_INCLUDE_DIR}
        ${GLFW_INCLUDE_DIR}
        ${PROTOBUF_INCLUDE_DIR}
        ${GLEW_INCLUDE_DIR}
        lib/freetype-2.6.3/include
        lib/glm
        lib/RxCpp-2.3.0/Rx/v2/src
)

target_link_libraries(
        patchgl
        ${OPENGL_LIBRARIES}
        ${GLFW_LIBRARIES}
        ${PROTOBUF_LIBRARIES}
        ${GLEW_LIBRARIES}
        freetype fern
)

install(TARGETS patchgl DESTINATION bin)
