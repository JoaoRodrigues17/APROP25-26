# Comparison of Parallelization mechanisms for getting the sum of an array's elements

The goal of this exercise was to test different strategies for parallelizing the problem of summing an array's elements:

* Explicit handling of the array splitting and shared sum variable 
* Parallel for loop with reduction
* Parallel taskgroup with reduction
* Parallel taskloop with reduction 

By testing these approaches multiple times, we ended up with the following ranking of best execution times and scalability

* 1. Parallel for loop
* 2. Parallel taskloop
* 3. Explicit handling
* 4. Parallel taskgroup


The approach envolved only using these openMP constructs on the algorithm, without tuning it by creating chunks for each task, for example. Using those modifications could lead to a better performance of the parallel taskgroup strategy, as the main issue with it was the extreme overhead of creating a task for a single calculation.