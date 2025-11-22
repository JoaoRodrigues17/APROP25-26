# Quicksort parallelization with the tasking model

* We verified that, following a similar approach to the one developed on PL2, with tasks, the execution times for the same array size are similar, with the task version, in average being slightly faster.

* As previously noticed, there is a sweet spot for parallelizing this algorithm in this fashion (without a threshold), as a very small array can be sorted faster sequentially and a very big array generates a huge overhead in thread management.