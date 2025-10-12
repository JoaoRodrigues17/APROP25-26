# Conclusions of the parallelism exercise on quicksort (part c)

Parallelization in this exercise was implemented as:

* Creating a thread for each recursive call

* Creating a thread for each recursive call, but sorting sequentially if the size of the partition is smaller than a predefined threshold

The following table shows the execution times for each of the implemented versions, using 2000 and 10.000 element arrays and a threshold of  1000 elements:

| Version                   | Execution time (2000) | Execution time (10000) |
|----------------------------|--------------------------|--------------------------|
| Sequential                 | 0.000269                   |  0.001708                   |
| recursive call parallelism  | 0.414295                   | 1.669927                  |
| recursive call parallelism with threshold      | 0.379156                   | 1.709600                   |


From this, and acknowledging some variation in execution times, we can conclude that the sequential algorithm is the most efficient, followed by the parallelism with threshold when the threshold is big in relation to the array size. The parallelism without threshold proved inefficient due to the overhead associated with creating and managing a very large number of threads.