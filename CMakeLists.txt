cmake_minimum_required(VERSION 3.10)
project(mpd_np)

set(CMAKE_CXX_STANDARD 11)

add_executable(mpd_np
        src/c/mpd_utils.c src/c/wrapper.h)

target_link_libraries(mpd_np mpdclient)
