pub type RunningResult<T> = Result<T, RunningError>;

#[derive(Debug)]
pub enum RunningError {
    ParserError(ParserError),
    IoError(std::io::Error),
}

impl std::fmt::Display for RunningError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            RunningError::ParserError(ref e) => e.fmt(f),
            RunningError::IoError(ref e) => e.fmt(f),
        }
    }
}

impl std::error::Error for RunningError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self {
            RunningError::ParserError(ref e) => Some(e),
            RunningError::IoError(ref e) => Some(e),
        }
    }
}

impl std::convert::From<ParserError> for RunningError {
    fn from(e: ParserError) -> Self {
        RunningError::ParserError(e)
    }
}

impl std::convert::From<std::io::Error> for RunningError {
    fn from(e: std::io::Error) -> Self {
        RunningError::IoError(e)
    }
}

pub struct ParserError {
    pub message: String,
}

impl ParserError {
    pub fn new(msg: impl ToString) -> Self {
        Self {
            message: msg.to_string(),
        }
    }
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ file: {}, line: {}, message: {} }}",
            file!(),
            line!(),
            self.message
        )
    }
}

impl std::fmt::Debug for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{{ file: {}, line: {}, message: {} }}",
            file!(),
            line!(),
            self.message
        )
    }
}

impl std::error::Error for ParserError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
