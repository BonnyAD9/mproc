# mproc
Measure process run time and peak memory usage.

Some code was inspired by [robotty/simple-process-stats](https://github.com/robotty/simple-process-stats)

## Support
- Windows
- Linux

MacOS is not supported because I have no way to test it.

## Usage
Show help:
```
mproc
```

Run program (with arguments):
```
mproc [AppName] [Arguments]
```

## Example
Run `meme-cutter` with arguments `file`, `image.png` and `result.png`.
```
 > mproc meme-cutter file image.png result.png
===============<< mproc results >>===============
Time: 5.3713ms

Memory: 3.836 MiB

Exit code: 0
```

## Links
- **Author:** [BonnyAD9](https://github.com/BonnyAD9)
- **GitHub repository:** [BonnyAD9/mproc](https://github.com/BonnyAD9/Bny.General)
- **My website:** [bonnyad9.github.io](https://bonnyad9.github.io/)
