#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <omp.h>

#define L 200

#define MIN_RAND -10
#define MAX_RAND 10

int array[L];  // single 1D array with size L

// Function declarations
void fill(int *array, int size);
int sum_array(int *array, int size);
int sum_array_parallel_omp(int *array, int size);

int main() {
    srand(time(NULL));
    
    fill(array, L);      // fill array with random numbers

    // Sequential sum timing
    double start = omp_get_wtime();
    int result_seq = sum_array(array, L);
    double end = omp_get_wtime();
    printf("Sum of array values (sequential): %d, time: %f seconds\n", result_seq, end - start);

    // Parallel atomic sum timing
    start = omp_get_wtime();
    int result_parallel = sum_array_parallel_omp(array, L);
    end = omp_get_wtime();
    printf("Sum of array values (parallel OpenMP atomic): %d, time: %f seconds\n", result_parallel, end - start);

    // Parallel for reduction sum timing
    start = omp_get_wtime();
    int result_reduction = sum_array_parallel_omp_reduction_for(array, L);
    end = omp_get_wtime();
    printf("Sum of array values (parallel OpenMP for with reduction): %d, time: %f seconds\n", result_reduction, end - start);

    // Parallel tasks with taskgroup reduction sum timing
    start = omp_get_wtime();
    int result_taskgroup = sum_array_parallel_omp_taskgroup(array, L);
    end = omp_get_wtime();
    printf("Sum of array values (parallel OpenMP tasks with taskgroup reduction): %d, time: %f seconds\n", result_taskgroup, end - start);

    // Parallel taskloop with reduction sum timing
    start = omp_get_wtime();
    int result_taskloop = sum_array_parallel_omp_taskloop(array, L);
    end = omp_get_wtime();
    printf("Sum of array values (parallel OpenMP taskloop with reduction): %d, time: %f seconds\n", result_taskloop, end - start);

    return 0;
}

// Fills the array with random values between MIN_RAND and MAX_RAND
void fill(int *array, int size) {
    for (int i = 0; i < size; i++) {
        array[i] = rand() % (MAX_RAND + 1);  // random number between 0 and MAX_RAND inclusive
    }
}

// Calculates the sum of all values in the array (sequential)
int sum_array(int *array, int size) {
    int total = 0;
    for (int i = 0; i < size; i++) {
        total += array[i];
    }
    return total;
}

// Calculates the sum of all values in the array (parallel with OpenMP)
int sum_array_parallel_omp(int *array, int size) {
    int shared_sum = 0;
    #pragma omp parallel
    {
        int local_sum = 0;
        #pragma omp for
        for (int i = 0; i < size; i++) {
            local_sum += array[i];
        }
        #pragma omp atomic //or critical
        shared_sum += local_sum;
    }
    return shared_sum;
}

// Calculates the sum of all values in the array using OpenMP parallel for with reduction
int sum_array_parallel_omp_reduction_for(int *array, int size) {
    int total = 0;
    #pragma omp parallel for reduction(+:total)
    for (int i = 0; i < size; i++) {
        total += array[i];
    }
    return total;
}

// Calculates the sum of all values in the array using OpenMP parallel task group with reduction
int sum_array_parallel_omp_taskgroup(int *array, int size) {
    int total = 0;
    #pragma omp parallel
    #pragma omp single
    {
        #pragma omp taskgroup task_reduction(+: total)
        {
            for (int i = 0; i < size; i++) {
            #pragma omp task in_reduction(+: total)
            {
                total += array[i];
            }
            }
        }
    }
    return total;
}

// Calculates the sum of all values in the array using OpenMP parallel task loop with reduction
int sum_array_parallel_omp_taskloop(int *array, int size) {
    int total = 0;
    #pragma omp parallel
    #pragma omp single
    {
        #pragma omp taskloop reduction(+: total)
        for (int i = 0; i < size; i++) {
            total += array[i];
        }
    }
    return total;
}
