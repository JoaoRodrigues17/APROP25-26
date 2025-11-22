#include <stdio.h>
#include <time.h>
#include <stdlib.h>
#include <unistd.h>
#include <errno.h>
#include <omp.h>

// Matrices dimensions, where A is LxM, B is MxN, and C is LxN
#define N 1024
#define BS 16
#define bx (N / BS)
#define DEFAULT_NUM_THREADS 4

int M[N][N];
int R[N][N]; // This stores the original matrix
int expected[N][N];
double sequential_time;


#define MIN_RAND -10
#define MAX_RAND 10

//Matrix multiplication versions
/**
 * Matrix multiplication: A[L,M]* B[M,N] = C[L,N]
 **/
void seq();
void par();
void par_blocks();

//Utility functions
void fill(int* matrix, int height,int width);
void print(int* matrix,int height,int width);
void assert(int C[N][N],int expected[N][N]);
void copy_matrix(int src[N][N], int dest[N][N]);
void setup();


int main(int argc, char *argv[])
{
    srand(time(NULL));
    
    setup();
    
    printf("Parallel Cells... ");
    double start = omp_get_wtime();
    par();
    double end = omp_get_wtime();
    double per_row_time = end - start;
    printf("done.\n");
    assert(M,expected);

    copy_matrix(R,M);
    
    printf("Parallel Blocks... ");
    double start1 = omp_get_wtime();
    par_blocks();
    double end1 = omp_get_wtime();
    double per_block_time = end1 - start1;
    printf("done.\n");
    assert(M,expected);
    printf("\n- ==== Performance ==== -\n");
    printf("Sequential time:     %fs\n",sequential_time);
    printf("Parallel cells time: %fs\n",per_row_time);
    printf("Parallel blocks time: %fs\n",per_block_time);  
}

/***
 * YOUR IMPLEMENTATIONS HERE! 
 **/
/**
 * Version where each thread is responsible for a set of rows
 **/

//Parallel version
void par(){
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


// Parallel blocks version
void par_blocks(){
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



/**

 * Example of a sequential matrix multiplication
*/
void seq()
{
    int i,j;
    for (i = 1; i < N - 1; i++){
        for (j = 1; j < N - 1; j++){
            M[i][j] = (M[i][j-1] + M[i-1][j] + M[i][j+1] + M[i+1][j])/4.0;
        }
    }
}


///////////////////////////////////////////////////////////////////////////
/**
 * UTILITY FUNCTIONS
*/


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

void assert(int M[N][N],int expected[N][N]){
    for (int l = 0; l < N; l++)
    {
        for (int n = 0; n < N; n++)
        {
            if(M[l][n] != expected[l][n]){
                printf("Wrong value at position [%d,%d], expected %d, but got %d instead\n",l,n,expected[l][n],M[l][n]);
                exit(-1);
            }
        }
        
    }
}


void copy_matrix(int src[N][N], int dest[N][N]){
    for (int l = 0; l < N; l++)
    {
        for (int n = 0; n < N; n++)
        {
            dest[l][n] = src[l][n];
        }
    }
}


void setup(){
    fill((int *)R,N,N);
    copy_matrix(R,M);
    printf("Sequential... ");
    clock_t begin = clock();
    seq();
    clock_t end = clock();
    sequential_time = (double)(end - begin) / CLOCKS_PER_SEC;
    printf("done.\n");

    for (int l = 0; l < N; l++)
    {
        for (int n = 0; n < N; n++)
        {
            expected[l][n] = M[l][n];
        }
    }
    copy_matrix(R,M);
}