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

install(TARGETS BindgenClangInterface DESTINATION lib)

# Gather dependencies for all libraries required by our target, recursively
function(target_link_libs_recursive out_libs target)
  get_target_property(imported ${target} IMPORTED)
  if (imported)
    get_target_property(target_link_libs ${target} INTERFACE_LINK_LIBRARIES)
  else()
    get_target_property(target_link_libs ${target} LINK_LIBRARIES)
  endif()
  if (target_link_libs)
    set(libs "")
    foreach(dep IN LISTS target_link_libs)
      list(FIND visited_targets ${dep} visited)
      if (${visited} EQUAL -1)
        list(APPEND visited_targets ${dep})
        list(APPEND libs ${dep})
        if (TARGET ${dep})
          set(dependencies "")
          target_link_libs_recursive(dependencies ${dep})
          list(APPEND libs ${dependencies})
        endif()
      endif()
    endforeach(dep)
    set(visited_targets ${visited_targets} PARENT_SCOPE)
    set(${out_libs} ${libs} PARENT_SCOPE)
  endif()
endfunction()

if (Clang_FOUND)
  target_link_libs_recursive(link_libs BindgenClangInterface)
  foreach(lib IN LISTS link_libs)
    string(REGEX MATCH "^-delayload" delayload ${lib})
    if (NOT delayload)
      message(STATUS "\ncargo:rustc-link-lib=" ${lib})
    endif()
  endforeach(lib)

  file(WRITE "${CMAKE_INSTALL_PREFIX}/BindgenClangInterface.deps" "${link_libs}")
else() # we didn't find clang cmake files
  file(REMOVE "${CMAKE_INSTALL_PREFIX}/BindgenClangInterface.deps")
endif()