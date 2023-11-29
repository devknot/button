
use core::fmt;

#[derive(Debug)]
enum Error {
    Button,
    Draw,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Button => "button",
            Self::Draw => "Draw",
        })
    }
}


