use std::fmt;
use std::fmt::Display;
use std::error::Error;

pub enum ParseErr {
    Empty,
    Malformed
}

// required by error trait
impl Display for ParseErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if true { /// HERE
            write!(f, "Fail to parse todo");
        } else {
            write!(f, "Fail to read todo file");
        }
    }
}

pub struct ReadErr {
    pub child_error: Box<dyn Error>
}

// required by error trait
impl Display for ReadErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

    }
}

impl Error for ParseErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.fmt())
    }
}

impl Error for ReadErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {

    }
}



// source which returns an Option with the error:
// For the ReadErr, it must return the option with the error.
// For the ParseErr, it will return an option which is None if the tasks are empty, and the error if the parsing is malformed.
