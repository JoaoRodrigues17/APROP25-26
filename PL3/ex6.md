### LLVM vs GCC

a) Execution Time Comparison of Parallelization Approaches
- GCC and LLVM OpenMP runtimes provide tasking models but overhead and performance depending on task granularity and dependency complexity.
- LLVM's OpenMP runtime (libomp) maintains a double-ended task queue per thread, favoring last-in-first-out execution and allowing work stealing to balance load. This is efficient for dynamic scheduling but can incur overhead for fine-grained tasks.
GCC's runtime (libgomp) implements a different task scheduling approach, generally allowing ready tasks on multiple queues and supporting a work-stealing mechanism.

- LLVM tends to have somewhat lower overhead for managing task dependencies, especially when fine-grained task dependencies are involved. This results in better scalability and faster completion times for highly parallelized code with complex dependencies.

- For coarse-grained tasks, differences diminish as overhead is amortized over longer task execution.

- A taskgraph framework built on LLVM further reduces overhead in task management and improves scalability significantly compared to vanilla GCC and LLVM runtimes, showing LLVM's potential advantage in future optimizations.

b. Strategy of Task Execution by Threads in GCC vs LLVM OpenMP Implementations
LLVM OpenMP uses thread-local double-ended queues for tasks. Threads push new tasks to their own queues and pop from the end for immediate execution (LIFO). If a thread’s queue is empty, it steals tasks from the front of other threads’ queues (FIFO steal) to maintain load balance.

GCC uses a similar work-stealing approach, but its management of the queues and task dependencies can differ, sometimes leading to varied contention and overhead patterns.

LLVM provides more aggressive task throttling and dynamic switching between undeferred (execute immediately) and deferred (spawn for future execution) tasks to control runtime overhead.

Both implementations support task dependencies, but LLVM's dependency resolution is often more optimized, facilitating faster progress and reduced contention during synchronization.