#[derive(Debug)]
pub enum LexerError {
    GetCharError,
    MoveError(usize),
    PeekError(usize),
    FontmatterError,
}
