//! # Source Position Display
//! Helps in displaying source code positions

use anyhow::{anyhow, Result};
use std::fmt::{Display, Formatter};

type BytePosition = usize;

static mut SOURCE_DOCUMENT: Option<String> = None;

/// Things with a position in the source code should implement this
pub trait SourcePosition {
    /// Finds the position of this object in the source code
    fn source_position(&self) -> SourcePositionData;
}

#[derive(Clone, Copy, Debug, PartialEq)]
/// Holds data about a range of source code positions
pub struct SourcePositionData {
    /// Starting byte position
    pub s: BytePosition,
    /// Ending byte position
    pub e: BytePosition,
}

impl Display for SourcePositionData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Unwrap line and column results
        let (Ok(start_line), Ok(start_column), Ok(end_line), Ok(end_column)) = (
            get_line_number(self.s),
            get_column(self.s),
            get_line_number(self.e),
            get_column(self.e),
        ) else {
            return Err(std::fmt::Error {});
        };

        // Write it out
        write!(
            f,
            "[{},{}]-[{},{}]",
            start_line, start_column, end_line, end_column
        )
    }
}

/// Sets the source code document being worked on
pub fn set_document(contents: &String) {
    unsafe { SOURCE_DOCUMENT = Some(contents.clone()) }
}

/// Finds the column of a given source code byte position
fn get_column(position: BytePosition) -> Result<usize> {
    Ok(position - get_line_start(position)? + 1)
}

/// Gets a static reference to the source document in use
fn get_document() -> Result<&'static String> {
    unsafe {
        SOURCE_DOCUMENT
            .as_ref()
            .ok_or(anyhow!("Attempt to read source document which is not set"))
    }
}

/// Finds the line number of a given source code byte position
fn get_line_number(position: BytePosition) -> Result<usize> {
    let line_start = get_line_start(position)?;
    let lines: Vec<char> = get_document()?
        .chars()
        .take(line_start)
        .filter(|c| *c == '\n')
        .collect();

    Ok(lines.len() + 1)
}

/// Finds the start of the line this byte position is on
fn get_line_start(position: BytePosition) -> Result<BytePosition> {
    let chop: Vec<char> = get_document()?.chars().take(position).collect();
    let mut count = 0;

    for c in chop.iter().rev() {
        match c {
            '\n' => break,
            _ => count += 1,
        };
    }

    Ok(position - count)
}
