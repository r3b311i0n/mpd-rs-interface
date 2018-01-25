#include <mpd/status.h>
#include <mpd/song.h>
#include <mpd/connection.h>
#include <mpd/client.h>
#include <stdlib.h>
#include <stdio.h>

const char *get_title() {
    struct mpd_connection *conn;
    struct mpd_status *status;
    struct mpd_song *song;
    enum mpd_state state;
    const char *NAME;

    unsigned idx = 0;
    enum mpd_tag_type title = MPD_TAG_TITLE;

    conn = mpd_connection_new(NULL, 0, 0);
    status = mpd_run_status(conn);
    if (status == NULL) {
        return "COULDN'T GET STATUS!";
    }
    if (!mpd_send_command(conn, "currentsong", NULL)) {
        return "SOMETHING WENT WRONG! COULDN'T GET CURRENT SONG!";
    }
    song = mpd_recv_song(conn);
    if (song == NULL) {
        return "COULDN'T GET SONG OBJECT!";
    }
    state = mpd_status_get_state(status);
    mpd_status_free(status);
    mpd_connection_free(conn);

    if (state > 1) {
        NAME = mpd_song_get_tag(song, title, idx);
        mpd_song_free(song);
        if (NAME == NULL) {
            return "Title not found!";
        } else {
            printf(NAME);
            return NAME;
        }
    }

    return NULL;
}

void goodbye(void *now_playing) {
    free(now_playing);
}
