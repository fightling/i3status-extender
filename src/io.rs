use std::io::*;

/// trait for an interface that can read lines from any source and to stdout
pub trait Io {
    fn read_line(&mut self) -> std::io::Result<String>;
    fn write_line(&mut self, line: &str) -> std::io::Result<()>;
}

/// does not read or write anything
pub struct NullIo {}

// create Stdio instance
impl NullIo {
    pub fn new() -> NullIo {
        NullIo{}
    }
}

impl Io for NullIo {
    /// read lines from stdin
    fn read_line(&mut self) -> std::io::Result<String> {
        return Ok(String::new())
    }
    fn write_line(&mut self, _line: &str) -> std::io::Result<()> {
        Ok(())
    }
}

/// reads lines from *stdin* and writes to *stdout*
pub struct StdIo {}

// create Stdio instance
impl StdIo {
    pub fn new() -> StdIo {
        StdIo{}
    }
}

impl Io for StdIo {
    /// read lines from stdin
    fn read_line(&mut self) -> std::io::Result<String> {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line)?;
        return Ok(line)
    }
    fn write_line(&mut self, line: &str) -> std::io::Result<()> {
        std::io::stdout().write_all(line.as_bytes())
    }
}

/// reads lines from `String` and writes to *stdout*
pub struct StringInStdOut<'a> {
    reader: BufReader<&'a[u8]>
}

impl<'a> StringInStdOut<'a> {
    /// create StringStdOutIo instance from String
    pub fn from_string(input: &'a String) -> StringInStdOut<'a> {
        StringInStdOut{ reader: BufReader::new(input.as_bytes()) }
    }
}

impl<'a> Io for StringInStdOut<'a> {
    /// read lines from String
    fn read_line(&mut self) -> std::io::Result<String>  {
        let mut line = String::new();
        self.reader.read_line(&mut line)?;
        return Ok(line);
    }
    fn write_line(&mut self, line: &str) -> std::io::Result<()> {
        std::io::stdout().write_all(line.as_bytes())
    }
}
