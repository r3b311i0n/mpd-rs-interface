extern crate mpd_rs_interface;
extern crate mpd;

use mpd::Client;
use std::net::TcpStream;
use std::env;
use mpd_rs_interface::{get_current_info, next, pause, play, prev, stop, update};


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
                "play" | "s" => play(conn),
                "pause" | "p" => pause(conn),
                "stop" => stop(conn),
                "next" | "ns" => next(conn),
                "prev" | "ps" => prev(conn),
                "update" => update(conn),
                "file" => get_current_info(conn, "file"),
                "stream-name" => get_current_info(conn, "stream-name"),
                "title" | "ct" => get_current_info(conn, "title"),
                "album" | "cal" => get_current_info(conn, "album"),
                "artist" | "cart" => get_current_info(conn, "artist"),
                "duration" => get_current_info(conn, "duration"),
                _ => ()
            }
        }
        _ => { () }
    }
}
