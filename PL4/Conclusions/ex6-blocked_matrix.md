# Group Evaluation #1 - PL3 ex6 (Analyzing and Evaluating)

Following, we present the statement for this exercise:

Analyze by profiling the execution of exercise 5 with tasks (with and without dependencies), in both GCC and LLVM implementations of OpenMP:
* a. Compare the execution time of the different approaches to parallelize program  
* b. Compare the strategy used by both implementation for the execution of tasks by the threads 

-----

We will be using both parallel implementations of the algorith developed in exercise 5 to perform this comparison. Our input will be a 1024*1024 matrix and a block size of 16.

It must be noted that the versions without dependencies do not produce correct results. 

Below are two tables, one for GCC and other for LLVM, that show the execution time (a 5 execution average) for each parallel algorithm. 

### GCC table 

| Algorithm Version       | Execution time        |
|---------------|-------------|
| Sequential                                 |     ~0.0089       |
| Cell parallelism (no dependencies)         |     ~2.2903       |
| Cell parallelism (with dependencies)       |     ~1.0214       |
| Block parallelism (no dependencies)        |     ~0.0012       |
| Block parallelism (with dependencies)      |     ~0.0016       |

### LLVM table 

| Algorithm Version       | Execution time        |
|---------------|-------------|
| Sequential                                 |     ~0.0089       |
| Cell parallelism (no dependencies)         |     ~0.9342       |
| Cell parallelism (with dependencies)       |     ~5.4278       |
| Block parallelism (no dependencies)        |     ~0.0013       |
| Block parallelism (with dependencies)      |     ~0.0058       |


We can conclude then that:

* Cell parallelism has a worse performance than sequential, while Block parallelism has a better performance;

* LLVM had worse performance than GCC for Block parallelism in general and for Cell parallelism with dependencies

* In gcc, with dependencies, there is an decrease in execution time for the Cell parallelism, while there is an increase for Block parallelism;

* In LLVM, Both algorithms get a worse execution time with dependencies.

----

## GCC vs LLVM

The results obtained above may be explained by the way GCC and LLVM work with the openMP tasking model.

* GCC uses Work-First Scheduling (WFS), where a thread that creates a task executes it immediately and places it in a central queue for others to steal. This reduces scheduling overhead for large tasks but can create load imbalance and idle threads when many fine-grained tasks are generated, like in the per-cell parallel version.

* LLVM, on the other hand, uses Breadth-First Scheduling (BFS) with per-thread queues and work stealing, running new tasks first and letting idle threads pick older tasks in order to maximize cache efficiency.

The main takeaways, correlating with the results obtained above is that GCC can perform better with larger tasks, while struggling with fine grained tasks (dependencies actually help reduce overhead here due to locality (cache)).

On the other hand, LLVM's cache efficiency can work better with fine grained tasks, but struggles with the overhead of keeping dependencies between tasks.