extern crate libc;

use libc::size_t;

#[link(name = "mpdutils", kind = "static")]
extern {
    fn get_title() -> size_t;
}

fn main() {
    let song_title = unsafe { get_title() };
    println!("{}", song_title);
}
