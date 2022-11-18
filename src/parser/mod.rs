use pest::Parser;

#[derive(Parser)]
#[grammar = "psf_ascii.pest"]
pub struct PsfAsciiParser;
