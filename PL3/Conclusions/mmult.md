# Matrix multiplication parallelization with the tasking model

The following table shows the times obtained for a fixed sized matrix (512*512), comparing creating tasks per line and per cell level:

| Version       | Time        |
|---------------|-------------|
| Per line      |  0.1012           |
| Per cell      |  0.5076           |

We verified that:

* With tasks, as was the case with parallel loops, parallelization is most efficient per line;

* With this size matrix, the task version performs slightly better than parallel loop version (~0.04 seconds faster). The gap between execution times gets wider with larger matrixes (1024*1024 gets ~0.4 seconds faster with tasks).