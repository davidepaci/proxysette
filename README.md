features:
- convert any file (.t64, .tap, .d64, .prg) to .wav using audiotap/wavprg
- extract files from .zip
- play .wav at x volume
- adjust x volume
- saves converted .wavs for instant playback
- instructions (car cassette adapter, press C=, wait, then type RUN)

converts:
- .tap audiotap
- .prg wav-prg
- .t64 wav-prg
- .d64 to .prg dirmaster

main workflow:
- get file
- check format
- if zip extract and find format (get first in order)
- convert to wav
- save on folder


- playback
- allow volume adjustment
- pause/stop playback
- list already converted wavs
- select already converted wavs for playback
- open instructions

todo:
- gui
- add c64ntsc support (only taps work for now)
- store user settings (platform for prg and t64, volume, converted software)
- document
- print only in debug mode
- doesn't sign wav
- proper readme + credits
- semantic versioning
- code refactor; increase quality
- lint code
- linux and mac portability
- find out what's that %appdata% stuff programs always use to store stuff?
- wiki
- proper readme (instructions, credits), logo, stats, etc
- packages? releases? projects?
- deployments? automatic authors? automatic documentation on wiki? automatic testing before pull request?
- ask opensource contribution to issues
- release on forums/sourceforge?