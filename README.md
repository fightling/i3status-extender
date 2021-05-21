# i3status_ext [![Rust](https://github.com/fightling/i3status_ext/actions/workflows/rust.yml/badge.svg)](https://github.com/fightling/i3status_ext/actions/workflows/rust.yml)

This is a crate which lets you extend the *i3status* display as described [here](https://i3wm.org/docs/i3status.html#_external_scripts_programs_with_i3status).

## How to use

First add this crate to your dependencies in you `Cargo.toml` file:

```toml
[dependencies]
i3status_ext = "0.0.9"
```

To compile the following sample code you will need to add the `clap` program arguments crate too.

```toml
clap = "3.0.0-beta.2"
```

Then add the external crate into you extension's source file and call `i3status_ext::begin()` once and `i3status_ext::update()` in a loop:

```rust
extern crate i3status_ext;
use clap::App;

fn main() {

    // read arguments
    let args = App::new("myext").args(&[
        Arg::new("position")
            .short('p')
            .takes_value(true),
        Arg::new("reverse")
            .short('r'),
    ]);

    // get arguments (with minimal plausibility check)
    let position = args
        .value_of("position")
        .unwrap_or("0")
        .parse::<usize>()
        .unwrap_or(0);
    let reverse = args.is_present("reverse");

    // start reading i3status' output from stdin
    let mut io = i3status_ext::begin();

    // prepare some text to insert
    let my_text = "Hello, World!";

    loop {
        // insert your part into i3status
        i3status_ext::update(&mut io, "my_text", position, reverse, my_text);
    }
}
```

The i3status process will keep sending you status updates via `stdin` in which you continue to insert your status item at the wished position.

If you are interested in a more complex example take a look at [i3owm](https://github.com/fightling/i3owm) which inserts information about the current weather into the i3status.

### add your extension to i3 status bar

To activate your extension find the `bar` definition in your i3 configuration file which usually is placed at `~/.config/i3/config`:

```i3
bar {
  status_command i3status | my_own_i3status_extension -rp1
}
```

## Reference Documentation

Beside this introduction there is a reference documentation which can be found [here](https://docs.rs/i3status_ext).

## Links

### Website

This README tastes better at [i3status_ext.thats-software.com](http://i3status_ext.thats-software.com).

### *github* repository

For the source code see [this repository](https://github.com/fightling/i3status_ext) at *github.com*.

### on *crates.io*

Published at [*crates.io*](https://crates.io/crates/i3status_ext).

## License

i3status_ext is licensed under the *MIT license* (LICENSE-MIT or http://opensource.org/licenses/MIT)
