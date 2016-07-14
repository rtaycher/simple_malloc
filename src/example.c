#include <stdlib.h>
#include <stdio.h>
#include <string.h>
int main(){
    char *a = malloc(sizeof("Hello World %i\n"));
    int *b = malloc(sizeof(int));
    *b = 5;
    strcpy(a, "Hello World %i\n");
    printf(a, *b);
    free(a);
}