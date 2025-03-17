# pacman4console in Rust 

<img align="right" src=https://github.com/user-attachments/assets/4e2f7e63-007b-4bd6-b2b6-e3a63915359c width="45%">

This repository is a Rust implementation of the **pacman4console** game originally written in C. The original project can be found at: [pacman4console](https://github.com/YoctoForBeaglebone/pacman4console).

### Running the Game:

### Debug Mode
To run the game in debug mode, use the following command:
```sh
cargo run -- <level number>
```
> [!Note]
> For example, to play level 9: `cargo run -- 9`

### Build and Run in Release Mode
To build the game in release mode, run:
```sh
cargo build --release
```

Then, navigate to the release directory and run the game:
```sh
cd ./target/release
./pacman <level number>
```
> [!Note]
> For example, to play level 9: `./pacman 9`

> [!Warning]
> The `Levels/` directory that exists in the `debug/` or `release/` directory **must be placed beside the `pacman` executable** when running the game. Ensure the level files are correctly positioned, or the game may not function as expected.

> [!Note]
> ### AI Assistance
> I used GPT chat assistance in parts of this project, and parts of this project were developed by AI.

