# Group Evaluation #1 - PL3 ex6 (Analyzing and Evaluating)

Following, we present the statement for this exercise:

Analyze by profiling the execution of exercise 5 with tasks, in both GCC and LLVM implementations of OpenMP: 
* a. Compare the execution time of the different approaches to parallelize program  
* b. Compare the strategy used by both implementation for the execution of tasks by the threads

-----

Exercise 5 asked for a parallelized version of the following algorithm:
``` C
 for (int i = 1; i < N - 1; i++){
        for (int j = 1; j < N - 1; j++){
                M[i][j] = (M[i][j-1] + M[i-1][j] + M[i][j+1] + M[i+1][j])/4.0;
        }
}
```

Which is an algorithm that could be used in a real setting, for example, for image smoothing/blurring.

We will be using both parallel implementations of this algorithm (task per cell and task per block) to perform this comparison. Our input will be fixed as a 1024*1024 matrix and a block size of 16.

To get accurate measurements, we used an average of 100 runs of each algorithm, using the following main function:

``` C
int main(int argc, char *argv[])
{
    srand(time(NULL));
    setup();

    int runs = 100;
    double total_par_time = 0.0;
    double total_block_time = 0.0;

    printf("Running %d iterations...\n", runs);

    // Run parallel cells "runs" times
    printf("Parallel Cells... ");
    for (int i = 0; i < runs; i++) {
        
        double start = omp_get_wtime();
        par();
        double end = omp_get_wtime();
        total_par_time += (end - start);
        copy_matrix(R, M);
    }
    double avg_par_time = total_par_time / runs;
    printf("done.\n");

    // Run parallel blocks multiple times
    printf("Parallel Blocks... ");
    for (int i = 0; i < runs; i++) {
        double start1 = omp_get_wtime();
        par_blocks();
        double end1 = omp_get_wtime();
        total_block_time += (end1 - start1);
        copy_matrix(R, M);
    }
    double avg_block_time = total_block_time / runs;
    printf("done.\n");

    // Print results
    printf("Sequential time:       %fs\n", sequential_time);
    printf("\n- ==== Performance (averaged over %d runs) ==== -\n", runs);
    printf("Parallel cells avg:    %fs\n", avg_par_time);
    printf("Parallel blocks avg:   %fs\n", avg_block_time);

    return 0;
}
```

Below are two tables that show the execution time for each parallel algorithm, for each compiler: GCC and LLVM

Baseline sequential time: ~0.0089

### GCC table 

| Algorithm Version       | Execution time        |
|---------------|-------------|
| Cell parallelism       |     ~0.9729       |
| Block parallelism      |     ~0.0020       |

### LLVM table 

| Algorithm Version       | Execution time        |
|---------------|-------------|
| Cell parallelism       |     ~4.4691       |
| Block parallelism      |     ~0.0083       |


We can conclude the following:


* Block parallelism has a much better performance than Cell parallelism;

* Cell parallelism has a worse performance than the sequential version;

* LLVM had worse performance than GCC for both algorithms, much more noticeable for cell parallelism.


----

## GCC vs LLVM implementation of OMP

The results obtained above may be explained by the way GCC and LLVM work with the openMP tasking model.

* GCC uses Work-First Scheduling (WFS), where a thread that creates a task executes it immediately and places it in a central queue for others to steal. This reduces the scheduling cost of creating multiple small-workload tasks, which is the reason why, while it still performed badly in comparison to the sequential algorithm, it still managed to out-perform LLVM's implementation on the cell-level parallelization of the algorithm.

* LLVM, on the other hand, uses Breadth-First Scheduling (BFS) with per-thread queues and work stealing, running new tasks first and letting idle threads pick older tasks in order to maximize cache efficiency. Here, as the generation and scheduling of tasks is preferred over running immediately, the overhead of running multiple small tasks becomes too large, resulting in the bad performance of the cell-level paralellization of the algorithm

The main takeaway, correlating with the results obtained above is that LLVM can perform better with a small number of larger tasks, while struggling with a large number of small workload tasks. The smaller difference in execution time for coarser-grained tasks (block level parallelism) is also an indicator of this.

LLVM's strengths can't shine in this exercise as they are tied with cache-locality, and having dependencies within the OMP pragma's makes it difficult for LLVM to be able to choose the tasks to run with more likelihood of cache hits.

