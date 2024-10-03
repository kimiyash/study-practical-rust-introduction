#include <stdlib.h>
#include <stdio.h>

void take_ownership(int *i, void(*dtor)(int *)) {
    printf("got %d\n", *i);
    // C のコードでメモリを開放する
    // Rust で用意した値は Rust から貰ったデストラクタで解放する
    dtor(i);
}

int* make_memory() {
    int *i;

    i = malloc(sizeof(int));
    *i = 2;

    return i;
}