extern crate gcc;

fn main() {
    gcc::Build::new()
        .file("src/mpd_utils.c")
        .flag("-lmpdclient")
        .compile("libmpdutils.a")
}
