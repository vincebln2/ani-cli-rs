# ani-cli-rs

A command-line anime streaming tool written in Rust.
Inspired by [ani-cli](https://github.com/pystardust/ani-cli)

## Highlights

![alt text](https://github.com/vincebln2/ani-cli-rs/showcase/frierenshowcase.png "example of running in terminal")

- Lets you search for anime by name in the terminal
- Choose between sub/dub options
- Interactive episode list
- Select a stream provider launched directly in mpv

## How to use after installing

- run ./ani-cli-rs in the terminal
- Follow prompts in terminal to select the anime to watch
- Enjoy!
- Quit using terminal menu

## How to install

- Install rust if not already installed
- Clone this repo, build using cargo

```bash
    git clone https://github.com/vincebln2/ani-cli-rs.git
    cd ani-cli-rs
    cargo build --release
```

- Move the binary somewhere in path, or run from the src/ directory.

## Dependencies

- mpv (terminal video player)
- rust toolchain (rustup)
