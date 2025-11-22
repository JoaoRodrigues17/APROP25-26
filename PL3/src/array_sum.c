#include <stdio.h>
#include <omp.h>
#include <assert.h>
#include <stdlib.h>

#define ARR_LEN 1000000

int array[ARR_LEN];
long expected_sum = ((long)(ARR_LEN - 1) * ARR_LEN) / 2;


void fill_array(int* array, int size){
    for (int i = 0; i < size; i++)
    {
        array[i] = i;
    }
    
}

//Algorithms
long sum_array_explicit(int* array){ //aka sequential...
    long sum = 0;
    #pragma omp parallel
    {
        int my_id = omp_get_thread_num();
        int num_threads = omp_get_num_threads();
        for(int i = my_id * ARR_LEN/num_threads; i < (my_id+1)*ARR_LEN/num_threads; i++){
            #pragma omp critical
            sum += array[i];
        }
    }
    return sum;

}

long sum_array_par_loop(int* array){

    long sum = 0;
    #pragma omp parallel for reduction(+:sum)
    for (int i = 0; i < ARR_LEN; i++)
    {
        sum += array[i];
    }
    return sum;
}

long sum_array_task_group(int* array){
    long sum = 0;
    #pragma omp parallel
    {
    #pragma omp single
    {
        #pragma omp taskgroup task_reduction(+:sum)
        {
            for (int i = 0; i < ARR_LEN; i++)
            {
                #pragma omp task in_reduction(+:sum)
                {
                    sum += array[i];
                }
            }
        }
    }
}
    return sum;

}

long sum_array_task_loop(int* array){
    long sum = 0;
    #pragma omp taskgroup task_reduction(+:sum)
    {
        #pragma omp taskloop in_reduction(+:sum)
        for (int i = 0; i < ARR_LEN; i++)
        {
            sum += array[i];
        }
    }
    
    return sum;
}

int main(){
    fill_array(array,ARR_LEN);
    double start1 = omp_get_wtime();
    long explicit_sum = sum_array_explicit(array);
    double finish1 = omp_get_wtime();
    assert(expected_sum == explicit_sum);
    printf("Explicit done.\n");

    double start2 = omp_get_wtime();
    long par_loop = sum_array_par_loop(array);
    double finish2 = omp_get_wtime();
    assert(expected_sum == par_loop);
    printf("Par loop done.\n");

    double start3 = omp_get_wtime();
    long task_group = sum_array_task_group(array);
    double finish3 = omp_get_wtime();
    assert(expected_sum == task_group);
    printf("Task group done.\n");

    double start4 = omp_get_wtime();
    long task_loop = sum_array_task_loop(array);
    double finish4 = omp_get_wtime();
    assert(expected_sum == task_loop);
    printf("Task loop done.\n");
    printf("\n- ==== Performance ==== -\n");
    printf("Explicit: %fs\n",finish1-start1);
    printf("Par loop: %fs\n",finish2-start2);
    printf("Task group: %fs\n",finish3-start3);
    printf("Task loop: %fs\n",finish4-start4);

    return 0;
}

