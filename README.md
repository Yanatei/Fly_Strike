# Fly Strike

A small Rust + Bevy arcade game where players shoot flying bubble-like objects to score points. The faster you hit them, the higher your score, and rankings are stored locally for each session.

## Features

- Arcade shooter game built with [Bevy](https://bevyengine.org/) and Rust
- Bubble / flying target objects with score based on reaction speed
- Cannon and bullet mechanics
- Local score ranking for each play session
- Menu and UI
- Supports desktop and Android builds

## Gameplay

- Aim the cannon and fire bullets at flying bubbles
- Destroy targets for score
- Shorter completion time yields higher points
- Score ranking is saved locally for the current run

## Controls

- Desktop: use mouse or keyboard input to aim and shoot
- Mobile / Android: touch controls are supported via `leafwing-input-manager`

## Build and Run

### Desktop

```powershell
cargo build --bin fly_strike
cargo run --bin fly_strike
```

### Android

Prepare Android SDK/NDK and install necessary Rust targets:

```powershell
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
cargo install cargo-ndk
```

Build for Android:

```powershell
cargo clean
cargo ndk -t arm64-v8a build --release --lib
./build-android.bat
```

## Project Structure

- `Cargo.toml` — project metadata and dependencies
- `src/main.rs` — Bevy app startup and window setup
- `src/` — game logic split into plugins
- `assets/` — fonts, images, and sounds used by the game

## Dependencies

- `bevy = "0.18.0"` with `debug` and `wav` features
- `bevy_hanabi = "0.18.0"`
- `leafwing-input-manager = "0.20.0"`
- `rand = "0.9.2"`

## Notes

This project is designed as a small public demo game for sharing on GitHub. The Android build path is included and can be adapted for mobile deployment.

## License

Add a license file if you want to make this project explicitly open source.
