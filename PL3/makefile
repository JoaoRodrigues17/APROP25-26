BIN= init mmult mandel arraysum

NUM_THREADS = 4
all: $(BIN)

init:
	mkdir -p bin
	
# mmult: src/mmult.c
# 	gcc -Wall src/mmult.c -o bin/mmult -fopenmp 

# mmult_run: mmult
# 	bin/mmult $(NUM_THREADS)

# mandel: src/mandelbrot.c
# 	gcc -Wall src/mandelbrot.c -o bin/mandel -fopenmp -lm 

# mandel_run: mandel
# 	bin/mandel $(NUM_THREADS)

# quicksort: src/quick_sort.c
# 	gcc -Wall src/quick_sort.c -o bin/quicksort -fopenmp 

# quicksort_run: quicksort
# 	bin/quicksort $(NUM_THREADS)

# bubblesort: src/bubble_sort.c
# 	gcc -Wall src/bubble_sort.c -o bin/bubblesort -fopenmp 

# bubblesort_run: bubblesort
# 	bin/bubblesort $(NUM_THREADS)

# arraysum: src/arraysum.c
# 	gcc -Wall src/arraysum.c -o bin/arraysum -fopenmp -lm 

# arraysum_run: arraysum
# 	bin/arraysum $(NUM_THREADS)

blockedmmult: src/blockedmmult.c
	gcc -Wall src/blockedmmult.c -o bin/blockedmmult -fopenmp

blockedmmult_run: blockedmmult
	bin/blockedmmult $(NUM_THREADS)
