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
  CXX_STANDARD 11
  CXX_EXTENSIONS OFF
  )


# Any changes to this list must be kept in sync with ../../build.rs in case
# cmake cannot find the clang cmake config files to generate a useful
# dependency list.
set(clang_libs
  clangAST
  clangFrontend
  clangIndex
  clangSema
  clangTooling
  clangBasic
  clangLex
  )

if (TARGET clang-cpp AND TARGET LLVM)
  target_link_libraries(BindgenClangInterface PRIVATE LLVM clang-cpp)
else()
  target_link_libraries(BindgenClangInterface PRIVATE ${clang_libs})
endif()

# rustc links with CC, so we need to explicitly include the C++ standard library
if (APPLE)
  target_link_libraries(BindgenClangInterface PRIVATE c++)
elseif(UNIX)
  target_link_libraries(BindgenClangInterface PRIVATE stdc++)
endif()


install(TARGETS BindgenClangInterface DESTINATION lib)

# Gather dependencies for all libraries required by our target, recursively
function(target_link_libs_recursive out_libs target)
  get_target_property(imported ${target} IMPORTED)
  if (imported)
    get_target_property(target_link_libs ${target} INTERFACE_LINK_LIBRARIES)
    if (NOT target_link_libs)
      get_target_property(target_config ${target} IMPORTED_CONFIGURATIONS)
      get_target_property(target_link_libs ${target} IMPORTED_LINK_DEPENDENT_LIBRARIES_${target_config})
    endif()
  else()
    get_target_property(target_link_libs ${target} LINK_LIBRARIES)
  endif()
  if (target_link_libs)
    set(libs "")
    foreach(dep IN LISTS target_link_libs)
      list(FIND visited_targets ${dep} visited)
      if (${visited} EQUAL -1)
        list(APPEND visited_targets ${dep})
        if (TARGET ${dep})
          set(dependencies "")
          target_link_libs_recursive(dependencies ${dep})
          list(APPEND libs ${dependencies})
        endif()
        list(APPEND libs ${dep})
      endif()
    endforeach(dep)
    set(visited_targets ${visited_targets} PARENT_SCOPE)
    set(${out_libs} ${libs} PARENT_SCOPE)
  endif()
endfunction()

if (Clang_FOUND)
  target_link_libs_recursive(link_libs BindgenClangInterface)

  file(WRITE "${CMAKE_INSTALL_PREFIX}/BindgenClangInterface.deps" "${link_libs}")
else() # we didn't find clang cmake files
  file(REMOVE "${CMAKE_INSTALL_PREFIX}/BindgenClangInterface.deps")
endif()
