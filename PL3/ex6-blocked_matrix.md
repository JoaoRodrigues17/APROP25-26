# Group Evaluation #1 - PL3 ex6 (Analyzing and Evaluating)

This report is based on the following statement:

Perform an analysis about parallelism by profiling the execution of exercise 5 with tasks with OpenMP. The source code was compiled by both GCC and LLVM. The analysis was done by performing: 
* a. A comparison of the execution time of each of the different approaches to parallelize the program. 
* b. A comparison of the strategy used by each compiler for the execution of tasks by the threads

-----

The objective of the exercise 5 was a parallelization of the following algorithm:
``` C
 for (int i = 1; i < N - 1; i++){
        for (int j = 1; j < N - 1; j++){
                M[i][j] = (M[i][j-1] + M[i-1][j] + M[i][j+1] + M[i+1][j])/4.0;
        }
}
```

The algorithm was perfectly parallelizable and could be used in a real setting, for example, for image smoothing/blurring.

At the exercise 5, the parallelization strategies impemented were: task per cell and task per block.
Both strategies were used to perform this comparison and measuring the execution times with an average of 10 runs per algorithm to reduce the effect of possible fluctuations. The main function used for that purpose is as follows:

``` C
int main(int argc, char *argv[])
{
    srand(time(NULL));
    setup();

    int runs = 10;
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

After performing multiple runs for eache strategy, was created below a collection of tables that show the execution time measured, and respectively, the percentual difference relative to the sequential counterpart for each parallel algorithm, taking into acount the variables: compiler, algorithm, matrix size, nr of threads, and for block-level parallelism, block size.

Our analysis of these tables will first include a comparison of the execution times of the parallel versions of the algorithm, limiting our scope to the GCC compiler's results and then, we will focus on comparing the results of GCC and LLVM, adding a theoretical explaination of the differences in implementation of OMP's tasks, with the aim of explaining the differences' origin. 

The machine running the tests had the following specs:
* Processor	Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz (2.59 GHz)
* Installed RAM	16.0 GB (15.8 GB usable)
* Using WSL



## **Using GCC compiler**

### **Sequential Algorithm**

#### **Execution time**

| Matrix Size   | Sequential Algorithm  |
|---------------------|---|
| 32*32               |0.000009 |
| 512*512             |0.002244 |
| 1024*1024           | 0.009601 |
| 2048*2048           |0.035605 |

---

### **Cell level parallelism**

#### **Execution time**

| Matrix Size \ Threads | 1         | 2         | 4         | 8         |
|------------------------|-----------|-----------|-----------|-----------|
| 32×32                  | 0.000263  | 0.000862  | 0.001032  | 0.000987  |
| 512×512                | 0.143900  | 0.170492  | 0.257036  | 0.218618  |
| 1024×1024              | 0.741816  | 0.737438  | 1.134946  | 0.940509  |
| 2048×2048              | 3.327312  | 3.666132  | 3.835253  | 3.547910  |


#### **% difference from sequential values**

| Matrix Size \ Threads | 1         | 2         | 4         | 8         |
|----------------------|-----------|-----------|-----------|-----------|
| 32×32                | +2822.22%   | +9468.89%   | +11355.56%  | +10866.67%  |
| 512×512              | +6309.26%   | +7498.31%   | +11353.08%  | +9644.16%   |
| 1024×1024            | +7623.18%   | +7578.40%   | +11726.48%  | +9692.01%   |
| 2048×2048            | +9247.22%   | +10298.49%  | +10772.65%  | +9859.54%   |



---

### **Block level parallelism**

#### **Execution time**

| Matrix Size \ Threads & Block size      | 1<br>(bs: 1, 16, 64, 128) | 2<br>(bs: 1, 16, 64, 128)  | 4<br>(bs: 1, 16, 64, 128)  | 8<br>(bs: 1, 16, 64, 128) |
|---------------------|---|---|---|---|
| 32*32               |  0.000010<br>0.000095<br>0.137252<br>2.438690 | 0.000018<br>0.000227<br>0.004345<br> 0.019813  | 0.000038<br>0.000385<br>0.003968<br>0.026056  | 0.000088<br>0.000271<br>0.003743<br>0.012492  |
| 512*512             |  0.002115<br> 0.002422<br>0.004868<br>0.013692 | 0.002163<br>0.001312<br>0.004809<br>0.012045  | 0.002590<br>0.000743<br>0.003733<br>0.017696  | 0.002420<br>0.000591<br>0.003748<br>0.011704  |
| 1024*1024           |  0.008843<br>0.009158<br>0.012815<br>0.025214 | 0.008257<br>0.004595<br>0.008782<br>0.029118  | 0.008919<br>0.002619<br>0.004543<br>0.014208  | 0.008785<br>0.001743<br>0.004543<br>0.016821  |
| 2048*2048           |  0.033927<br>0.035716<br>0.040065<br> 0.051929 | 0.034380<br>0.018514<br>0.021029<br>0.034748  | 0.034688<br>0.010692<br>0.012861<br>0.021287  | 0.034878<br>0.007113<br>0.008680<br>0.017858  |



#### **% difference from sequential values**

| Matrix Size \ Threads & Block size | 1<br>(bs:1,16,64,128)                                      | 2<br>(bs:1,16,64,128)                                    | 4<br>(bs:1,16,64,128)                                     | 8<br>(bs:1,16,64,128)                                     |
| ---------------------------------- | ---------------------------------------------------------- | -------------------------------------------------------- | --------------------------------------------------------- | --------------------------------------------------------- |
| 32×32                          | +11.1% <br> +955.6% <br> +1524355% <br> +27095444% | +100% <br> +2422% <br> +47055% <br> +219022%       | +322% <br> +4177% <br> +43966% <br> +288289%        | +878% <br> +2911% <br> +41478% <br> +138700%        |
| 512×512                        | −5.8% <br> +7.9% <br> +117% <br> +510%                 | −3.6% <br> −41.5% <br> +114% <br> +437%          | +15.4% <br> −66.9% <br> +66.4% <br> +689%             | +7.8% <br> −73.6% <br> +67.0% <br> +421%              |
| 1024×1024                      | −7.9% <br> −4.6% <br> +33.5% <br> +162.6%          | −14.0% <br> −52.1% <br> −8.5% <br> +203.4%   | −7.1% <br> −72.7% <br> −52.7% <br> +48.0%     | −8.5% <br> **−81.8%** <br> -52.7% <br> +75.1%     |
| 2048×2048                      | −4.7% <br> +0.3% <br> +12.5% <br> +45.9%               | −3.4% <br> −48.0% <br> −40.9% <br> −2.4% | −2.6% <br> −70.0% <br> −63.9% <br> −40.2% | −2.0% <br> −80.0% <br> −75.6% <br> −49.9% |


---

## **Using LLVM compiler**

### **Sequential Algorithm**

#### **Execution time**

| Matrix Size   | Sequential Algorithm  |
|---------------------|---|
| 32*32               |0.000007 |
| 512*512             |0.002159 |
| 1024*1024           |0.008576 |
| 2048*2048           | 0.034407 |


---

### **Cell level parallelism**

#### **Execution time**

| Matrix Size \ Threads      | 1 | 2 | 4 | 8 |
|---------------------|---|---|---|---|
| 32*32               | 0.000295  | 0.000989  | 0.000886  | 0.001515  |
| 512*512             | 0.022045  | 0.456415  | 0.453564  | 0.613745  |
| 1024*1024           | 0.087755  | 2.843085  |  3.083455  | 3.454878  |
| 2048*2048           | 0.366021  | 24.487839  | 25.318506  | 27.968112  |




#### **% difference from sequential values**

| Matrix Size \ Threads | 1         | 2         | 4         | 8         |
|----------------------|-----------|-----------|-----------|-----------|
| 32×32                | +4114.29%   | +14114.29%  | +12642.86%  | +21642.86%  |
| 512×512              | +920.09%    | +21085.12%  | +20964.84%  | +28416.38%  |
| 1024×1024            | +923.37%    | +33096.41%  | +35955.85%  | +40271.63%  |
| 2048×2048            | +963.19%    | +71042.38%  | +73499.02%  | +81135.27%  |


---

### **Block level parallelism**

#### **Execution time**

| Matrix Size \ Threads & Block size      | 1<br>(bs: 1, 16, 64, 128) | 2<br>(bs: 1, 16, 64, 128)  | 4<br>(bs: 1, 16, 64, 128)  | 8<br>(bs: 1, 16, 64, 128) |
|---------------------|---|---|---|---|
| 32*32               |  0.000501<br>0.000040<br>0.000367<br>0.001527 | 0.000221<br>0.000395<br>0.004011<br>0.018460  | 0.000336<br>0.000221<br>0.003051<br>0.014152  | 0.000494<br>0.000268<br>0.004693<br>0.023798  |
| 512*512             |  0.002083<br>0.001944<br>0.002269<br>0.003475 | 0.002714<br>0.001330<br>0.006633<br>0.031392  | 0.002576<br>0.000737<br>0.004409<br>0.028642  | 0.002773<br>0.000704<br>0.007367<br>0.036415  |
| 1024*1024           |  0.008319<br>0.008247<br>0.008290<br>0.009297 | 0.008820<br>0.004760<br>0.010764<br>0.040276  | 0.008715<br>0.002475<br>0.006269<br>0.037146  | 0.010181<br>0.002037<br>0.008606<br>0.039188  |
| 2048*2048           |  0.033016<br>0.033387<br>0.035865<br>0.035162 | 0.033392<br>0.019026<br>0.028455<br>0.054568  | 0.034870<br>0.010341<br>0.013133<br>0.033697  | 0.039406<br>0.007259<br>0.009584<br>0.039859  |



#### **% difference from sequential values**

| Matrix Size \ Threads & Block size | 1<br>(bs:1,16,64,128)                      | 2<br>(bs:1,16,64,128)                         | 4<br>(bs:1,16,64,128)                            | 8<br>(bs:1,16,64,128)                             |
| ---------------------------------- | ------------------------------------------ | --------------------------------------------- | ------------------------------------------------ | ------------------------------------------------- |
| 32×32                          | +7087% <br> +471% <br> +5143% <br> +21743% | +3057% <br> +5536% <br> +57157% <br> +263428% | +4700% <br> +3057% <br> +43586% <br> +201028%    | +6971% <br> +3729% <br> +66900% <br> +339686%     |
| 512×512                        | −3.5% <br> −9.96% <br> +5.1% <br> +61%     | +25.7% <br> −38.4% <br> +207% <br> +1354%     | +19.3% <br> −65.9% <br> +104% <br> +1226%    | +28.5% <br> −67.4% <br> +241% <br> +1587%     |
| 1024×1024                     | −3.0% <br> −3.8% <br> −3.3% <br> +8.4%     | +2.8% <br> -44.5% <br> +25.4% <br> +369%  | +1.6% <br> −71.2% <br> −26.9% <br> +333%     | +18.7% <br> −76.3% <br> +0.35% <br> +357%     |
| 2048×2048                      | −4.0% <br> −3.0% <br> +4.2% <br> +2.2%     | −3.0% <br> −44.7% <br> −17.3% <br> +58.6% | +1.3% <br> −69.9% <br> −61.8% <br> −2.0% | +14.5% <br> **−78.9%** <br> −72.1% <br> +1.3% |


---

## **Conclusions**

### **a) Comparing the results of the parallel versions of the algorithm**

As mentioned above, to simplify this analysis, and since the comparison between the parallel algorithms is similar for both compilers, we will only take into account the results of GCC for this paragraph.

Following are the conclusions taken from the data gathered above:

* **Cell-level parallelism**

    * All the configurations had a worse performance than the sequential version;

    * The size of the matrix has an unpredictable effect on the performance, although a larger matrix should result in worse performance, due to more tasks being generated.

    * In general, for 1, 2 and 4 threads the algorithm performed ascendingly worse, then got slightly better for 8 threads, with the best results being at 1 thread;

    * The worst performing (most execution time gain) configuration was: 1024*1024 size, 4 threads;

    * The best performing (least execution time gain) configuration was: 32*32 size, 1 thread; 

    * Cell-level parallelism creates an excessive number of fine-grained tasks (each cell is a task), resulting in large scheduling and synchronization overheads. The cost of managing these tasks far outweighs the computational savings, especially for small matrix elements. This explains why even with more threads, performance degrades instead of improving.

* **Block-level parallelism**
    * Some configurations showed improvement relative to the sequential version;

    * Smaller matrix sizes led to a worse performance, while larger matrixes had enormous performance gains;

    * In general, a block size of 16 performed the best, with 64 also being decent and the remaining performing poorly (exception for small matrix sizes where block size 1 also performs better);

    * Increasing number of threads generated better performance

    * The worst performing configuration was: 32*32 size, 1 thread, 128 block size;

    * The best performing configuration was: 1024*1024 size, 8 threads, 16 block size.
    
    * Block-level parallelism reduces the total number of tasks by grouping cells into larger blocks, which minimizes task creation overhead and improves cache locality. The best performance at block size 16 and 8 threads suggests a good balance between parallel workload distribution and memory efficiency.

    * Larger block sizes reduce parallelism and may increase load imbalance among threads, which can explain the performance degradation at block size 128.

* A block level-parallelization reveals itelf as the go-to way to parallelize this problem, achieving significantly greater results with the correct configurations, with the potential to achieve a 81.8% improvement in comparison to the sequential algorithm.

### **b) Comparing the results of GCC and LLVM's implementation of OMP tasks**

Now, to compare the results between GCC and LLVM, we will point out the best and worst cases for both algorithms in LLVM, as we did above for GCC with the intent of finding results that can be correlated to their implementations.

* **Cell-level parallelism**

    * GCC worst performance: 1.134946 (+11726.48%) @ 1024*1024 matrix size and 4 threads;

    * LLVM worst performance: 27.968112s (+81135.27%) @ 2048*2048 matrix size and 8 threads;

    * GCC best performance: 0.000263 (+2822.22%) @ 32*32 matrix size and 1 thread;

    * LLVM best performance: 0.022045 (+920.09%) @ 512*512 matrix size and 1 thread;

    * GCC has a worse performance "in the middle", where the matrix is not too large but not too small, and the number of threads is also not too large but not too small;

    * LLVM has a more linear difference: as size increases and the number of threads increases, the performance gets worse. The amount of small tasks being created has a negative impact on LLVM;

    * It is important to note that as soon as there is more than 1 thread in LLVM, in cell level parallelism, all execution times are worse than GCC.

    * Since both GCC and LLVM rely on OpenMP tasking, the magnitude of task creation overhead depends on the runtime’s task scheduler efficiency — LLVM’s runtime seems to handle large task counts worse, which explains its steeper degradation.

* **Block-level parallelism**

    * GCC worst performance: 2.438690 (+27095444%) @ 32*32 matrix size, 1 thread and 128 block size;

    * LLVM worst performance: 0.023798s (+339686%) @ 32*32 matrix size, 8 threads and 128 block size;

    * GCC best performance: 0.001743 (-81.1%) @ 1024*1024 matrix size, 8 threads and 16 block size;

    * LLVM best performance: 0.007259 (−78.9%) @ 2048*2048 matrix size, 8 threads and 16 block size;

    * Both GCC and LLVM behaved simillarly with both of the compilers having greater improvements of performance as the matrix grew bigger in size, with a higher number of threads and the block size of 16;

    * In general, GCC's execution times seem to almost always be slightly better than LLVM's. This behaviour is expected for smaller sized matrixes, and smaller block sizes. Perhaps the testing scope was too small, however, it would be theoretically expected that LLVM's implementation would provide better results as matrix size and block size grew, as we'll see below.

----

## GCC vs LLVM implementation of OMP

The results obtained above may be explained by the way GCC and LLVM work with the openMP tasking model. The following is a short comparison between the two:

### GCC (libgomp)

GCC’s OpenMP runtime follows a Work-First Scheduling (WFS) model with tied tasks, meaning each task remains bound to the thread that first executes it. This approach minimizes task migration and reduces scheduling overhead, which is particularly beneficial for fine-grained workloads like cell-level parallelism. Tasks are often executed immediately either by the spawning thread or the idle threads that can take it, preserving data locality and avoiding excessive queuing. As a result, GCC achieves lower overhead and better performance when the number of small tasks is high, though its static task–thread binding can occasionally limit load balancing for irregular workloads. Such is the case in cell-level parallelism.

### LLVM (libomp)

LLVM’s OpenMP runtime employs a Breadth-First scheduling (BFS) policy with a dynamic task model, where every thread has their own queue, one thread initially creates all tasks and other idle threads try stealing the oldest task to try to maximize cache hits. While this enables improved load balancing for coarse-grained parallelism (e.g., block-level tasks), it also introduces higher task creation and synchronization overhead due to the frequent need for coordination. This design leads to significant slowdowns for fine-grained (cell-level) workloads, where the number of tasks is extremely high, which explains its low performance for cell-level parallelism. However, as the workload becomes coarser and more balanced — for instance, with larger block sizes and fewer tasks — LLVM’s dynamic scheduling becomes more effective, potentially being able to surpass GCC's performance on block-level parallelism for a decent sized matrix and blocks.

---

in short:

* GCC’s Work-First, tied-task strategy is more efficient for workloads with many small, independent computations (fine-grained tasks).

* LLVM’s Breadth-First, dynamic task model is better suited for coarser-grained parallelism, where fewer tasks exist and load balancing matters more than per-task overhead.

