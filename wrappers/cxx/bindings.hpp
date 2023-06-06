#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct ByteBuffer {
  uint8_t *ptr;
  int32_t length;
  int32_t capacity;
};

extern "C" {

int32_t maze_width();

int32_t maze_height();

bool wall_front();

bool wall_right();

bool wall_left();

void move_forward(uint32_t distance);

void turn_right();

void turn_left();

void set_wall(uint32_t x, uint32_t y, const uint8_t *direction_utf8, int32_t direction_len);

void clear_wall(uint32_t x, uint32_t y, const uint8_t *direction_utf8, int32_t direction_len);

void set_color(uint32_t x, uint32_t y, const uint8_t *color_utf8, int32_t color_len);

void clear_color(uint32_t x, uint32_t y);

void clear_all_color();

void set_text(uint32_t x, uint32_t y, const uint8_t *text_utf8, int32_t text_len);

void clear_text(uint32_t x, uint32_t y);

void clear_all_text();

bool was_reset();

void ack_reset();

void free_byte_buffer(ByteBuffer *buffer);

ByteBuffer *get_stat(const uint8_t *query_utf8, int32_t query_len);

} // extern "C"
