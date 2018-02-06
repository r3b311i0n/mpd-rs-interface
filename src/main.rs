extern crate mpd;
extern crate serde;
extern crate serde_json;

use mpd::Client;
use std::net::TcpStream;
use std::env;

// TODO: Implement JSON serializable for info.
pub fn play(mut conn: Client) { conn.play().unwrap(); }

pub fn pause(mut conn: Client) { conn.pause(true).unwrap(); }

pub fn stop(mut conn: Client) { conn.stop().unwrap(); }

pub fn next(mut conn: Client) { conn.next().unwrap(); }

pub fn prev(mut conn: Client) { conn.prev().unwrap(); }

pub fn update(mut conn: Client) { conn.rescan().unwrap(); }

fn main() {
    let conn: Client<TcpStream> = Client::connect("127.0.0.1:6600").unwrap();

    parse_cmd_args(conn);
}

fn parse_cmd_args(conn: Client) {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let cmd = &args[1].to_lowercase();
            match &cmd[..] {
                "play" => play(conn),
                "pause" => pause(conn),
                "stop" => stop(conn),
                "next" => next(conn),
                "prev" => prev(conn),
                "update" => update(conn),
                "file" => get_current_info(conn, "file"),
                "stream-name" => get_current_info(conn, "stream-name"),
                "title" => get_current_info(conn, "title"),
                "album" => get_current_info(conn, "album"),
                "artist" => get_current_info(conn, "artist"),
                "duration" => get_current_info(conn, "duration"),
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
            "file" => println!("{}", &s.file),
            "album" => match &s.tags.get("Album") {
                &None => println!("Album not found!"),
                &Some(album) => println!("{}", album),
            },
            "artist" => match &s.tags.get("Artist") {
                &None => println!("Artist not found!"),
                &Some(artist) => println!("{}", artist),
            },
            "duration" => match &s.duration {
                &None => no_play(),
                &Some(ref duration) => {
                    println!("{minutes}.{seconds}",
                             minutes = duration.num_minutes(),
                             seconds = format!("{:02}", (duration.num_seconds() % 60))
                    );
                }
            },
            "title" => match &s.title {
                &None => no_play(),
                &Some(ref title) => { println!("{}", title); }
            },
            _ => ()
        }
    }
}
