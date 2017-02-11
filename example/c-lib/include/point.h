
#ifndef cheddar_generated_point_h
#define cheddar_generated_point_h


#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>
#include <stdbool.h>



typedef struct Point {
	int32_t x;
	int32_t y;
} Point;

Point* new_point(int32_t x, int32_t y);

void destroy_point(Point* p);

void inspect_point(Point* p);



#ifdef __cplusplus
}
#endif


#endif
