#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>
#include <pthread.h>
#include "timed_event.h"
#include <sys/syscall.h>
#include <string.h>

callback_fn_t global_callback_fn;

void register_fn(callback_fn_t callback_fn) {
    printf("TID: %ld -> hello from register\n", syscall(SYS_gettid));
    global_callback_fn = callback_fn;
    return;
}

void* timer_thread(void * thread_arg) {
    timer_value_t* timer_param = (timer_value_t *)thread_arg;
    int i = 0;
    printf("TID: %ld -> hello from thread\n", syscall(SYS_gettid));
    while (i < timer_param->retry) {
        i++;
	sleep(timer_param->interval);
	printf("TID: %ld -> inside the loop\n", syscall(SYS_gettid));
	global_callback_fn();
    } 
    printf("TID: %ld -> bye from thread\n", syscall(SYS_gettid));
    return NULL;
}

void init_timer(void) {
    printf("TID: %ld -> hello from init_timer\n", syscall(SYS_gettid));
    pthread_t tid = {0};
    timer_value_t *timer_params = NULL;
    timer_params = calloc(1, sizeof(timer_value_t));
    timer_params->retry = 10;
    timer_params->interval = 1;
    pthread_create(&tid,
		   NULL,
	           timer_thread,
	           timer_params);
    (void)pthread_detach(tid);
    printf("TID: %ld -> bye from init_timer\n", syscall(SYS_gettid));
}
