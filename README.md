# Proxysette
[![forthebadge](https://forthebadge.com/images/badges/made-with-rust.svg)](https://forthebadge.com) [![Say Thanks!](https://img.shields.io/badge/Say%20Thanks-!-1EAEDB.svg)](https://saythanks.io/to/davidepaci)

Proxysette is an easy to use datasette emulator for the original Commodore 64.

By using Proxysette you can run pretty much any software you find available for Commodore 64 on your actual Commodore 64, provided you are using a car cassette adapter or can write on tape!

**Written in Rust, it currently runs on Windows platforms only. Linux/macOS support coming in the future.**

Simply feed it any kind of C64 image file (`.tap`, `.t64`, `.prg`, `.d64`, even if stored in `.zip`) and Proxysette will automatically convert it to a `.wav` file which you can then play while your system is connected to the car cassette adapter or you can write on your tape cassette.

Proxysette also stores the converted `.wav` files to playback whenever you want.

Basically TapDancer for PC.
# Installation
- Downloaded the latest [Release](https://github.com/davidepaci/Proxysette/releases) and extract it
## How to Use
- Open up your computer's terminal
- Run this command: `.\proxysette <file path>`
- Converted files will be in `converted` folder inside the same folder as where the tool is stored
# Supported Formats
- `.tap`
- `.prg`
- `.t64`
- `.zip`
# Roadmap and Current Status
Proxysette is currently in **active development** and is not complete yet. Feel free to contribute!

The tool is meant to have playback functionality, `.d64` support, Linux/macOS support and a functional GUI.

All planned features are in the [Issues](https://github.com/davidepaci/Proxysette/issues) section.
# Contribution
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.
# License
Proxysette is available as open source under the terms of the [MIT License](https://opensource.org/license/mit/). See the [LICENSE.md](https://github.com/davidepaci/Proxysette/blob/main/LICENSE) file for details
# Acknowledgments
- [Audiotap](https://wav-prg.sourceforge.io/audiotap.html) by Fabrizio Gennari
- [WAV-PRG](https://wav-prg.sourceforge.io/wavprg.html) by Fabrizio Gennari
