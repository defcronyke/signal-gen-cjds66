use clap;
use std::fmt;

pub type ErrorKind = clap::ErrorKind;

pub fn get_code(kind: ErrorKind) -> i32 {
    match kind {
        ErrorKind::InvalidValue                  => 0b_00000000000000000001_i32,
        ErrorKind::UnknownArgument               => 0b_00000000000000000011_i32,
        ErrorKind::InvalidSubcommand             => 0b_00000000000000000111_i32,
        ErrorKind::UnrecognizedSubcommand        => 0b_00000000000000001111_i32,
        ErrorKind::EmptyValue                    => 0b_00000000000000011111_i32,
        ErrorKind::ValueValidation               => 0b_00000000000000111111_i32,
        ErrorKind::TooManyValues                 => 0b_00000000000001111111_i32,
        ErrorKind::TooFewValues                  => 0b_00000000000011111111_i32,
        ErrorKind::WrongNumberOfValues           => 0b_00000000000111111111_i32,
        ErrorKind::ArgumentConflict              => 0b_00000000001111111111_i32,
        ErrorKind::MissingRequiredArgument       => 0b_00000000011111111111_i32,
        ErrorKind::MissingSubcommand             => 0b_00000000111111111111_i32,
        ErrorKind::MissingArgumentOrSubcommand   => 0b_00000001111111111111_i32,
        ErrorKind::UnexpectedMultipleUsage       => 0b_00000011111111111111_i32,
        ErrorKind::InvalidUtf8                   => 0b_00000111111111111111_i32,
        ErrorKind::HelpDisplayed                 => 0b_00001111111111111111_i32,
        ErrorKind::VersionDisplayed              => 0b_00011111111111111111_i32,
        ErrorKind::ArgumentNotFound              => 0b_00111111111111111111_i32,
        ErrorKind::Io                            => 0b_01111111111111111111_i32,
        ErrorKind::Format                        => 0b_11111111111111111111_i32,
    }
}

pub fn handle_exit(res: Result<i32, Error>) -> Result<i32, Error> {
    res.map_err(
        |e| {
            match e.kind {
                ErrorKind::HelpDisplayed => {
                    println!("{}", e.message);
                    e
                },

                ErrorKind::VersionDisplayed => {
                    println!("{}", e.message);
                    e
                },

                _ => {
                    println!("{}", e);
                    e
                },
            }
        }
    )
}

#[derive(Debug)]
pub struct Error {
    pub message:    String,
    pub info:       Option<Vec<String>>,
    pub kind:       ErrorKind,
    pub code:       i32,
    pub name:       String
}

impl Error {
    pub fn new<'a>(message: &'a str, info: Option<Vec<String>>, kind: ErrorKind) -> Self {
        let message = String::from(message);
        let code = get_code(kind);
        let name = format!("{:?}", kind);

        Self { message, info, kind, code, name }
    }
}

pub trait From {
    fn from_clap_error(val: clap::Error) -> Self;
    fn with_description(message: &str, kind: clap::ErrorKind) -> Self;
}

impl From for Error {
    fn from_clap_error(e: clap::Error) -> Self {
        Error::new(&e.message, e.info, e.kind)
    }

    fn with_description(message: &str, kind: clap::ErrorKind) -> Self {
        let e = clap::Error::with_description(message, kind);
        Error::new(&e.message, e.info, e.kind)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.info.is_some() {
            write!(f, "\nerror ({} {}): {}: {:?}\n", self.name, self.code, self.message, self.info.clone().unwrap_or_default())
        } else {
            write!(f, "\nerror ({} {}): {}\n", self.name, self.code, self.message)
        }
    }
}
