#include <stdio.h>
#include <stdint.h>

// Rust と同じ定義を書く
struct point {
    int x;
    int y;
};

// Rust の関数プロトタイプ宣言
// 上の struct point とこれは丁寧にやるならヘッダを作るべき
double dist(struct point *, struct point *);

int main() {
    struct point p1 = { 1, 0 }, p2 = {0, 1};
    double ret;

    ret = dist(&p1, &p2);
    printf("%f\n", ret);
}