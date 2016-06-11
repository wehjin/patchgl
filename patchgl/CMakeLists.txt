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
        src/patchgl/cpp/charon.cpp src/patchgl/cpp/charon.h
        src/patchgl/cpp/removable/Removable.h
        src/patchgl/cpp/removable/BooleanRemovable.cpp src/patchgl/cpp/removable/BooleanRemovable.h
        src/patchgl/cpp/gx/Display.h
        src/patchgl/cpp/gx/Shape.h
        src/patchgl/cpp/gx/Frame.h src/patchgl/cpp/gx/Argb.h src/patchgl/cpp/gx/GlfwDisplay.cpp src/patchgl/cpp/gx/GlfwDisplay.h src/patchgl/cpp/removable/RemovedRemovable.h)

include_directories(/usr/local/include fern lib/RxCpp-2.3.0/Rx/v2/src)

add_subdirectory(fern)
add_subdirectory(fern_tests)
add_subdirectory(lib/RxCpp-2.3.0)
add_subdirectory(lib/glfw-3.1.2)

link_directories(/usr/local/lib)
add_executable(patchgl ${SOURCE_FILES})
find_package(OpenGL REQUIRED)
find_package(Freetype REQUIRED)
find_package(Protobuf REQUIRED)

include_directories(${OPENGL_INCLUDE_DIR}
        ${FREETYPE_INCLUDE_DIRS}
        ${PROTOBUF_INCLUDE_DIRS})

target_link_libraries(patchgl
        ${OPENGL_LIBRARIES}
        ${FREETYPE_LIBRARIES}
        ${PROTOBUF_LIBRARIES}
        glfw3 fern ftgl)

install(TARGETS patchgl DESTINATION bin)
