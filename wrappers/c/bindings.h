#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct ByteBuffer {
  uint8_t *ptr;
  int32_t length;
  int32_t capacity;
} ByteBuffer;

int32_t maze_width(void);

int32_t maze_height(void);

bool wall_front(void);

bool wall_right(void);

bool wall_left(void);

void move_forward(uint32_t distance);

void turn_right(void);

void turn_left(void);

void set_wall(uint32_t x, uint32_t y, const uint8_t *direction_utf8, int32_t direction_len);

void clear_wall(uint32_t x, uint32_t y, const uint8_t *direction_utf8, int32_t direction_len);

void set_color(uint32_t x, uint32_t y, const uint8_t *color_utf8, int32_t color_len);

void clear_color(uint32_t x, uint32_t y);

void clear_all_color(void);

void set_text(uint32_t x, uint32_t y, const uint8_t *text_utf8, int32_t text_len);

void clear_text(uint32_t x, uint32_t y);

void clear_all_text(void);

bool was_reset(void);

void ack_reset(void);

void free_byte_buffer(struct ByteBuffer *buffer);

struct ByteBuffer *get_stat(const uint8_t *query_utf8, int32_t query_len);
