# ASCIICAM

Totally bare bones and only supports Linux with `Video4Linux` support. Requires 720p resolution and 30fps.

# Run

Note: To quit the program you have to terminate the terminal session, there are no keybinds **yet**!

```sh
cargo run --release 2> /dev/null
```

Pipe `stderr` into `/dev/null` because I just wanted to get something to work so the logging behaviour is not customizable yet.
There might be more backends in the future (render with OpenGL or w/e).

# Future

I will definitely add support for proper keybinds like quitting with 'q', a way to switch between different ASCII tables, adjusting the threshold and inverting the image.
