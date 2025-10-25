/*
 * Copyright 2022 Instituto Superior de Engenharia do Porto
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * 	http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
#include <stdio.h>
#include <time.h>
#include <stdlib.h>
#include <unistd.h>
#include <errno.h>
#include <omp.h>

// Matrices dimensions, where A is LxM, B is MxN, and C is LxN
// #define L 512
// #define M 512
// #define N 512

#define L 1024
#define M 1024
#define N 1024


#define DEFAULT_NUM_THREADS 4

int A[L][M];
int B[M][N];
int C[L][N];
int expected[L][N];
double sequential_time;


#define MIN_RAND -10
#define MAX_RAND 10

//Matrix multiplication versions
/**
 * Matrix multiplication: A[L,M]* B[M,N] = C[L,N]
 **/
void seq();
void par_row(int num_threads);
void par_row_c1(int num_threads);
void par_row_c2(int num_threads);
void par_row_sched_static(int num_threads);
void par_row_sched_dynamic(int num_threads);
void par_row_sched_guided(int num_threads);
void par_block(int num_threads);
void par_block_c1(int num_threads);

//Utility functions
void calc(int l,int n);
void fill(int* matrix, int height,int width);
void print(int* matrix,int height,int width);
void assert(int C[L][N],int expected[L][N]);
void c_clean();
void setup();

// C[0][0] = sum(A[0][i] * B[0][i]) for i = 0 to M ...
// do this from c[0][0] till c[L-1][N-1]
int main(int argc, char *argv[])
{
    srand(time(NULL));
    int num_threads = DEFAULT_NUM_THREADS;
    if(argc < 2){
        printf("Number of threads was not specified. Will use default value: %d\n",DEFAULT_NUM_THREADS);
    }else{
        num_threads = atoi(argv[1]);
    }
    printf("Working with %d threads to multiplicate two matrices: A{%d,%d}*B{%d,%d} = C{%d,%d}\n", num_threads, L, M, M, N, L, N);

    setup();
    
    printf("Thread working on lines... ");
    double begin = omp_get_wtime();
    par_row(num_threads);
    double end = omp_get_wtime();
    double per_row_time = (end - begin);
    printf("done.\n");
    assert(C,expected);

    c_clean();

    printf("Thread working on lines collapse 1... ");
    double begin_c1 = omp_get_wtime();
    par_row_c1(num_threads);
    double end_c1 = omp_get_wtime();
    double per_row_time_c1 = (end_c1 - begin_c1);
    printf("done.\n");
    assert(C,expected);

    c_clean();

    printf("Thread working on lines collapse 2... ");
    double begin_c2 = omp_get_wtime();
    par_row_c2(num_threads);
    double end_c2 = omp_get_wtime();
    double per_row_time_c2 = end_c2 - begin_c2;
    printf("done.\n");
    assert(C,expected);

    c_clean();

    printf("Thread working on lines... ");
    double begin_sched_static = omp_get_wtime();
    par_row(num_threads);
    double end_sched_static = omp_get_wtime();
    double per_row_time_sched_static = (end_sched_static - begin_sched_static);
    printf("done.\n");
    assert(C,expected);

    c_clean();

    printf("Thread working on lines... ");
    double begin_sched_dynamic = omp_get_wtime();
    par_row(num_threads);
    double end_sched_dynamic = omp_get_wtime();
    double per_row_time_sched_dynamic = (end_sched_dynamic - begin_sched_dynamic);
    printf("done.\n");
    assert(C,expected);

    c_clean();

    printf("Thread working on lines... ");
    double begin_sched_guided = omp_get_wtime();
    par_row(num_threads);
    double end_sched_guided = omp_get_wtime();
    double per_row_time_sched_guided = (end_sched_guided - begin_sched_guided);
    printf("done.\n");
    assert(C,expected);

    c_clean();

    printf("Thread working on region (block)... ");
    begin = omp_get_wtime();
    par_block(num_threads);
    end = omp_get_wtime();
    double per_block_time = (end - begin);
    printf("done.\n");
    assert(C,expected);

    c_clean();

    printf("Thread working on region (block) collapse 1... ");
    begin = omp_get_wtime();
    par_block_c1(num_threads);
    end = omp_get_wtime();
    double per_block_time_c1 = (end - begin);
    printf("done.\n");
    assert(C,expected);

    printf("\n- ==== Performance ==== -\n");
    printf("Sequential time:     %fs\n",sequential_time);
    printf("Parallel lines time: %fs\n",per_row_time);
    printf("Parallel lines time collapse 1: %fs\n",per_row_time_c1);
    printf("Parallel lines time collapse 2: %fs\n",per_row_time_c2);
    printf("Parallel lines time sched static: %fs\n",per_row_time_sched_static);
    printf("Parallel lines time sched dynamic: %fs\n",per_row_time_sched_dynamic);
    printf("Parallel lines time sched guided: %fs\n",per_row_time_sched_guided);
    printf("Parallel block time: %fs\n",per_block_time);  
    printf("Parallel block time collapse 1: %fs\n",per_block_time_c1);  
}

/***
 * YOUR IMPLEMENTATIONS HERE! 
 **/
/**
 * Version where each thread is responsible for a set of rows
 **/

void par_row(int num_threads){
    #pragma omp parallel for 
    for (int l = 0; l < L; l++)
    {
        for (int n = 0; n < N; n++)
        {
            int sum = 0;
            for (int m = 0; m < M; m++)
            {
                sum += A[l][m] * B[m][n];
            }
            C[l][n] = sum;
        }
    }
}

void par_row_c1(int num_threads){
    #pragma omp parallel for collapse(1)
    for (int l = 0; l < L; l++)
    {
        for (int n = 0; n < N; n++)
        {
            int sum = 0;
            for (int m = 0; m < M; m++)
            {
                sum += A[l][m] * B[m][n];
            }
            C[l][n] = sum;
        }
    }
}

void par_row_c2(int num_threads){
    #pragma omp parallel for collapse(2)
    for (int l = 0; l < L; l++)
    {
        for (int n = 0; n < N; n++)
        {
            int sum = 0;
            for (int m = 0; m < M; m++)
            {
                sum += A[l][m] * B[m][n];
            }
            C[l][n] = sum;
        }
    }
}

void par_row_sched_static(int num_threads){
    #pragma omp parallel for schedule(static)
    for (int l = 0; l < L; l++)
    {
        for (int n = 0; n < N; n++)
        {
            int sum = 0;
            for (int m = 0; m < M; m++)
            {
                sum += A[l][m] * B[m][n];
            }
            C[l][n] = sum;
        }
    }
}

void par_row_sched_dynamic(int num_threads){
    #pragma omp parallel for schedule(dynamic)
    for (int l = 0; l < L; l++)
    {
        for (int n = 0; n < N; n++)
        {
            int sum = 0;
            for (int m = 0; m < M; m++)
            {
                sum += A[l][m] * B[m][n];
            }
            C[l][n] = sum;
        }
    }
}

void par_row_sched_guided(int num_threads){
    #pragma omp parallel for schedule(guided)
    for (int l = 0; l < L; l++)
    {
        for (int n = 0; n < N; n++)
        {
            int sum = 0;
            for (int m = 0; m < M; m++)
            {
                sum += A[l][m] * B[m][n];
            }
            C[l][n] = sum;
        }
    }
}

/**
 * Version where each thread is responsible for a block of cells
 **/
void par_block(int num_threads){
    for (int l = 0; l < L; l++)
    {
        #pragma omp parallel for
        for (int n = 0; n < N; n++)
        {
            int sum = 0;
            for (int m = 0; m < M; m++)
            {
                sum += A[l][m] * B[m][n];
            }
            C[l][n] = sum;
        }
    }
}

void par_block_c1(int num_threads){
    for (int l = 0; l < L; l++)
    {
        #pragma omp parallel for collapse(1)
        for (int n = 0; n < N; n++)
        {
            int sum = 0;
            for (int m = 0; m < M; m++)
            {
                sum += A[l][m] * B[m][n];
            }
            C[l][n] = sum;
        }
    }
}


/**

 * Example of a sequential matrix multiplication
*/
void seq()
{
    for (int l = 0; l < L; l++)
    {
        for (int n = 0; n < N; n++)
        {
            int sum = 0;
            for (int m = 0; m < M; m++)
            {
                sum += A[l][m] * B[m][n];
            }
            C[l][n] = sum;
        }
    }
}


///////////////////////////////////////////////////////////////////////////
/**
 * UTILITY FUNCTIONS
*/
void calc(int l,int n){
    int sum = 0;
    for (int m = 0; m < M; m++)
    {
        sum += A[l][m] * B[m][n];
    }
    C[l][n] = sum;
}

void fill(int* matrix, int height,int width){
    for (int l = 0; l < height; l++)
    {
        for (int n = 0; n < width; n++)
        {
            *((matrix+l*width) + n) = MIN_RAND + rand()%(MAX_RAND-MIN_RAND+1);
        }
    }
}

void print(int* matrix,int height,int width){
    
    for (int l = 0; l < height; l++)
    {
        printf("[");
        for (int n = 0; n < width; n++)
        {
            printf(" %5d",*((matrix+l*width) + n));
        }
        printf(" ]\n");
    }
}

void assert(int C[L][N],int expected[L][N]){
    for (int l = 0; l < L; l++)
    {
        for (int n = 0; n < N; n++)
        {
            if(C[l][n] != expected[l][n]){
                printf("Wrong value at position [%d,%d], expected %d, but got %d instead\n",l,n,expected[l][n],C[l][n]);
                exit(-1);
            }
        }
        
    }
}

void c_clean(){
    for (int l = 0; l < L; l++)
    {
        for (int n = 0; n < N; n++)
        {
            C[l][n] = 0;
        }
    }
}


void setup(){
    fill((int *)A,L,M);
    fill((int *)B,M,N);
    double begin = omp_get_wtime();
    seq();
    double end = omp_get_wtime();
    sequential_time = (end - begin);

    for (int l = 0; l < L; l++)
    {
        for (int n = 0; n < N; n++)
        {
            expected[l][n] = C[l][n];
            C[l][n] = 0;
        }
    }
}