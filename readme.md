# Lumberjack
By Hevanafa (12-12-2022)

A basic text-based game with SFX player.

**Stack:** Rust + Pancurses + Kira

`pancurses`: working `ncurses` port for Windows

I chose `kira` for the audio player because `rodio` was too hard to use, especially the case where I want to reuse the audio stream.

# How to Start
1. Clone the repo
2. `cargo build --release`
3. `.\target\release\rand_trees`

# How to Play
Walk with the arrow keys.  Walk into the trees to cut them.
