# Time comparison of 2D vec matrix multiplication vs DMatrix vs C sequential equivalent

In this exercise we aimed to make a comparison of the efficiency of matrix multiplication with different implementations.

We used different matrix sizes got the following results:

| Version                   | Execution time (3x3) | Execution time (10x10)   | Execution time (100x100)   | Execution time (500x500)  | Execution time (1000x1000)  | 
|---------------|--------------------------|--------------------------|--------------------------|--------------------------|--------------------------|
| Rust 2D vec                 | 0.000005s                   | 0.000431s                   | 0.083803s                   | 10.724185s                   | 95.232266s  | 
| Rust DMatrix          | 0.000027s                   | 0.000345s                   | 0.0359723s                  | 3.999475s                   |   34.720241s    |
| C sequential equivalent          | 0.000001s                   | 0.000006s                   | 0.003127s                   | 0.436208s                   |   3.503768s      | 

## Conclusions

* Rust's versions were much slower than the C sequential equivalent. In terms of general performance, the versions ranked: C > DMatrix > 2D Vectors;

* This difference in performance is due to C's optimization in terms of cache locality (contiguous memory array);

* The 2D vector version has poor cache locality and the multiplication algorithm causes double indirection and pointer changing, which drastically slows slows down processing;

* We verified that the 2D vector version can be better than DMatrix for very small matrixes;

* Finally, DMatrix's poor performance might be due to its lack of optimization, which can be tweaked by enabling BLAS/LAPACK libraries.