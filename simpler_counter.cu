#include <stdio.h>
#include <unistd.h>
#include <cuda_runtime.h>

__device__ int counter = 100;

__global__ void increment()
{
    counter++;
}

void checkCuda(cudaError_t result, const char *msg) {
    if (result != cudaSuccess) {
        fprintf(stderr, "CUDA Error: %s - %s\n", msg, cudaGetErrorString(result));
        exit(1);
    }
}

int main(void)
{
    // Initialize CUDA
    checkCuda(cudaFree(0), "Initializing CUDA");

    // Initialize counter to 100 on the device
    int initialCounter = 100;
    checkCuda(cudaMemcpyToSymbol(counter, &initialCounter, sizeof(int)), "Initializing counter");

    while (true) {
        int hCounter = 0;

        // Launch the increment kernel
        increment<<<1, 1>>>();
        checkCuda(cudaDeviceSynchronize(), "Kernel execution");

        // Copy the counter from device to host
        checkCuda(cudaMemcpyFromSymbol(&hCounter, counter, sizeof(counter)), "Copying counter to host");

        // Print the current counter value
        printf("%d\n", hCounter);

        // Wait for 1 second
        sleep(1);
    }

    return 0;
}

