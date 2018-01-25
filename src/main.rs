extern crate mpd;

use mpd::Client;
use std::net::TcpStream;
use std::env;


pub fn play(mut conn: Client) { conn.play().unwrap(); }

pub fn stop(mut conn: Client) { conn.stop().unwrap(); }

fn main() {
    let conn: Client<TcpStream> = Client::connect("127.0.0.1:6600").unwrap();

    parse_cmd_args(conn);
}

fn parse_cmd_args(conn: Client) {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let cmd = &args[1];
            match &cmd[..] {
                "play" => play(conn),
                "stop" => stop(conn),
                "now" => get_current_info(conn, "title"),
                "album" => get_current_info(conn, "album"),
                "artist" => get_current_info(conn, "artist"),
                _ => ()
            }
        }
        _ => { () }
    }
}

fn get_current_info(mut conn: Client, tag: &str) {
    let song = conn.currentsong().unwrap();

    fn no_play() { println!("Nothing is playing!"); }

    match &song {
        &None => no_play(),
        &Some(ref s) => match tag {
            "album" => match &s.tags.get("Album") {
                &None => println!("Album not found!"),
                &Some(album) => println!("{}", album),
            },
            "artist" => match &s.tags.get("Artist") {
                &None => println!("Artist not found!"),
                &Some(artist) => println!("{}", artist),
            },
            "title" => match &s.title {
                &None => no_play(),
                &Some(ref t) => { println!("{}", t); }
            },
            _ => ()
        }
    }
}
