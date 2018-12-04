#include <cstdint>
#include <cstdlib>

struct Point {
  int32_t x;
  int32_t y;
};

extern "C" {

void destroy_point(Point *p);

void inspect_point(Point *p);

Point *new_point(int32_t x, int32_t y);

} // extern "C"
