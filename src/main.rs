extern crate libc;

use std::ffi::CStr;
use std::str;
use libc::c_char;

#[link(name = "mpdutils")]
extern {
    fn get_title() -> *const c_char;
}

fn main() {
    let c_buffer: *const c_char = unsafe { get_title() };
    let c_string: &CStr = unsafe { CStr::from_ptr(c_buffer) };
    let string_slice: &str = c_string.to_str().unwrap();
    let song_title: String = string_slice.to_owned();
    println!("{}", song_title);
}
