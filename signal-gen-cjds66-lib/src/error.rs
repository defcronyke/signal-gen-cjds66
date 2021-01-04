/* Copyright © 2020-2021 Jeremy Carter <jeremy@jeremycarter.ca>

By using this software, you agree to the LICENSE TERMS 
outlined in the file titled LICENSE.md contained in the 
top-level directory of this project. If you don't agree
to the LICENSE TERMS, you aren't allowed to use this
software.
*/

/*! Handle errors in a common way for everything in this 
crate. This is mostly just wrapping the
[clap](https://docs.rs/clap/latest/clap) crate's 
[Error](https://docs.rs/clap/latest/clap/struct.Error.html) 
type, and adding numeric exit codes to each of their types 
of errors which are suitable for returning to the parent 
shell or execution environment, to support properly checking 
various error conditions for advanced scripting purposes 
(bash scripts, Windows batch files, or Powershell scripts, 
for example).
*/

use clap;
use clap::ErrorKind;
use clap::ErrorKind::*;
use std::fmt;

/** Get an exit code for each kind of error. This
code is suitable for passing back to the parent
shell or execution environment as an exit value,
to facilitate proper error checking when using
this library's functions with advanced scripting,
such as with a bash script, Windows batch file,
or PowerShell script.

"kind" parameter, accepts any clap::ErrorKind value, and then the function returns the associated number:
```ignore
InvalidValue => 1
UnknownArgument => 2
InvalidSubcommand => 3
UnrecognizedSubcommand => 4
EmptyValue => 5
ValueValidation => 6
TooManyValues => 7
TooFewValues => 8
WrongNumberOfValues => 9
ArgumentConflict => 10
MissingRequiredArgument => 11
MissingSubcommand => 12
MissingArgumentOrSubcommand => 13
UnexpectedMultipleUsage => 14
InvalidUtf8 => 15
HelpDisplayed => 16
VersionDisplayed => 17
ArgumentNotFound => 18
Io => 19
Format => 20
```
*/
pub fn get_code(kind: ErrorKind) -> i32 {
	match kind {
		InvalidValue => 1,
		UnknownArgument => 2,
		InvalidSubcommand => 3,
		UnrecognizedSubcommand => 4,
		EmptyValue => 5,
		ValueValidation => 6,
		TooManyValues => 7,
		TooFewValues => 8,
		WrongNumberOfValues => 9,
		ArgumentConflict => 10,
		MissingRequiredArgument => 11,
		MissingSubcommand => 12,
		MissingArgumentOrSubcommand => 13,
		UnexpectedMultipleUsage => 14,
		InvalidUtf8 => 15,
		HelpDisplayed => 16,
		VersionDisplayed => 17,
		ArgumentNotFound => 18,
		Io => 19,
		Format => 20,
	}
}

/** Handle the return value of a function from the `command` module,
suppressing any non-important types of errors.
*/
pub fn handle_exit(res: Result<i32, Error>) -> Result<i32, Error> {
	let suppress_errors = [HelpDisplayed, VersionDisplayed, MissingArgumentOrSubcommand];

	res.map_err(|e| {
		for suppress in suppress_errors.iter().cloned() {
			if e.kind == suppress {
				println!("{}", e.message);
				return e;
			}
		}

		println!("\nexiting with error: {}\nexit code:\n{}", e, e.code);
		e
	})
}

/** A data type which wraps the
[clap](https://docs.rs/clap/latest/clap) crate's 
[Error](https://docs.rs/clap/latest/clap/struct.Error.html) 
type, and adds numeric exit codes to each of their types 
of errors, as well as a few extra helper functions.
*/
#[derive(Debug)]
pub struct Error {
	/// An error message.
	pub message: String,

	/// An optional set of info about the error.
	pub info: Option<Vec<String>>,

	/// The kind of error that it is.
	pub kind: ErrorKind,

	/// The numeric error code, suitable for returning to the parent shell on exit.
	pub code: i32,

	/// The printable name of the kind of error that it is.
	pub name: String,
}

impl Error {
	/// Construct a new Error value.
	pub fn new<'a>(message: &'a str, info: Option<Vec<String>>, kind: ErrorKind) -> Self {
		let message = String::from(message);
		let code = get_code(kind);
		let name = format!("{:?}", kind);

		Self {
			message,
			info,
			kind,
			code,
			name,
		}
	}
}

/// Easily construct a new Error value using one of these convenience functions.
pub trait From {
	/** Construct a new Error value from an existing
	[clap](https://docs.rs/clap/latest/clap) 
	[Error](https://docs.rs/clap/latest/clap/struct.Error.html)
	*/
	fn from_clap_error(val: clap::Error) -> Self;

	/** Construct a new Error value with a given error message, using a certain
	[clap](https://docs.rs/clap/latest/clap)::[ErrorKind](https://docs.rs/clap/latest/clap/enum.ErrorKind.html)
	as a base.
	*/
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
			write!(
				f,
				"\nerror ({} {}): {}: {:?}\n",
				self.name,
				self.code,
				self.message,
				self.info.clone().unwrap_or_default()
			)
		} else {
			write!(
				f,
				"\nerror ({} {}): {}\n",
				self.name, self.code, self.message
			)
		}
	}
}
