cmake_minimum_required(VERSION 3.12.4)

option(unittests "unittests" OFF)
set_property(GLOBAL PROPERTY GLOBAL_FETCHDEPS_BASE ${CMAKE_CURRENT_SOURCE_DIR}/al2o3 )
include(FetchContent)
FetchContent_Declare( al2o3 GIT_REPOSITORY https://github.com/DeanoC/al2o3 GIT_TAG master )
FetchContent_GetProperties(al2o3)
if(NOT al2o3_POPULATED)
	FetchContent_Populate(al2o3)
	add_subdirectory(${al2o3_SOURCE_DIR} ${al2o3_BINARY_DIR})
endif()
INIT_AL2O3(${CMAKE_CURRENT_SOURCE_DIR})

project(compute C CXX CUDA)

add_subdirectory(compute_cuda)
#add_subdirectory(compute_sycl)