extern crate serde_json;

use regex::Regex;
use serde::{Deserialize, Serialize};

pub mod io;

use io::*;

#[cfg(test)]
mod tests;

/// i3status json entry in a struct like described
/// [here](https://i3wm.org/docs/i3status.html#_external_scripts_programs_with_i3status)
#[derive(Serialize, Deserialize, Debug)]
struct I3StatusItem {
    name: String,
    instance: Option<String>,
    markup: String,
    color: Option<String>,
    full_text: String,
}

// Internal
fn begin_io<IO : Io>(io: &mut IO) -> std::io::Result<()> {
    // read first two lines, check and ignore them
    let line = io.read_line()?;
    io.write_line(&line)?;
    assert!(line == "{\"version\":1}\n");
    let line = io.read_line()?;
    io.write_line(&line)?;
    assert!(line == "[\n");
    return Ok(());
}

/// Call this function once at program start so that `i3status_ext` can pass-through the header
/// from `stdin` which comes initially from i3status.
pub fn begin() -> std::io::Result<StdIo> {
    let mut io = StdIo::new();
    begin_io(&mut io)?;
    return Ok(io);
}

/// dummiy version of `begin()`
pub fn begin_dummy() -> std::io::Result<StdIo> {
    return Ok(StdIo::new());
}

/// Call this function once at program start so that `i3status_ext` can pass-through the header
/// from given String (used for tests).
pub fn begin_str<'a>( input : &'a String) -> std::io::Result<StringInStdOut<'a>> {
    let mut io = StringInStdOut::from_string(&input);
    begin_io(&mut io)?;
    return Ok(io);
}

/// Insert new an item into *i3status*'s json string at given position.
/// Call this within a loop continuously to add your custom item into the json data from *i3status*.
/// #### Parameters
/// - `io`: input and output channels behind `Io` trait
/// - `name`: name of the *i3status* item (could be anything)
/// - `position`: insert item at this position (from left to right)
/// - `reverse`: reverse `position` to count from  right to left.
/// - `what`: text to insert
pub fn update<IO: Io>(
    io: &mut IO,
    name: &str,
    position: usize,
    reverse: bool,
    what: &str,
) -> std::io::Result<()> {
    // read one line from stdin
    let mut line = io.read_line()?;
    // check if begin() was called
    assert!(line != "{\"version\":1}");
    assert!(line != "[");
    // handle prefix comma
    if line.chars().next().unwrap() == ',' {
        line.remove(0);
        io.write_line(",")?;
    }
    // read all incoming entries
    match serde_json::from_str(&line) {
        Ok(i) => {
            let mut items: Vec<I3StatusItem> = i;
            // insert this one
            let w: I3StatusItem = I3StatusItem {
                full_text: what.to_string(),
                markup: "none".to_string(),
                name: name.to_string(),
                instance: None,
                color: None,
            };
            // insert at given position
            if reverse {
                items.insert(items.len() - 1 - position, w);
            } else {
                items.insert(position, w);
            }
            // format output back up json string
            io.write_line(&format_json(format!("{:?}", items)))?;
        }
        _ => io.write_line(&line)?,
    }
    Ok(())
}

/// preprocess output so that i3bar will eat it
fn format_json(line: String) -> String {
    // FIXIT: all the following replacements are needed because I just can not deal
    // with serde_json the right way :/ PLEASE HELP!
    let line = line
        // remove all the 'Item' names
        // thought about using '#[serde(rename = "name")]' but could not make it work
        .replace("I3StatusItem", "")
        // remove optional values which are 'None'
        // tried '#[serde(skip_serializing_if = "Option::is_none")]' but did not work.
        .replace(", color: None", "")
        .replace(", instance: None", "")
        // add quotations arround json names. can you setup serge_json doing that?
        .replace(", full_text:", ",\"full_text\":")
        .replace(", instance:", ",\"instance\":")
        .replace(", color:", ",\"color\":")
        .replace(", markup:", ",\"markup\":")
        .replace("{ name:", "{\"name\":");
    // remove the 'Some()' envelop from all optional values
    let re = Regex::new(r"Some\((?P<v>[^\)]*)\)").unwrap();
    re.replace_all(&line, "$v").to_owned().to_string()
}
