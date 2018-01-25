//extern crate libc;
//
//use std::ffi::CStr;
//use std::str;
//use std::ops::Deref;
//use libc::c_char;

//#[link(name = "mpdutils")]
//extern {
//    fn get_title() -> *const c_char;
//    fn goodbye(now_playing: *const c_char);
//}
//
//struct MpdStatus {
//    now_playing: *const c_char,
//}
//
//impl MpdStatus {
//    fn new() -> MpdStatus {
//        MpdStatus { now_playing: unsafe { get_title() } }
//    }
//}
//
//impl Drop for MpdStatus {
//    fn drop(&mut self) {
//        println!("MpdStatus is being killed!");
//        unsafe {
//            goodbye(self.now_playing);
//        }
//    }
//}
//
//impl Deref for MpdStatus {
//    type Target = str;
//
//    fn deref<'a>(&'a self) -> &'a str {
//        let c_str = unsafe { CStr::from_ptr(self.now_playing) };
//        c_str.to_str().unwrap()
//    }
//}

fn main() {
//    let c_buffer: *const c_char = unsafe { get_title() };
//    let c_string: &CStr = unsafe { CStr::from_ptr(c_buffer) };
//    let string_slice: &str = c_string.to_str().unwrap();
//    let song_title: String = string_slice.to_owned();
//    let mpd_status = MpdStatus::new();
//    let title_string = str::from_utf8(mpd_status.deref().as_bytes());
//    let song_title = match title_string {
//        Ok(v) => v,
//        Err(_err) => "Err: COULDN'T FORMAT TO UTF-8",
//    };
//    println!("{}", song_title);
}
