#include <stdio.h>
#include "al2o3_platform/platform.h"
#include "accelcuda.h"

void testcall(float value)
{
	printf("Hello, world from C! Value passed: %f\n",value);
	Cuda* cuda = AccelCUDA_Create();
	AccelCUDA_Destroy(cuda);
}