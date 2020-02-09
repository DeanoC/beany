SET_MIN_VERSIONS()
set(LibName compute_sycl)

list(APPEND CMAKE_MODULE_PATH ${CMAKE_CURRENT_SOURCE_DIR}/cmake/Modules)
set(ComputeCpp_DIR "C:/Program Files/codeplay/computecpp")

# spir64, spirv64, ptx64
set(_COMPUTECPP_BACKEND "spir64")
set(COMPUTECPP_BITCODE "${_COMPUTECPP_BACKEND}" CACHE STRING "Bitcode type to use as SYCL target in compute++")
find_package(ComputeCpp REQUIRED)


project(${LibName} C CXX)
file(GLOB_RECURSE Src CONFIGURE_DEPENDS include/*.h include/*.hpp src/*.c src/*.cpp)
set(Deps
		al2o3_platform
		al2o3_memory
		)

add_compile_definitions( _COMPUTECPP_BACKEND COMPUTECPP_BACKEND_${_COMPUTECPP_BACKEND} )

ADD_LIB2(${LibName} "${Src}" "${Deps}")
add_sycl_to_target(TARGET ${LibName} SOURCES
		src/accelsycl.cpp
		)
install(TARGETS ${LibName} DESTINATION .)