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

// TODO: Rewrite this test to compare against a specific song. Otherwise it's useless.
#[test]
fn can_get_tag() {
    let mut conn: Client<TcpStream> = Client::connect("127.0.0.1:6600").unwrap();
    let title = get_tag(&mut conn, "title");
    print!("{}", title);
//    assert!(song.contains("title"));
}

// TODO: This test always succeeds; Find a nice way to check whether song is JSON.
#[test]
fn song_gets_serialized() {
    let conn: Client<TcpStream> = Client::connect("127.0.0.1:6600").unwrap();
    let song = get_song(conn);
    assert!(song.contains("title"));
}
