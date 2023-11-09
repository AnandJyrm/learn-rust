#include <stdio.h>
#include <stdlib.h>
#include "cov_c.h"

void cov_function(int abc) {
    if (abc > 2) {
        printf("Greater than 2\n");
    } else {
        printf("Lesser than 2\n");
    }
}
