# Minimum version of cmake required
cmake_minimum_required(VERSION 2.8.0)

# Required Defines first:
set(BUILD_TYPE "Release")
set(RSMI_INC_DIR ${ROCM_DIR}/include)
set(RSMI_LIB_DIR ${ROCM_DIR}/lib)

message("")
message("Configuration init:")
message("-----------ROCM_DIR: " ${ROCM_DIR})
message("-----------RSMI: " ${RSMI})
message("-----------RSMI_ROOT: " ${RSMI_ROOT})
message("-----------RSMI_LIBS: " ${RSMI_LIBS})
message("-----------PROJECT_BINARY_DIR: " ${PROJECT_BINARY_DIR})
message("-----------RSMI_LIB_DIR: " ${RSMI_LIB_DIR})
message("-----------RSMI_INC_DIR: " ${RSMI_INC_DIR})
message("-----------RSMISources: " ${RSMISources})
message("-----------functionalSources: " ${functionalSources})
message("")

# Determine RSMI Library file is present
if("${RSMI_BLD_BITS}" STREQUAL 32)
  set (ONLY64STR "")
  set (IS64BIT 0)
else()
  set (ONLY64STR "64")
  set (IS64BIT 1)
endif()
#
if (${IS64BIT} EQUAL 0)
  if(NOT EXISTS ${RSMI_LIB_DIR}/librocm_smi32.so)
    message("ERROR: ${RSMI_LIB_DIR}/librocm_smi32.so doesn't exist. Check value of ROCM_DIR define")
    return()
  endif()
else()
  if(NOT EXISTS ${RSMI_LIB_DIR}/librocm_smi64.so)
    message("ERROR: Define RSMI_LIB_DIR pointing to RSMI library is not set")
    message(" missing: ${RSMI_LIB_DIR}/librocm_smi64.so")
    return()
  endif()
endif()

message("")
message("Configuration (after lib check):")
message("-----------ROCM_DIR: " ${ROCM_DIR})
message("-----------RSMI: " ${RSMI})
message("-----------RSMI_ROOT: " ${RSMI_ROOT})
message("-----------RSMI_LIBS: " ${RSMI_LIBS})
message("-----------PROJECT_BINARY_DIR: " ${PROJECT_BINARY_DIR})
message("-----------RSMI_LIB_DIR: " ${RSMI_LIB_DIR})
message("-----------RSMI_INC_DIR: " ${RSMI_INC_DIR})
message("-----------RSMISources: " ${RSMISources})
message("-----------functionalSources: " ${functionalSources})
message("")

message("")
message("Build Configuration:")
message("-------------IS64BIT: " ${IS64BIT})
message("-----------BuildType: " ${BUILD_TYPE})
message("------------Compiler: " ${CMAKE_CXX_COMPILER})
message("-------------Version: " ${CMAKE_CXX_COMPILER_VERSION})
message("--------Proj Src Dir: " ${PROJECT_SOURCE_DIR})
message("--------Proj Bld Dir: " ${PROJECT_BINARY_DIR})
message("--------Proj Lib Dir: " ${PROJECT_BINARY_DIR}/lib)
message("--------Proj Exe Dir: " ${PROJECT_BINARY_DIR}/bin)
message("--------RSMI Lib Dir: " ${RSMI_LIB_DIR})
message("")

# Set the build type based on user input
set(CMAKE_BUILD_TYPE ${BUILD_TYPE})

# Flag to enable / disable verbose output.
set(CMAKE_VERBOSE_MAKEFILE on)

add_definitions(-D__linux__)
add_definitions(-DLITTLEENDIAN_CPU=1)

# Linux Compiler options
set(CMAKE_CXX_FLAGS "-std=c++11 ")

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fexceptions")
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fno-rtti")
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fno-math-errno")
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fno-threadsafe-statics")
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fmerge-all-constants")
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fms-extensions")
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall")
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wextra")
# set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -pedantic")

## Address Sanitize Flag
if (${ADDRESS_SANITIZER})
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fsanitize=address -g")
    set(CMAKE_EXE_LINKER_FLAGS -fsanitize=address)

    if (BUILD_SHARED_LIBS})
        set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -shared-libsan" )
    else ()
        set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -static-libsan" )
    endif ()
endif()

# Extend the compiler flags for 64-bit builds
if (IS64BIT)
  set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -m64  -msse -msse2")
else()
  set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -m32")
endif()

# Add compiler flags to include symbol information for debug builds
if(ISDEBUG)
  set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -ggdb -O0")
endif()
MESSAGE("ISDEBUG STEP:Done")


set(RSMI_ROOT ${CMAKE_CURRENT_SOURCE_DIR})

# Other source directories
aux_source_directory(${RSMI_ROOT}/functional functionalSources)

# Specify the directory containing various libraries of ROCR
# to be linked against for building ROC Perf applications
LINK_DIRECTORIES(${RSMI_LIB_DIR} ${GTEST_LIB_DIR})

# Extend the list of libraries to be used for linking rsmi apps
if (IS64BIT)
  set(RSMI_LIBS ${RSMI_LIBS} rocm_smi64)
  set(RSMI "rsmi64")
else()
  set(RSMI_LIBS ${RSMI_LIBS} rocm_smi32)
  set(RSMI "rsmi")
endif()

# Source files for building rocrtst
aux_source_directory(${RSMI_ROOT} RSMISources)

# Header file include path
include_directories(${RSMI_INC_DIR})
include_directories(${RSMI_ROOT}/..)
include_directories(${RSMI_ROOT}/src)
include_directories(${RSMI_ROOT}/queries)

# Build rules
add_library(${RSMI} STATIC ${RSMISources} ${functionalSources})

target_link_libraries(${RSMI} ${RSMI_LIBS} c stdc++ pthread)

install(TARGETS ${RSMI}
        ARCHIVE DESTINATION ${PROJECT_BINARY_DIR}/lib
        LIBRARY DESTINATION ${PROJECT_BINARY_DIR}/lib
        RUNTIME DESTINATION ${PROJECT_BINARY_DIR}/bin)

message("")
message("Configuration: (Final)")
message("-----------ROCM_DIR: " ${ROCM_DIR})
message("-----------RSMI: " ${RSMI})
message("-----------RSMI_ROOT: " ${RSMI_ROOT})
message("-----------RSMI_LIBS: " ${RSMI_LIBS})
message("-----------PROJECT_BINARY_DIR: " ${PROJECT_BINARY_DIR})
message("-----------RSMI_LIB_DIR: " ${RSMI_LIB_DIR})
message("-----------RSMI_INC_DIR: " ${RSMI_INC_DIR})
message("-----------RSMISources: " ${RSMISources})
message("-----------functionalSources: " ${functionalSources})
message("")