//! ```
//! use inputlib::input;
//!
//! fn main() {
//!     let name = input!("What's your name? ");
//!     println!("Hello, {}!", name);
//! }
//! ```

use std::io::{self, stdin, stdout, Write};

/// Read a line of text from stdin after writing the provided prompt.
///
/// The returned string is trimmed of a trailing newline/carriage-return.
pub fn read_input(prompt: &str) -> io::Result<String> {
    let mut stdout = stdout();
    stdout.write_all(prompt.as_bytes())?;
    stdout.flush()?;

    let mut line = String::new();
    stdin().read_line(&mut line)?;
    Ok(trim_newline(line))
}

fn trim_newline(mut input: String) -> String {
    if input.ends_with('\n') {
        input.pop();
        if input.ends_with('\r') {
            input.pop();
        }
    }
    input
}

#[cfg(test)]
fn read_input_from<R: Read, W: Write>(prompt: &str, input: &mut R, output: &mut W) -> io::Result<String> {
    output.write_all(prompt.as_bytes())?;
    output.flush()?;

    let mut line = String::new();
    input.read_to_string(&mut line)?;
    Ok(trim_newline(line))
}

#[macro_export]
macro_rules! input {
    ($prompt:expr $(, $args:expr)* $(,)?) => {{
        let prompt = format!($prompt $(, $args)*);
        $crate::read_input(&prompt).expect("Failed to read input")
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn read_input_trims_newline() {
        let mut input = Cursor::new(b"hello\n" as &[u8]);
        let mut output = Vec::new();
        let result = read_input_from("Enter: ", &mut input, &mut output).unwrap();

        assert_eq!(result, "hello");
        assert_eq!(String::from_utf8(output).unwrap(), "Enter: ");
    }
}
