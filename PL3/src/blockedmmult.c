/*
 * Copyright 2022 Instituto Superior de Engenharia do Porto
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *  http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
#include <stdio.h>
#include <pthread.h>
#include <time.h>
#include <stdlib.h>
#include <unistd.h>
#include <errno.h>
#include <omp.h>

// Matrix dimensions
#define N 1024
#define BS 16
#define bx (N / BS)
#define DEFAULT_NUM_THREADS 4

// Single NxN matrix for stencil operation
double M[N][N];
double M_temp[N][N];
double expected[N][N];
double sequential_time;

#define MIN_RAND 0.0
#define MAX_RAND 100.0

// Stencil processing functions
void seq_stencil();
void process_block_matrix();
void process_block_matrix_blocks(int block_size);

// Utility functions
void init_matrix();
void copy_matrix(double src[N][N], double dst[N][N]);
void assert_matrix(double M[N][N], double expected[N][N]);
void m_clean();

int main(int argc, char *argv[])
{
    srand(time(NULL));
    
    printf("Stencil computation on %dx%d matrix\n", N, N);
    printf("Block size: %d, Number of blocks: %d\n", BS, bx);
    printf("Number of threads: %d\n\n", DEFAULT_NUM_THREADS);

    // Initialize and compute sequential version
    init_matrix();
    
    printf("Computing sequential version...");
    clock_t begin = clock();
    seq_stencil();
    clock_t end = clock();
    sequential_time = (double)(end - begin) / CLOCKS_PER_SEC;
    printf("done.\n");
    
    // Save expected results
    copy_matrix(M, expected);
    
    // Test row-wise parallelization
    m_clean();
    printf("Block matrix processing (row-wise tasks with dependencies)...");
    begin = clock();
    process_block_matrix();
    end = clock();
    double block_process_time = (double)(end - begin) / CLOCKS_PER_SEC;
    printf("done.\n");
    assert_matrix(M, expected);

    // Test block-wise parallelization
    m_clean();
    printf("Block matrix processing (block-wise with BS=%d)...", BS);
    begin = clock();
    process_block_matrix_blocks(BS);
    end = clock();
    double block_process_blocks_time = (double)(end - begin) / CLOCKS_PER_SEC;
    printf("done.\n");
    assert_matrix(M, expected);

    printf("\n- ==== Performance ==== -\n");
    printf("Sequential time:              %fs\n", sequential_time);
    printf("Stencil row-wise tasks:       %fs (Speedup: %.2fx)\n", 
           block_process_time, sequential_time/block_process_time);
    printf("Stencil block-wise (BS=%d):   %fs (Speedup: %.2fx)\n", 
           BS, block_process_blocks_time, sequential_time/block_process_blocks_time);
    
    return 0;
}

//============================================================================
// STENCIL OPERATIONS
//============================================================================

/**
 * Sequential stencil computation
 * Each point is the average of its 4 neighbors
 */
void seq_stencil() {
    for (int i = 1; i < N - 1; i++) {
        for (int j = 1; j < N - 1; j++) {
            M_temp[i][j] = (M[i][j-1] + M[i-1][j] + M[i][j+1] + M[i+1][j]) / 4.0;
        }
    }
    
    // Copy results back to M
    for (int i = 1; i < N - 1; i++) {
        for (int j = 1; j < N - 1; j++) {
            M[i][j] = M_temp[i][j];
        }
    }
}

/**
 * Alinea a: Row-wise parallelization with task dependencies
 * Each row depends on the previous row, current row, and next row
 * This creates a wave-front of parallelism
 */
void process_block_matrix() {
    #pragma omp parallel 
    #pragma omp single
    {
        for (int i = 1; i < N - 1; i++){
            for (int j = 1; j < N - 1; j++){
                #pragma omp task depend (in: M[i][j-1], M[i-1][j], M[i][j+1], M[i+1][j]) depend (out: M[i][j])
                M[i][j] = (M[i][j-1] + M[i-1][j] + M[i][j+1] + M[i+1][j])/4.0;
            }
        }
    }
}

/**
 * Alinea b: Block-wise parallelization
 * Divides the matrix into blocks of size BS x BS
 * Each block can be processed independently using double buffering
 */
void process_block_matrix_blocks(int block_size) {
    #pragma omp parallel
    #pragma omp single
    {
        for(int bi = 0; bi < BS; bi++){
            for(int bj = 0; bj < BS; bj++){
                int inf_i = 1 + bi * bx;
                int inf_j = 1 + bj * bx;
                int sup_i = (inf_i + bx < N - 1) ? (inf_i + bx) : (N - 1);
                int sup_j = (inf_j + bx < N - 1) ? (inf_j + bx) : (N - 1);
                #pragma omp task depend (in: M[inf_i-1][inf_j:sup_j-1], M[sup_i][inf_j:sup_j-1], M[inf_i:sup_i-1][inf_j-1], M[inf_i:sup_i-1][sup_j]) depend (out: M[inf_i:sup_i-1][inf_j:sup_j-1])
                {
                    for (int i = inf_i; i < sup_i; i++){
                        for (int j = inf_j; j < sup_j; j++){
                            M[i][j] = (M[i][j-1] + M[i-1][j] + M[i][j+1] + M[i+1][j])/4.0;
                        }
                    }
            }

            }

        }
    }

}

///////////////////////////////////////////////////////////////////////////
/**
 * UTILITY FUNCTIONS
 */

void init_matrix() {
    for (int i = 0; i < N; i++) {
        for (int j = 0; j < N; j++) {
            M[i][j] = MIN_RAND + ((double)rand() / RAND_MAX) * (MAX_RAND - MIN_RAND);
            M_temp[i][j] = 0.0;
        }
    }
}

void copy_matrix(double src[N][N], double dst[N][N]) {
    for (int i = 0; i < N; i++) {
        for (int j = 0; j < N; j++) {
            dst[i][j] = src[i][j];
        }
    }
}

void assert_matrix(double M[N][N], double expected[N][N]) {
    double epsilon = 1e-6;
    int errors = 0;
    
    for (int i = 1; i < N - 1; i++) {
        for (int j = 1; j < N - 1; j++) {
            if (fabs(M[i][j] - expected[i][j]) > epsilon) {
                if (errors < 10) {  // Only print first 10 errors
                    printf("Wrong value at position [%d,%d], expected %f, but got %f instead\n",
                           i, j, expected[i][j], M[i][j]);
                }
                errors++;
            }
        }
    }
    
    if (errors > 0) {
        printf("Total errors found: %d\n", errors);
        exit(-1);
    } else {
        printf("Validation passed!\n");
    }
}

void m_clean() {
    init_matrix();
}
