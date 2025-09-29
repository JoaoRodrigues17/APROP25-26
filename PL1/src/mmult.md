# Conclusions of the parallelism exercise in matrix multiplication (part d)

Parallelization in this exercise was implemented at several levels: at the position level, at the row level, and at the block-of-rows level, with sequential processing used as a control.

The following table shows the execution times for each of the implemented versions, using 200x200 and 400x400 matrices:

| Version                   | Execution time (200x200) | Execution time (400x400) |
|----------------------------|--------------------------|--------------------------|
| Sequential                 | 0.0333                   | 0.2727                   |
| Position-level parallelism | 9.0334                   | 45.6080                  |
| Row-level parallelism      | 0.0484                   | 0.4050                   |
| Block-of-rows parallelism (20 and 40 rows, respectively) | 0.0117 | 0.4810 |

From this, and acknowledging some variation in execution times, we can conclude that block-of-rows parallelism is the most efficient, followed by row-level parallelism. Position-level parallelism proved inefficient due to the overhead associated with creating and managing a very large number of threads.
