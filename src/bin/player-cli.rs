extern crate mpd_rs_interface;

extern crate mpd;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

use mpd::Client;
use std::net::TcpStream;
use std::env;
use std::fs::File;
use std::io::Write;
use mpd_rs_interface::{get_tag, next, pause, play, prev, stop, update};

#[derive(Serialize, Deserialize, Debug)]
struct Conf {
    mpd_host: String,
    mpd_port: String,
}


fn main() {
    let conn: Client<TcpStream> = Client::connect(get_conf()).unwrap();
    parse_cmd_args(conn);
}

fn get_conf() -> String {
    let conf_path = "./mpd_rsi.json";
    let conf_file = File::open(&conf_path);
    let conf: Conf;

    match conf_file {
        Ok(json) => conf = serde_json::from_reader(json).unwrap(),
        Err(_) => conf = || -> Conf {
            let mut new_file = File::create(&conf_path).unwrap();
            let new_conf = Conf {
                mpd_host: "127.0.0.1".to_owned(),
                mpd_port: "6600".to_owned(),
            };
            let conf_json = serde_json::to_string(&new_conf).unwrap();

            println!("Configuration file not found!\nCreating a new one...\n");
            new_file.write_all(&conf_json.as_bytes()).unwrap();

            return new_conf;
        }(),
    }

    return format!("{}:{}", &conf.mpd_host, &conf.mpd_port);
}

fn get_current_info(mut conn: Client) {
    let title = get_tag(&mut conn, "title");
    let album = get_tag(&mut conn, "album");
    let artist = get_tag(&mut conn, "artist");
    let duration = get_tag(&mut conn, "duration");
    let file = get_tag(&mut conn, "file");
    println!("{}\n{}\n{}\n{}\n{}\n", title, album, artist, duration, file);
}

fn show_help() {
    println!(
        "h ⇾ Show This\n\
        play | s ⇾ Play\n\
        pause | p ⇾ Pause\n\
        stop ⇾ Stop\n\
        next | ns ⇾ Next Song\n\
        prev | ps ⇾ Previous Song\n\
        update ⇾ Update Database\n\
        info | i ⇾ Get Current Song's Information\n"
    );
}

fn parse_cmd_args(conn: Client) {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => show_help(),
        2 => {
            let cmd = &args[1].to_lowercase();
            match &cmd[..] {
                "h" => show_help(),
                "play" | "s" => play(conn),
                "pause" | "p" => pause(conn),
                "stop" => stop(conn),
                "next" | "ns" => next(conn),
                "prev" | "ps" => prev(conn),
                "update" => update(conn).clear(),
                "info" | "i" => get_current_info(conn),
                _ => ()
            }
        }
        _ => { () }
    }
}
