# GamePad Controlled Launcher

This is a simple application launcher designed to be a home screen for my home media center.
It displays a clock and a row of applications to run.
It also respects gamepad inputs:
d-pad buttons work as arrow keys and the bottom action button works as enter.

![Screenshot](screenshot.png)

*Now the project is on prototyping stage*

## Build

The launcher is written in Rust, so it is built with `cargo`.
But it also depends on Qt6 and a proper qmake version of `qmake`.
I build it with the command: `QMAKE=qmake6 cargo build`.

The minimum Qt version is `6.5.0`.

## Configure

The config file should be placed at `$XDG_CONFIG_HOME/gcpl.toml`.
Now only the applications list can be configured.
Example:
```toml
[[items]]
name = "Steam"
icon = "/usr/share/icons/hicolor/256x256/apps/steam.png"
exec = "steam -gamepadui"

[[items]]
name = "Kodi"
icon = "/usr/share/icons/hicolor/256x256/apps/kodi.png"
exec = "kodi"
```