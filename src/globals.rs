use std::fmt;

#[derive(Debug)]
pub enum LogLevel {
    NONE,
    ERROR,
    WARN,
    DEBUG,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}
