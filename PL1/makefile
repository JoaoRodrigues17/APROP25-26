mmult: src/mmult.c
	gcc src/mmult.c -lpthread -o bin/mmult

quicksort: src/quick_sort.c
	gcc src/quick_sort.c -lpthread -o bin/quicksort

philosofers: src/dining_philosophers.c
	gcc src/dining_philosophers.c -lpthread -o bin/philosophers

barber: src/barber.c
	gcc src/barber.c -lpthread -o bin/barber

extra: src/extra_lab.c
	gcc src/extra_lab.c -lpthread -o bin/extra

init:
	mkdir -p bin

clean:
	rm -rf bin/*