# Mandelbrot parallelization with the tasking model

The goal of this exercise was to develop a parallel solution using tasks for the mandelbrot problem and compare it to the previously developed solution with worksharing constructs.

The approaches we did with tasks were:

* Using taskloop (in a similar fashion to omp parallel for)
* Creating a task per point
* Creating a task per line

The most efficient version was the one that created a task per line, and in comparison with the previous solution with worksharing constructs, this one was ever so slightly slower. This could be due to the overhead of managing tasks and the threadpool