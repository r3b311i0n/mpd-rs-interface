#[macro_use]
extern crate serde_derive;

extern crate mpd;
extern crate serde;
extern crate serde_json;

use mpd::Client;

// TODO: Implement JSON serializable for info.

#[derive(Serialize, Debug)]
struct JsonSong {
    title: String,
    album: String,
    artist: String,
    duration: String,
}

pub fn play(mut conn: Client) { conn.play().unwrap(); }

pub fn pause(mut conn: Client) { conn.toggle_pause().unwrap(); }

pub fn stop(mut conn: Client) { conn.stop().unwrap(); }

pub fn next(mut conn: Client) { conn.next().unwrap(); }

pub fn prev(mut conn: Client) { conn.prev().unwrap(); }

pub fn update(mut conn: Client) { conn.rescan().unwrap(); }

pub fn get_current_info(mut conn: Client, tag: &str) {
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
