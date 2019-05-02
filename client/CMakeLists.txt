cmake_minimum_required(VERSION 3.10.2)
project(ultimate-desktop-client C CXX)

include(CheckCXXCompilerFlag)

set(CMAKE_CXX_STANDARD 14)

if(NOT $ENV{PKG_CONFIG_PATH} STREQUAL "") # TODO: prettify
  set(pkg_config_path_leading_separator ":")
endif()
set(ENV{PKG_CONFIG_PATH} "$ENV{PKG_CONFIG_PATH}${pkg_config_path_leading_separator}${CMAKE_CURRENT_BINARY_DIR}/external/lib/pkgconfig")

find_package(Threads)
find_package(PkgConfig)

pkg_check_modules(PKG_CONFIG_FOUND REQUIRED egl glesv2 x11 xcb x11-xcb wayland-client)

include_directories(${PKG_CONFIG_FOUND_INCLUDE_DIRS} ${CMAKE_CURRENT_BINARY_DIR}/external/include)
link_directories(${PKG_CONFIG_FOUND_LIBRARY_DIRS} ${CMAKE_CURRENT_BINARY_DIR}/external/lib)
link_libraries(${PKG_CONFIG_FOUND_LIBRARIES})

aux_source_directory(source sources)
aux_source_directory(external/source external_sources)
aux_source_directory(include include_sources)
aux_source_directory(test test_sources)
aux_source_directory(benchmark benchmark_sources)

set(
  client_sources
  ${sources}
  ${external_sources}
  ${include_sources}
)

add_executable(client ${client_sources})
add_executable(client_test ${client_sources} ${test_sources})
add_executable(client_benchmark ${client_sources} ${benchmark_sources})

target_compile_definitions(client_test PUBLIC ULTIMATE_DESKTOP_CLIENT_CUSTOM_MAIN)
target_compile_definitions(client_benchmark PUBLIC ULTIMATE_DESKTOP_CLIENT_CUSTOM_MAIN)

target_link_libraries(client_test gtest gtest_main)
target_link_libraries(client_benchmark gtest gtest_main benchmark)

enable_testing()
add_test(test ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/${CMAKE_CFG_INTDIR}/client_test)

include(ExternalProject)

ExternalProject_Add(
  external-googletest
  URL https://github.com/google/googletest/archive/release-1.8.1.tar.gz
  URL_MD5 2e6fbeb6a91310a16efe181886c59596
  PREFIX external/build/googletest
  CMAKE_ARGS -DCMAKE_INSTALL_PREFIX=${CMAKE_CURRENT_BINARY_DIR}/external -DCMAKE_BUILD_TYPE=RelWithDebInfo
)
add_dependencies(client_test external-googletest)

ExternalProject_Add(
  external-benchmark
  DEPENDS external-googletest
  URL https://github.com/google/benchmark/archive/v1.4.1.tar.gz
  URL_MD5 482dddb22bec43f5507a000456d6bb88
  PREFIX external/build/benchmark
  CMAKE_ARGS -DCMAKE_INSTALL_PREFIX=${CMAKE_CURRENT_BINARY_DIR}/external -DCMAKE_BUILD_TYPE=RelWithDebInfo
)
add_dependencies(client_benchmark external-benchmark)
