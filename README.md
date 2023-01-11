# whichgpu - An Intel MacBook Pro GPU Query Tool

This little command line tool shows whether the integrated or discrete GPU is
used on an Intel-based MacBook Pro, or other Apple hardware that utilizes
dynamic GPU switching.

To use, first ensure you have a Rust compiler installed, and then simply run
`cargo install --git https://github.com/thedjinn/whichgpu.git` to install the
tool, and then run `whichgpu`. Alternatively, you can clone the repository and
run `cargo install`.

Note that this tool has not been tested on Apple Silicon hardware and so the
message it prints on these devices is unknown. However, since these devices
only have a single powerful GPU, this tool is not very relevant on such a
machine anyway.

## Example:

```
$ whichgpu
Currently using the integrated GPU
```
