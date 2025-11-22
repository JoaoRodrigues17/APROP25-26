# Bubble sort parallelization findings

The Bubble sort algorithm tightly depends on its last iteration, making it impossible to parellelize without a slight tweak to the algorithm.

By splitting the sequential sorting algorithm in even-odd groups, we are able to run parallelizable sections sequentially, as the swaps done don't impact the next iterations.

Using this approach, the following table shows the time each algorithm takes to perform a sort for x size arrays:

| Version                    | Execution time (1.000)   | Execution time (5.000)   | Execution time (10.000)  | Execution time (20.000)  | Execution time (50.000)  | Execution time (100.000)  |
|----------------------------|--------------------------|--------------------------|--------------------------|--------------------------|--------------------------|--------------------------|
| Sequential                 | 0.0032                   | 0.0678                   | 0.3225                   | 1.3788                   | 8.6942                   | 35.0315                  |
| Even-odd parallel          | 0.0063                   | 0.0258                   | 0.2503                   | 0.2304                   | 2.1974                   | 8.0553                   |

Concluding that, for very small array sizes, the parallel version is not worth it, while, as array sizes get bigger, the speedup is very considerable 