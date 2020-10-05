# Ascii-Art

Converts images to ASCII based on pixel brightness and prints the result.

Based on blog post [Programming Projects for Advanced Beginners #1: ASCII art](https://robertheaton.com/2018/06/12/programming-projects-for-advanced-beginners-ascii-art/) by [Robert Heaton](https://robertheaton.com)

## Usage

`ascii-art [FLAGS] [OPTIONS] <image>`

```bash
FLAGS:
    -h, --help       Prints help information
    -i, --invert     Inverts image
    -V, --version    Prints version information

OPTIONS:
    -b, --brightness <brightness>     [default: average]  [possible values: Average, MinMax, Luma]
    -o, --output <output>            Output type of image [default: ascii]  [possible values: Ascii, Matrix, Color]
    -w, --width <width>              Width of output [default: 90]

ARGS:
    <image>    Path to image
```

## TODO

- Clean up code
- Fix invert flag for color output
- Combine printing loops into single iter
