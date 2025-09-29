#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>
#include <unistd.h>

#define NR_PHILOSOPHERS 5 

pthread_mutex_t forks[NR_PHILOSOPHERS];

void think(int id){
    printf("%d is thinking\n", id+1);

    sleep(rand() % 3);
}
void eat(int id){
    printf("%d is eating\n", id+1);

    sleep(rand() % 3);
}

void *philosopher_func(void *arg){
    int id = *(int *) arg;
    int left_fork = id;
    int right_fork = (id+1)% NR_PHILOSOPHERS;
    while(1){
        think(id);
        if (id == NR_PHILOSOPHERS - 1) {                    // Every philosopher picks the right fork first
            pthread_mutex_lock(&forks[right_fork]);
            printf("%d got fork %d\n", id+1, right_fork+1);
            pthread_mutex_lock(&forks[left_fork]);
            printf("%d got fork %d\n", id+1, left_fork+1);
        } else {
            pthread_mutex_lock(&forks[left_fork]);          // Except the last one to avoid deadlock
            printf("%d got fork %d\n", id+1, left_fork+1);  // (every philosopher has one fork in their right hand, no one can eat)
            pthread_mutex_lock(&forks[right_fork]);
            printf("%d got fork %d\n", id+1, right_fork+1);
        }

        eat(id);

        pthread_mutex_unlock(&forks[left_fork]);
        pthread_mutex_unlock(&forks[right_fork]);

    }
}

int main(){
    pthread_t tid[NR_PHILOSOPHERS];
    int ids[NR_PHILOSOPHERS];

    srand(time(NULL));

    // Initialize forks
    for (int i = 0; i < NR_PHILOSOPHERS; i++) {
        pthread_mutex_init(&forks[i], NULL);
    }

    // Create philosopher threads
    for (int i = 0; i < NR_PHILOSOPHERS; i++) {
        ids[i] = i;
        pthread_create(&tid[i], NULL, philosopher_func, &ids[i]);
    }

// ------- unreacheable code, end with sigint (CTRL + C)
    for (int i = 0; i < NR_PHILOSOPHERS; i++) {
        pthread_join(tid[i], NULL);
    }

    for (int i = 0; i < NR_PHILOSOPHERS; i++) {
        pthread_mutex_destroy(&forks[i]);
    }

    return 0;
}