cmake_minimum_required(VERSION 3.10)
project(mpd_rs_interface)

set(CMAKE_CXX_STANDARD 11)

add_executable(mpd_rs_interface
        src/c/mpd_utils.c src/c/wrapper.h)

target_link_libraries(mpd_rs_interface mpdclient)
