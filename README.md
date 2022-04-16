# minecraft-rs

A very minimal and fast minecraft server implementation, written in Rust, created to be used in CTF VMs/Containers.

## Building

Make sure you have installed rust & cargo before building this project.
All you need to do to build this project is to run
```shell
cargo build --release
```
The build output is inside target directory `./target/release/minecraft-rs`

## Project scope

This project isn't a full minecraft implementation, it should be a minimal implementation, that allows players to ping the server, connect, move and chat with each other. Physics, terrain generation etc. are not part of this project.  
  
Feel free to fork this repository, and add those features, but PRs adding stuff out of this project's scope will probably not be merged.

## Useful links
- [Minecraft protocol wiki](https://wiki.vg)
- [Rust VarInt/VarLong implementation](https://github.com/luojia65/mc-varint) (`Writer.write_var_i32(0)` doesn't work)
- [LiveOverflow's Minecraft:HACKED series](https://youtube.com/playlist?list=PLhixgUqwRTjwvBI-hmbZ2rpkAl4lutnJG)
