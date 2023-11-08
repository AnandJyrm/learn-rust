typedef void (
    *callback_fn_t)(void);

typedef struct timer_value_t_ {
    int retry;
    int interval;
} timer_value_t;

void register_fn(callback_fn_t callback_fn);

void init_timer(void);

