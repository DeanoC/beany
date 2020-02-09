// License Summary: MIT see LICENSE file
#pragma once

typedef struct Cuda Cuda;

AL2O3_EXTERN_C Cuda* AccelCUDA_Create();
AL2O3_EXTERN_C void AccelCUDA_Destroy(Cuda* cuda);