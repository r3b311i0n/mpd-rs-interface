#include <mpd/status.h>
#include <mpd/song.h>
#include <mpd/connection.h>
#include <mpd/client.h>

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
    if (!status) {
        return NULL;
    }
    if (!mpd_send_command(conn, "currentsong")) {
        return NULL;
    }
    song = mpd_recv_song(conn);
    if (!song) {
        return NULL;
    }
    state = mpd_status_get_state(status);
    mpd_status_free(status);
    mpd_connection_free(conn);

    if (state > 1) {
        NAME = mpd_song_get_tag(song, title, idx);
        return NAME;
    }

    return NULL;
}

//int main(void) {
//    struct mpd_connection *conn;
//    struct mpd_status *status;
//    struct mpd_song *song;
//    enum mpd_state state;
//
//    conn = mpd_connection_new(NULL, 0, 0);
//    status = mpd_run_status(conn);
//    if (!status) return 0;
//    if (!mpd_send_command(conn, "currentsong")) return 1;
//    song = mpd_recv_song(conn);
//    if (!song) return 0;
//    state = mpd_status_get_state(status);
//    mpd_status_free(status);
//    mpd_connection_free(conn);
//
//    if (state > 1) {
//        printf("%d\n%s\n%s\n",
//               mpd_status_get_song_id(status),
//               mpd_song_get_uri(song),
//               getTitle(song)
//        );
//    }
//
//    return 0;
//}

