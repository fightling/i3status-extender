extern crate serde_json;

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::io::{BufRead,LineWriter,Write};

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

/// Call this function once at program start so that `i3status_ext` can read the header
/// from `stdin` which comes initially from i3status.
pub fn begin<R: BufRead, W: Write>( reader: &mut R, writer: &mut LineWriter<W> ) -> std::io::Result<()>{
    // read first two lines, check and ignore them
    let mut line = String::new();
    reader.read_line(&mut line)?;
    writer.write_all(line.as_bytes())?;
    assert!(line == "{\"version\":1}\n");
    line = String::new();
    reader.read_line(&mut line)?;
    writer.write_all(line.as_bytes())?;
    assert!(line == "[\n");
    Ok(())
}

/// Insert new an item into *i3status*'s json string at given position.
/// Call this within a loop continuously to add your custom item into the json data from *i3status*.
/// #### Parameters
/// - `reader`:
/// - `name`: name of the *i3status* item (could be anything)
/// - `position`: insert item at this position (from left to right)
/// - `reverse`: reverse `position` to count from  right to left.
/// - `what`: text to insert
pub fn update<R: BufRead, W: std::io::Write>( reader: &mut R, writer: &mut LineWriter<W>, name: &str, position: usize, reverse: bool, what: &str) -> std::io::Result<()> {
    // read one line from stdin
    let mut line = String::new();
    reader.read_line(&mut line)?;
    // check if begin() was called
    assert!(line != "{\"version\":1}");
    assert!(line != "[");
    // handle prefix comma
    if line.chars().next().unwrap() == ',' {
        line.remove(0);
        writer.write_all(b",")?;
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
            writer.write_all(format_json(format!("{:?}", items)).as_bytes())?;
        }
        _ => writer.write_all(line.as_bytes())?,
    }
    writer.write_all(b"\n")?;
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
