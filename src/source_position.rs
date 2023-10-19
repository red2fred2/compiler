use std::fmt::Display;

type BytePosition = usize;

static mut SOURCE_DOCUMENT: Option<String> = None;

pub trait SourcePosition {
    fn source_position(&self) -> SourcePositionData;
}

pub fn set_document(contents: &String) {
    unsafe { SOURCE_DOCUMENT = Some(contents.clone()) }
}

fn get_column(pos: BytePosition) -> usize {
    pos - get_line_start(pos) + 1
}

fn get_document() -> &'static String {
    unsafe { SOURCE_DOCUMENT.as_ref().unwrap() }
}

fn get_line_number(pos: BytePosition) -> usize {
    let line_start = get_line_start(pos);
    let lines: Vec<char> = get_document()
        .chars()
        .take(line_start)
        .filter(|c| *c == '\n')
        .collect();

    lines.len() + 1
}

fn get_line_start(pos: BytePosition) -> BytePosition {
    let chop: Vec<char> = get_document().chars().take(pos).collect();
    let mut count = 0;

    for c in chop.iter().rev() {
        match c {
            '\n' => break,
            _ => count += 1,
        };
    }

    pos - count
}

#[derive(Clone, Debug, PartialEq)]
pub struct SourcePositionData {
    pub s: BytePosition,
    pub e: BytePosition,
}

impl Display for SourcePositionData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{},{}]-[{},{}]",
            get_line_number(self.s),
            get_column(self.s),
            get_line_number(self.e),
            get_column(self.e)
        )
    }
}
