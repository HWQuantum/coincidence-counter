# Coincidence counting library
This is a rust wrapper for PicoQuant's Hydra Harp 400 library. <https://www.picoquant.com/products/category/tcspc-and-time-tagging-modules/hydraharp-400-multichannel-picosecond-event-timer-tcspc-module>
It produces a python library when compiled, which can be used in python programs directly.

# Requirements
Rust (nightly version)
HydraHarp library version 3.*

# Building Instructions
## Linux
### Nix
If you're using the nix package manager, there's a shell.nix file which produces an environment with the hydraharp lib available, and rust so you just need to run 
`cargo build --release` to produce the python library.
### No Nix
You need to install the hydraharp library, and hopefully it'll just build
## Windows
You'll probably need to install microsoft visual studio build tools. 
Go to [this page](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2017), scroll down to the section "Tools for Visual Studio 2019", and click download on the "Build tools for visual studio 2019".
Then install this - default options should be fine (I think?).

Get and install [rust](https://rustup.rs/).

Get the hydra harp library and install it to the folder `C:\Program Files\PicoQuant\HydraHarp-HHLibv30` (this is where the linker searches).

Run `cargo build --release` from this folder, and hopefully the library will build.

If it's built correctly, you'll find it in `coincidence-counter\target\release\` named something like `hhlib.dll`. Remove the `.dll` part and replace it with `.pyd`, and you can just copy that file into your python library, and import it like a normal python thing.

If you want documentation of the functions in this library, you can run `cargo doc --open`. This'll show you the types and functions, etc. that it provides.
