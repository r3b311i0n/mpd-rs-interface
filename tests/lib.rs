extern crate mpd_rs_interface;

extern crate mpd;

use mpd::Client;
use std::net::TcpStream;
use mpd_rs_interface::{get_song, get_tag, update};

#[test]
fn db_updates() {
    let conn: Client<TcpStream> = Client::connect("127.0.0.1:6600").unwrap();
    assert_eq!(update(conn), String::from("Updating Database"));
}

#[test]
fn song_gets_serialized() {
    let conn: Client<TcpStream> = Client::connect("127.0.0.1:6600").unwrap();
    let song = get_song(conn);
    assert!(song.contains("title"));
}
