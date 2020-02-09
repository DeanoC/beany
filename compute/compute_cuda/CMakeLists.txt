SET_MIN_VERSIONS()
set(LibName compute_cuda)
project(${LibName} C CXX CUDA)
file(GLOB_RECURSE Src CONFIGURE_DEPENDS include/*.h include/*.hpp src/*.c src/*.cpp src/*.cu)

set(Deps
		al2o3_platform
		al2o3_memory
		)
ADD_LIB2(${LibName} "${Src}" "${Deps}")
