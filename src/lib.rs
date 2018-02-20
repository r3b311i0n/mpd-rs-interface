extern crate mpd;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

use mpd::Client;


#[derive(Serialize, Debug)]
struct JsonSong {
    file: String,
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

pub fn update(mut conn: Client) -> String {
    let db_scan = conn.rescan();
    match db_scan {
        Ok(_) => String::from("Updating Database"),
        Err(e) => e.to_string()
    }
}

pub fn get_song(mut conn: Client) -> String {
    let song = JsonSong {
        file: get_tag(&mut conn, "file"),
        title: get_tag(&mut conn, "title"),
        album: get_tag(&mut conn, "album"),
        artist: get_tag(&mut conn, "artist"),
        duration: get_tag(&mut conn, "duration"),
    };

    return serde_json::to_string(&song).unwrap();
}

pub fn get_tag(conn: &mut Client, tag: &str) -> String {
    let song = conn.currentsong().unwrap();

    let no_play = String::from("Nothing is playing!");

    match &song {
        &None => return no_play.to_owned(),
        &Some(ref s) => match tag {
            "file" => return s.file.to_owned(),
            "album" => match &s.tags.get("Album") {
                &None => return String::from("Album not found!").to_owned(),
                &Some(album) => return album.to_owned(),
            },
            "artist" => match &s.tags.get("Artist") {
                &None => return String::from("Artist not found!").to_owned(),
                &Some(artist) => return artist.to_owned(),
            },
            "duration" => match &s.duration {
                &None => return no_play.to_owned(),
                &Some(ref duration) => {
                    return format!("{minutes}.{seconds}",
                                   minutes = duration.num_minutes(),
                                   seconds = format!("{:02}", (duration.num_seconds() % 60))
                    ).to_owned();
                }
            },
            "title" => match &s.title {
                &None => return no_play.to_owned(),
                &Some(ref title) => return title.to_owned()
            },
            _ => return String::from("🐄🦄").to_owned()
        }
    }
}
