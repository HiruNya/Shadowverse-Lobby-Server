# Shadowverse-Lobby-Server
(This is the server version NOT the client that the normal user would use).<br>
Furthermore this a big work in progress as not even a client to use this has been made.

A lobby to post your private match codes in Shadowverse to play against other people.<br>
This is a server that is meant to accept TCP connections from the client and create, remove, update, and send a list of currently available private games that can be used by others in events or quests where they have to play a private match to get the reward. (For example the "Play 1 private match" daily mission).

This is currently built in Rust on the Tokio framework and may require the nightly compiler. (Not sure about this).

## Client
The Client is not yet available and therefore this server is pretty much useless until one is made available.

## Host your own server
You can host your own server by either:
- [Downloading the program in the Release section](https://github.com/HiruNya/Shadowverse-Lobby-Server/releases)
- Building the server from the source

### How to build from source
To do this both the Rust compiler (with the nightly toolchain) and the package manager, Cargo, are required.
1. Clone this repo from git:```git clone https://github.com/HiruNya/Shadowverse-Lobby-Server/```
2. Build this by doing ```cargo build +nightly --release``` or run it straight after you build it by doing ```cargo run +nightly --release```
