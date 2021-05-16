# i3status_ext

This is a crate which lets you extend the i3status display as described [here](https://i3wm.org/docs/i3status.html#_external_scripts_programs_with_i3status).

## How to use

First add this crate to your dependencies in you `Cargo.toml` file:

```toml
[dependencies]
i3status_ext = "0.0.3"
```

Then add the external crate into you extension's source file and call `i3status_ext::begin()` once and `i3status_ext::update()` in a loop:

```rust
extern crate i3status_ext;

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
    i3status_ext::begin();

    // prepare some text to insert
    let my_text = "Hello, World!";

    loop {
        // insert your part into i3status
        i3status_ext::update("my_text", position, reverse, my_text);
    }
}
```

The i3status process will keep sending you status updates via `stdin` in which you continue to insert your status item at the wished position.

If you are interested in a more complex example take a look at [i3owm](https://github.com/fightling/i3owm) which inserts information about the current weather into the i3status.

### add your extension to i3 status bar

To activate your extension find the `bar` definition in your i3 configuration file which usually is placed at `~/.config/i3/config`:

```i3
bar {
  status_command i3status | my_own_i3status_extension -rp1"
}
```

### Reference

#### i3status_ext::begin()

Just call this function once at program start so that i3status_ext can read the header which comes initially from i3status.

##### Definition:

`pub fn begin()`

#### i3status_ext::update()

Place this call into a loop like in the example above to continuously add your custom item into the json data from i3status.

##### Definition:

`pub fn update(name: &str, position: usize, reverse: bool, what: &str)`

##### Parameters:

-   `name` : Names your added status item in the json output
-   `position` : Position (beginning with  `0`) of which your item will be placed at within the orignal i3status.
-   `reverse` : If `true` the `position` will be counted backwards from the end.
