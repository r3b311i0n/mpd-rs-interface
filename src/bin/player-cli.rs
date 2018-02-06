extern crate mpd_rs_interface;
extern crate mpd;

use mpd::Client;
use std::net::TcpStream;
use std::env;
use mpd_rs_interface::{get_tag, next, pause, play, prev, stop, update};


fn main() {
    let conn: Client<TcpStream> = Client::connect("127.0.0.1:6600").unwrap();
    parse_cmd_args(conn);
}

fn get_current_info(mut conn: Client) {
    let title = get_tag(&mut conn, "title");
    let album = get_tag(&mut conn, "album");
    let artist = get_tag(&mut conn, "artist");
    let duration = get_tag(&mut conn, "duration");
    let file = get_tag(&mut conn, "file");
    println!("{}\n{}\n{}\n{}\n{}\n", title, album, artist, duration, file);
}

fn parse_cmd_args(conn: Client) {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let cmd = &args[1].to_lowercase();
            match &cmd[..] {
                "play" | "s" => play(conn),
                "pause" | "p" => pause(conn),
                "stop" => stop(conn),
                "next" | "ns" => next(conn),
                "prev" | "ps" => prev(conn),
                "update" => update(conn),
                "info" | "i" => get_current_info(conn),
//                "file" => get_current_info(conn, "file"),
//                "title" | "ct" => get_current_info(conn),
//                "album" | "cal" => get_current_info(conn, "album"),
//                "artist" | "cart" => get_current_info(conn, "artist"),
//                "duration" => get_current_info(conn, "duration"),
                _ => ()
            }
        }
        _ => { () }
    }
}
