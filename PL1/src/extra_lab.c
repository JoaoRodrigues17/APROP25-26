#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>
#include <unistd.h>

#define THREAD_NR 4
#define NUM_TASKS 30   // total tasks

int tasks[NUM_TASKS];    // task array (each int = factorial input)
int next_task = 0;       // index of next task to assign
int total_tasks = 0;     // how many tasks have been generated
int done = 0;

pthread_cond_t no_task;
pthread_mutex_t mux;

int calculate_factorial(int n) {

    sleep((rand()%3)+1); //simulate longer processing

    int res = 1;
    for (int i = 2; i <= n; i++)
        res *= i;
    return res;
}

void *thread_function(void *arg) {
    int id = *(int *) arg;

    while (1) {
        pthread_mutex_lock(&mux);

        // Wait for tasks or for "done" flag
        while (next_task >= total_tasks && !done) {
            pthread_cond_wait(&no_task, &mux);
        }

        // If no tasks left AND done flag set → exit
        if (done && next_task >= total_tasks) {
            pthread_mutex_unlock(&mux);
            break;
        }

        // Take one task
        int task = tasks[next_task];
        next_task++;

        pthread_mutex_unlock(&mux);

        // Process the task outside the lock
        int result = calculate_factorial(task);
        printf("Thread %d: factorial(%d) = %d\n", id+1, task, result);
    }

    printf("Thread %d exiting.\n", id);
    return NULL;
}

int main() {
    pthread_t threads[THREAD_NR];
    int thread_ids[THREAD_NR];
    pthread_mutex_init(&mux, NULL);
    pthread_cond_init(&no_task, NULL);

    // Start worker threads
    for (int i = 0; i < THREAD_NR; i++) {
        thread_ids[i] = i;
        pthread_create(&threads[i], NULL, thread_function, &thread_ids[i]);
    }

    // Produce tasks gradually
    for (int i = 0; i < NUM_TASKS; i++) {
        pthread_mutex_lock(&mux);
        tasks[total_tasks] = (rand() % 10) + 1;  // random number 1–10
        total_tasks++;
        pthread_cond_signal(&no_task); // one thread is woken
        pthread_mutex_unlock(&mux);

        sleep((rand() % 2) + 1); // simulate random delay
    }

    
    pthread_mutex_lock(&mux);
    done = 1; 
    pthread_mutex_unlock(&mux);
    pthread_cond_broadcast(&no_task); //signal end of tasks (thread termination)

    for (int i = 0; i < THREAD_NR; i++) {
        pthread_join(threads[i], NULL);
    }

    pthread_mutex_destroy(&mux);
    pthread_cond_destroy(&no_task);

    printf("All tasks complete. Main thread exiting.\n");
    return 0;
}
