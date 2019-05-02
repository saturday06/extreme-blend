cmake_minimum_required(VERSION 3.10.2)
project(ultimate-desktop-linux C CXX)

include(CheckCXXCompilerFlag)

set(CMAKE_CXX_STANDARD 14)

if(NOT $ENV{PKG_CONFIG_PATH} STREQUAL "") # TODO: prettify
  set(pkg_config_path_leading_separator ":")
endif()
set(ENV{PKG_CONFIG_PATH} "$ENV{PKG_CONFIG_PATH}${pkg_config_path_leading_separator}${CMAKE_CURRENT_BINARY_DIR}/external/lib/pkgconfig")

find_package(Threads)
find_package(PkgConfig)

pkg_check_modules(PKG_CONFIG_FOUND REQUIRED wayland-server egl glesv2 x11 xcb x11-xcb wayland-scanner)
execute_process(
  COMMAND ${PKG_CONFIG_EXECUTABLE} wayland-scanner --variable wayland_scanner OUTPUT_VARIABLE wayland_scanner_executable
  OUTPUT_STRIP_TRAILING_WHITESPACE
)

include_directories(${PKG_CONFIG_FOUND_INCLUDE_DIRS} ${CMAKE_CURRENT_BINARY_DIR}/external/include)
link_directories(${PKG_CONFIG_FOUND_LIBRARY_DIRS} ${CMAKE_CURRENT_BINARY_DIR}/external/lib)
link_libraries(${PKG_CONFIG_FOUND_LIBRARIES})

aux_source_directory(source sources)
aux_source_directory(external/source external_sources)
aux_source_directory(test test_sources)
aux_source_directory(benchmark benchmark_sources)
aux_source_directory(include include_sources)

set(
  compositor_sources
  ${sources}
  ${external_sources}
  ${include_sources}
  external/include/wayland-server-header.h
  external/source/wayland-code.c
  external/include/xdg-shell-unstable-v6-server-header.h
  external/source/xdg-shell-unstable-v6-code.c
)

add_executable(compositor ${compositor_sources})
add_executable(compositor_test ${compositor_sources} ${test_sources})
add_executable(compositor_benchmark ${compositor_sources} ${benchmark_sources})

target_compile_definitions(compositor_test PUBLIC CUSTOM_MAIN)
target_compile_definitions(compositor_benchmark PUBLIC CUSTOM_MAIN)

target_link_libraries(compositor_test gtest gtest_main)
target_link_libraries(compositor_benchmark gtest gtest_main benchmark)

enable_testing()
add_test(test ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/${CMAKE_CFG_INTDIR}/compositor_test)

include(ExternalProject)

ExternalProject_Add(
  external-googletest
  URL https://github.com/google/googletest/archive/release-1.8.1.tar.gz
  URL_MD5 2e6fbeb6a91310a16efe181886c59596
  PREFIX external/build/googletest
  CMAKE_ARGS -DCMAKE_INSTALL_PREFIX=${CMAKE_CURRENT_BINARY_DIR}/external -DCMAKE_BUILD_TYPE=RelWithDebInfo
)
add_dependencies(compositor_test external-googletest)

ExternalProject_Add(
  external-benchmark
  DEPENDS external-googletest
  URL https://github.com/google/benchmark/archive/v1.4.1.tar.gz
  URL_MD5 482dddb22bec43f5507a000456d6bb88
  PREFIX external/build/benchmark
  CMAKE_ARGS -DCMAKE_INSTALL_PREFIX=${CMAKE_CURRENT_BINARY_DIR}/external -DCMAKE_BUILD_TYPE=RelWithDebInfo
)
add_dependencies(compositor_benchmark external-benchmark)

add_custom_command(
  OUTPUT external/include/wayland-server-header.h
  DEPENDS external-directory /usr/share/wayland/wayland.xml
  COMMAND ${wayland_scanner_executable} server-header /usr/share/wayland/wayland.xml external/include/wayland-server-header.h
)

add_custom_command(
  OUTPUT external/source/wayland-code.c
  DEPENDS external-directory /usr/share/wayland/wayland.xml
  COMMAND ${wayland_scanner_executable} code /usr/share/wayland/wayland.xml external/source/wayland-code.c
)

add_custom_command(
  OUTPUT external/include/xdg-shell-unstable-v6-server-header.h
  DEPENDS external-directory /usr/share/wayland-protocols/unstable/xdg-shell/xdg-shell-unstable-v6.xml
  COMMAND ${wayland_scanner_executable} server-header /usr/share/wayland-protocols/unstable/xdg-shell/xdg-shell-unstable-v6.xml external/include/xdg-shell-unstable-v6-server-header.h
)

add_custom_command(
  OUTPUT external/source/xdg-shell-unstable-v6-code.c
  DEPENDS external-directory /usr/share/wayland-protocols/unstable/xdg-shell/xdg-shell-unstable-v6.xml
  COMMAND ${wayland_scanner_executable} code /usr/share/wayland-protocols/unstable/xdg-shell/xdg-shell-unstable-v6.xml external/source/xdg-shell-unstable-v6-code.c
)

add_custom_target(external-directory ALL COMMAND ${CMAKE_COMMAND} -E make_directory external/include external/source)
