pub type PrintResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    //  Command name, line number
    InvalidCommandInLine(Option<String>, Option<usize>),
    UnsupportedCommand(String),
    InputOutputError(std::io::Error),
}
