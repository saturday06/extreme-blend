cmake_minimum_required(VERSION 3.10.2)
project(ultimate-desktop-linux C CXX)

include(CheckCXXCompilerFlag)

set(CMAKE_CXX_STANDARD 14)

find_package(Threads)
find_package(PkgConfig)

pkg_check_modules(PKG_CONFIG_FOUND REQUIRED wayland-server egl glesv2 x11 xcb x11-xcb wayland-scanner)

include_directories(${PKG_CONFIG_FOUND_INCLUDE_DIRS} ${CMAKE_CURRENT_BINARY_DIR}/external/include)
link_directories(${PKG_CONFIG_FOUND_LIBRARY_DIRS} ${CMAKE_CURRENT_BINARY_DIR}/external/lib)
link_libraries(${PKG_CONFIG_FOUND_LIBRARIES})

aux_source_directory(source sources)
aux_source_directory(test test_sources)
aux_source_directory(benchmark benchmark_sources)
aux_source_directory(include include_sources)

add_executable(compositor ${sources} ${include_sources})
add_executable(compositor_test ${sources} ${include_sources} ${test_sources})
add_executable(compositor_benchmark ${sources} ${include_sources} ${benchmark_sources})

target_link_libraries(compositor_test gtest gtest_main)
target_link_libraries(compositor_benchmark benchmark)

enable_testing()
add_test(test ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/${CMAKE_CFG_INTDIR}/compositor_test)

include(ExternalProject)

ExternalProject_Add(googletest
  URL https://github.com/google/googletest/archive/release-1.8.1.tar.gz
  URL_MD5 2e6fbeb6a91310a16efe181886c59596
  PREFIX external/src/googletest
  CMAKE_ARGS -DCMAKE_INSTALL_PREFIX=${CMAKE_CURRENT_BINARY_DIR}/external -DCMAKE_BUILD_TYPE=Debug
)
add_dependencies(compositor_test googletest)

ExternalProject_Add(benchmark
  URL https://github.com/google/benchmark/archive/v1.4.1.tar.gz
  URL_MD5 482dddb22bec43f5507a000456d6bb88
  PREFIX external/src/benchmark
  CMAKE_ARGS -DCMAKE_INSTALL_PREFIX=${CMAKE_CURRENT_BINARY_DIR}/external -DCMAKE_BUILD_TYPE=RelWithDebInfo
)
add_dependencies(compositor_benchmark benchmark)
