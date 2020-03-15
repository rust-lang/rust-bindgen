cmake_minimum_required(VERSION 3.4.3)
project(BindgenClangInterface)

set(SRCS
  clang_interface.cpp
  libclang_compat.cpp
  )

if( PROJECT_NAME STREQUAL "LLVM" )
  # We are building in-tree, we can use LLVM cmake functions

  add_definitions(-DCLANG_BIN_PATH="${CMAKE_INSTALL_PREFIX}/bin")

  add_clang_library(BindgenClangInterface ${SRCS} DEPENDS clang-headers)

  set(LLVM_LINK_COMPONENTS support)
else()
  find_package(LLVM REQUIRED CONFIG)

  # Debian and Ubuntu's clang cmake files are broken, so we can't require the
  # package here. We already have to manually order the link against the clang
  # libs in build.rs, so that's not so bad.
  find_package(Clang CONFIG)

  include_directories(${LLVM_INCLUDE_DIRS} ${CLANG_INCLUDE_DIRS})
  add_definitions(${LLVM_DEFINITIONS} ${CLANG_DEFINITIONS})

  if (DEFINED CLANG_INSTALL_PREFIX)
    add_definitions(-DCLANG_BIN_PATH="${CLANG_INSTALL_PREFIX}/bin")
  elseif(DEFINED LLVM_INSTALL_PREFIX)
    add_definitions(-DCLANG_BIN_PATH="${LLVM_INSTALL_PREFIX}/bin")
  elseif(DEFINED LLVM_TOOLS_BINARY_DIR)
    add_definitions(-DCLANG_BIN_PATH="${LLVM_TOOLS_BINARY_DIR}")
  else()
    message(FATAL_ERROR "Cannot find path to clang binary")
  endif()

  set(LLVM_LINK_COMPONENTS support)

  # LLVM is not always built with RTTI, we don't need it either.
  set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fno-rtti")

  # The library
  add_library(BindgenClangInterface STATIC ${SRCS})
endif()

# set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fsanitize=address")


add_definitions(-DCLANG_LIBDIR_SUFFIX="${LLVM_LIBDIR_SUFFIX}")

set_target_properties(BindgenClangInterface PROPERTIES
  CXX_STANDARD 14
  CXX_EXTENSIONS OFF
  )

# PRIVATE was added to make BindgenClangInterface build with LLVM 6.0. Keyword
# description: https://cmake.org/pipermail/cmake/2016-May/063400.html
target_link_libraries(BindgenClangInterface PRIVATE
  clangAST
  clangFrontend
  clangIndex
  clangSema
  clangTooling
  clangBasic
  )
