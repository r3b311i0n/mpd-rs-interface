cmake_minimum_required(VERSION 3.10)
project(mpd-rs-interface)

set(CMAKE_CXX_STANDARD 11)

add_executable(mpd-rs-interface
        src/c/mpd_utils.c src/c/wrapper.h)

target_link_libraries(mpd-rs-interface mpdclient)
