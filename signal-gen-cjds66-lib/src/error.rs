use clap;
use clap::ErrorKind;
use clap::ErrorKind::*;
use std::fmt;

pub fn get_code(kind: ErrorKind) -> i32 {
    match kind {
        InvalidValue                  => 1,
        UnknownArgument               => 2,
        InvalidSubcommand             => 3,
        UnrecognizedSubcommand        => 4,
        EmptyValue                    => 5,
        ValueValidation               => 6,
        TooManyValues                 => 7,
        TooFewValues                  => 8,
        WrongNumberOfValues           => 9,
        ArgumentConflict              => 10,
        MissingRequiredArgument       => 11,
        MissingSubcommand             => 12,
        MissingArgumentOrSubcommand   => 13,
        UnexpectedMultipleUsage       => 14,
        InvalidUtf8                   => 15,
        HelpDisplayed                 => 16,
        VersionDisplayed              => 17,
        ArgumentNotFound              => 18,
        Io                            => 19,
        Format                        => 20,
    }
}

pub fn handle_exit(res: Result<i32, Error>) -> Result<i32, Error> {
    let suppress_errors = [
        HelpDisplayed,
        VersionDisplayed,
        MissingArgumentOrSubcommand,
    ];

    res.map_err(
        |e| {
            for suppress in suppress_errors.iter().cloned() {
                if e.kind == suppress {
                    println!("{}", e.message);
                    return e;
                }
            }

            println!("\nexiting with error: {}\nexit code:\n{}", e, e.code);
            e
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
