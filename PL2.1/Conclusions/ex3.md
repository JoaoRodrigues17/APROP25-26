# Conclusions for matrix multiplication and mandlebrot for altering collapse and schedule clauses

## Key takeaways:

* For Matrix multiplication, varying the collapse clause from 1 to 2 did not significantly impact performance, nor did changing the schedule clause from static to dynamic. This suggests that for this specific workload, the overhead of managing more complex scheduling or collapsing loops does not yield performance benefits.

* For the Mandelbrot set computation, reducing the collapse clause from 2 to 1 resulted in a an error. Changing the schedule clause from static to guided did not produce a significant performance difference. However, using guided scheduling significantly reduced the execution time.