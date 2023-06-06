use std::num::NonZeroU32;

use crate::{MmsApi as Api, Stat};

#[repr(C)]
pub struct ByteBuffer {
    ptr: *mut u8,
    length: i32,
    capacity: i32,
}

#[allow(dead_code)]
impl ByteBuffer {
    fn len(&self) -> usize {
        self.length
            .try_into()
            .expect("buffer length negative or overflowed")
    }

    fn from_vec(bytes: Vec<u8>) -> Self {
        let length = i32::try_from(bytes.len()).expect("buffer length cannot fit into a i32.");
        let capacity =
            i32::try_from(bytes.capacity()).expect("buffer capacity cannot fit into a i32.");

        // keep memory until call delete
        let mut v = std::mem::ManuallyDrop::new(bytes);

        Self {
            ptr: v.as_mut_ptr(),
            length,
            capacity,
        }
    }

    fn from_vec_struct<T: Sized>(bytes: Vec<T>) -> Self {
        let element_size = i32::try_from(std::mem::size_of::<T>()).unwrap();

        let length = i32::try_from(bytes.len()).unwrap() * element_size;
        let capacity = i32::try_from(bytes.capacity()).unwrap() * element_size;

        let mut v = std::mem::ManuallyDrop::new(bytes);

        Self {
            ptr: v.as_mut_ptr().cast::<u8>(),
            length,
            capacity,
        }
    }

    fn destroy_into_vec(self) -> Vec<u8> {
        if self.ptr.is_null() {
            vec![]
        } else {
            let capacity: usize = self
                .capacity
                .try_into()
                .expect("buffer capacity negative or overflowed");
            let length: usize = self
                .length
                .try_into()
                .expect("buffer length negative or overflowed");

            unsafe { Vec::from_raw_parts(self.ptr, length, capacity) }
        }
    }

    fn destroy_into_vec_struct<T: Sized>(self) -> Vec<T> {
        if self.ptr.is_null() {
            vec![]
        } else {
            let element_size = i32::try_from(std::mem::size_of::<T>()).unwrap();
            let length = usize::try_from(self.length * element_size).unwrap();
            let capacity = usize::try_from(self.capacity * element_size).unwrap();

            unsafe { Vec::from_raw_parts(self.ptr.cast::<T>(), length, capacity) }
        }
    }

    fn destroy(self) {
        drop(self.destroy_into_vec());
    }
}

fn string_to_native(value: String) -> *mut ByteBuffer {
    let buf = ByteBuffer::from_vec(value.into_bytes());
    Box::into_raw(Box::new(buf))
}

fn native_to_string(str_utf8: *const u8, str_len: i32) -> String {
    let slice = unsafe { std::slice::from_raw_parts(str_utf8, usize::try_from(str_len).unwrap()) };
    String::from_utf8(slice.to_vec()).unwrap()
}

#[no_mangle]
pub extern "C" fn maze_width() -> i32 {
    Api::maze_width()
}

#[no_mangle]
pub extern "C" fn maze_height() -> i32 {
    Api::maze_height()
}

#[no_mangle]
pub extern "C" fn wall_front() -> bool {
    Api::wall_front()
}

#[no_mangle]
pub extern "C" fn wall_right() -> bool {
    Api::wall_right()
}

#[no_mangle]
pub extern "C" fn wall_left() -> bool {
    Api::wall_left()
}

#[no_mangle]
#[allow(unused_must_use)]
pub extern "C" fn move_forward(distance: u32) {
    Api::move_forward(if distance < 1 {
        None
    } else {
        Some(NonZeroU32::new(distance).unwrap())
    });
}

#[no_mangle]
#[allow(unused_must_use)]
pub extern "C" fn turn_right() {
    Api::turn_right();
}

#[no_mangle]
#[allow(unused_must_use)]
pub extern "C" fn turn_left() {
    Api::turn_left();
}

#[no_mangle]
#[allow(unused_must_use)]
pub extern "C" fn set_wall(x: u32, y: u32, direction_utf8: *const u8, direction_len: i32) {
    Api::set_wall(
        x,
        y,
        &native_to_string(direction_utf8, direction_len)
            .parse()
            .unwrap(),
    );
}

#[no_mangle]
#[allow(unused_must_use)]
pub extern "C" fn clear_wall(x: u32, y: u32, direction_utf8: *const u8, direction_len: i32) {
    Api::clear_wall(
        x,
        y,
        &native_to_string(direction_utf8, direction_len)
            .parse()
            .unwrap(),
    );
}

#[no_mangle]
#[allow(unused_must_use)]
pub extern "C" fn set_color(x: u32, y: u32, color_utf8: *const u8, color_len: i32) {
    Api::set_color(
        x,
        y,
        &native_to_string(color_utf8, color_len).parse().unwrap(),
    );
}

#[no_mangle]
#[allow(unused_must_use)]
pub extern "C" fn clear_color(x: u32, y: u32) {
    Api::clear_color(x, y);
}

#[no_mangle]
#[allow(unused_must_use)]
pub extern "C" fn clear_all_color() {
    Api::clear_all_color();
}

#[no_mangle]
#[allow(unused_must_use)]
pub extern "C" fn set_text(x: u32, y: u32, text_utf8: *const u8, text_len: i32) {
    Api::set_text(x, y, &native_to_string(text_utf8, text_len));
}

#[no_mangle]
#[allow(unused_must_use)]
pub extern "C" fn clear_text(x: u32, y: u32) {
    Api::clear_text(x, y);
}

#[no_mangle]
#[allow(unused_must_use)]
pub extern "C" fn clear_all_text() {
    Api::clear_all_text();
}

#[no_mangle]
pub extern "C" fn was_reset() -> bool {
    Api::was_reset()
}

#[no_mangle]
#[allow(unused_must_use)]
pub extern "C" fn ack_reset() {
    Api::ack_reset();
}

#[no_mangle]
pub unsafe extern "C" fn free_byte_buffer(buffer: *mut ByteBuffer) {
    let buf = Box::from_raw(buffer);
    // drop inner buffer, if you need Vec<u8>, use buf.destroy_into_vec() instead.
    buf.destroy();
}

#[no_mangle]
pub extern "C" fn get_stat(query_utf8: *const u8, query_len: i32) -> *mut ByteBuffer {
    use Stat::{
        BestRunDistance, BestRunEffectiveDistance, BestRunTurns, CurrentRunDistance,
        CurrentRunEffectiveDistance, CurrentRunTurns, Score, TotalDistance, TotalEffectiveDistance,
        TotalTurns,
    };
    let slice =
        unsafe { std::slice::from_raw_parts(query_utf8, usize::try_from(query_len).unwrap()) };
    let query = String::from_utf8(slice.to_vec()).unwrap();
    let s = match Api::get_stat(&query.parse().unwrap()) {
        TotalDistance(i)
        | TotalTurns(i)
        | BestRunDistance(i)
        | BestRunTurns(i)
        | CurrentRunDistance(i)
        | CurrentRunTurns(i) => i.to_string(),
        TotalEffectiveDistance(f)
        | BestRunEffectiveDistance(f)
        | CurrentRunEffectiveDistance(f)
        | Score(f) => f.to_string(),
    };

    string_to_native(s)
}
