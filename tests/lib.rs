extern crate mpd_rs_interface;

extern crate mpd;

use mpd::Client;
use std::net::TcpStream;
use mpd_rs_interface::{get_song, get_tag, next, pause, play, prev, stop, update};

#[test]
fn song_gets_serialized() {
    let conn: Client<TcpStream> = Client::connect("127.0.0.1:6600").unwrap();
    let song = get_song(conn);
    println!("{}\n", song);

    assert!(song.contains("title"));
}
