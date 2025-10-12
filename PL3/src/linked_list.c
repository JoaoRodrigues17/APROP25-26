#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <omp.h>

#define SIZE 1000
#define LIST_SIZE 100

typedef struct data {
    int key;
    char data[SIZE];
} data_t;

typedef struct node {
    data_t item;
    struct node *next;
    omp_lock_t lock;
} node_t;

typedef struct list {
    node_t *head;
} list_t;

/* Initialize empty list */
list_t* create_list() {
    list_t *list = (list_t*) malloc(sizeof(list_t));
    list->head = NULL;
    return list;
}

/* Clean up all nodes and locks */
void destroy_list(list_t *list) {
    node_t *curr = list->head;
    while (curr != NULL) {
        node_t *next = curr->next;
        omp_destroy_lock(&curr->lock);
        free(curr);
        curr = next;
    }
    free(list);
}

/* Thread-safe ordered insert (by key) */
void insert(list_t *list, data_t item) {
    node_t *new_node = (node_t*) malloc(sizeof(node_t));
    new_node->item = item;
    new_node->next = NULL;
    omp_init_lock(&new_node->lock);

    if (list->head == NULL) {
        // Empty list — no locking needed
        list->head = new_node;
        return;
    }

    node_t *prev = NULL;
    node_t *curr = list->head;

    // Lock head node
    omp_set_lock(&curr->lock);

    while (curr != NULL && curr->item.key < item.key) {
        if (prev != NULL) omp_unset_lock(&prev->lock);
        prev = curr;
        curr = curr->next;
        if (curr != NULL) omp_set_lock(&curr->lock);
    }

    // Insert new node in correct position
    if (prev == NULL) {
        // Insert at beginning
        new_node->next = list->head;
        list->head = new_node;
    } else {
        new_node->next = curr;
        prev->next = new_node;
    }

    // Unlock nodes
    if (curr != NULL) omp_unset_lock(&curr->lock);
    if (prev != NULL) omp_unset_lock(&prev->lock);
}

/* Thread-safe delete */
void delete(list_t *list, int key) {
    if (list->head == NULL) return;

    node_t *pred = NULL;
    node_t *curr = list->head;

    // lock the head
    omp_set_lock(&curr->lock);

    // traverse until key >= curr->key
    while (curr && curr->item.key < key) {
        if (pred) omp_unset_lock(&pred->lock);
        pred = curr;
        curr = curr->next;
        if (curr) omp_set_lock(&curr->lock);
    }

    // not found
    if (!curr || curr->item.key != key) {
        if (curr) omp_unset_lock(&curr->lock);
        if (pred) omp_unset_lock(&pred->lock);
        return;
    }

    // unlink
    if (pred == NULL) {
        // deleting head
        list->head = curr->next;
    } else {
        pred->next = curr->next;
    }

    // destroy and free safely
    omp_unset_lock(&curr->lock);
    omp_destroy_lock(&curr->lock);
    free(curr);

    if (pred) omp_unset_lock(&pred->lock);
}

/* Thread-safe search */

int search(list_t *list, int key) {
    node_t *prev = NULL;
    node_t *curr = list->head;

    if (curr == NULL) return 0;

    omp_set_lock(&curr->lock);

    while (curr != NULL && curr->item.key < key) {
        if (prev != NULL) omp_unset_lock(&prev->lock);
        prev = curr;
        curr = curr->next;
        if (curr != NULL) omp_set_lock(&curr->lock);
    }

    int found = 0;
    if (curr != NULL && curr->item.key == key) {
        printf("Found key: %d, data: %s\n", curr->item.key, curr->item.data);
        found = 1;
    }
    if(!found) {
        printf("Key %d not found.\n", key);
    }

    if (curr != NULL) omp_unset_lock(&curr->lock);
    if (prev != NULL) omp_unset_lock(&prev->lock);
    return found;
}

/* Helper: print list (not thread-safe, for debug only) */
void print_list(list_t *list) {
    node_t *curr = list->head;
    printf("List: ");
    while (curr != NULL) {
        printf("(%d,%s) -> ", curr->item.key, curr->item.data);
        curr = curr->next;
    }
    printf("NULL\n");
}

/* Example parallel test */
int main() {
    printf("Starting parallel linked list operations...\n");
    list_t *list = create_list();
    printf("List created.\n");

    #pragma omp parallel for
    for (int i = 0; i < LIST_SIZE; i++) {
        data_t item;
        item.key = i;
        snprintf(item.data, SIZE, "data_%d", item.key);
        insert(list, item);
    }
    printf("Insertions complete.\n");
   #pragma omp parallel for
    for (int i = LIST_SIZE/2; i < LIST_SIZE; i++) {
        int key = i;
        delete(list, key);
    }
    printf("Deletions complete.\n");
    #pragma omp parallel for
    for (int i = 0; i < LIST_SIZE; i++) {
        int key = i;
        search(list, key);
    }
    printf("Searches complete.\n");
    print_list(list);
    destroy_list(list);
    return 0;
}
