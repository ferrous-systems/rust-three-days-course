#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

typedef struct {
  int32_t x;
  int32_t y;
} Point;

void destroy_point(Point *p);

void inspect_point(Point *p);

Point *new_point(int32_t x, int32_t y);
