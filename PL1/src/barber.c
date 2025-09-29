#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>
#include <semaphore.h>
#include <unistd.h>

#define CHAIRS 5   // Number of waiting chairs
#define BARBERS 3  // Number of barbers (change to 1 for a)
#define CUSTOMERS 20 // Total customer number

sem_t customers;   // available customers
sem_t barbers;     // available barbers
pthread_mutex_t mutex;  // mutex to access "waiting"
int waiting = 0;   

void *barber(void *arg) {
    while (1) {
        sem_wait(&customers);      // Wait for a customer (barber goes to sleep)
        pthread_mutex_lock(&mutex);
        waiting--;                 // One customer will get served
        pthread_mutex_unlock(&mutex);
        
        sem_post(&barbers);        // Barber ready for haircut
        printf("Barber is cutting hair...\n");
        sleep(rand() % 3 + 1);     // Simulate haircut time
        printf("Barber finished haircut.\n");
    }
    return NULL;
}

void *customer(void *arg) {
    printf("Customer nr.%ld arrived!\n", (long) arg+1);
    pthread_mutex_lock(&mutex);
    if (waiting < CHAIRS) { // Check if all chairs are taken
        waiting++;
        printf("Customer %ld is waiting. Waiting customers = %d\n", (long)arg+1, waiting);
        sem_post(&customers);    // Notify barber
        pthread_mutex_unlock(&mutex);

        sem_wait(&barbers);      // Wait for barber to be free
        printf("Customer %ld is getting a haircut.\n", (long)arg+1);
    } else {
        printf("Customer %ld left (no chairs).\n", (long)arg+1);
        pthread_mutex_unlock(&mutex);
    }
    return NULL;
}

int main() {
    pthread_t barberThreads[BARBERS];
    pthread_t customersThreads[CUSTOMERS];

    sem_init(&customers, 0, 0);
    sem_init(&barbers, 0, 0);
    pthread_mutex_init(&mutex, NULL);

    for (long i = 0; i < BARBERS; i++)
        pthread_create(&barberThreads[i], NULL, barber, (void *)i); 

    for (long i = 0; i < 20; i++) {
        pthread_create(&customersThreads[i], NULL, customer, (void *)i);
        sleep(rand() % 3); // Customers arrive at random intervals
    }

    for (int i = 0; i < CUSTOMERS; i++)
        pthread_join(customersThreads[i], NULL);

    for (int i = 0; i < BARBERS; i++)
        pthread_cancel(barberThreads[i]);
        
    pthread_mutex_destroy(&mutex);
    sem_destroy(&customers);
    sem_destroy(&barbers);

    return 0;
}
