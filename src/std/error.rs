use alloc::boxed::Box;
use collections::string::String;
use core::fmt::{self, Debug, Display};
use core::marker::{Send, Sync, Reflect};

/// Base functionality for all errors in Rust.
//#[stable(feature = "rust1", since = "1.0.0")]
pub trait Error: Debug + Display + Reflect {
    /// A short description of the error.
    ///
    /// The description should not contain newlines or sentence-ending
    /// punctuation, to facilitate embedding in larger user-facing
    /// strings.
    //#[stable(feature = "rust1", since = "1.0.0")]
    fn description(&self) -> &str;

    /// The lower-level cause of this error, if any.
    //#[stable(feature = "rust1", since = "1.0.0")]
    fn cause(&self) -> Option<&Error> { None }
}

//#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, E: Error + 'a> From<E> for Box<Error + 'a> {
    fn from(err: E) -> Box<Error + 'a> {
        Box::new(err)
    }
}

//#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, E: Error + Send + Sync + 'a> From<E> for Box<Error + Send + Sync + 'a> {
    fn from(err: E) -> Box<Error + Send + Sync + 'a> {
        Box::new(err)
    }
}

//#[stable(feature = "rust1", since = "1.0.0")]
impl From<String> for Box<Error + Send + Sync> {
    fn from(err: String) -> Box<Error + Send + Sync> {
        #[derive(Debug)]
        struct StringError(String);

        impl Error for StringError {
            fn description(&self) -> &str { &self.0 }
        }

        impl Display for StringError {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                Display::fmt(&self.0, f)
            }
        }

        Box::new(StringError(err))
    }
}

//#[stable(feature = "string_box_error", since = "1.7.0")]
impl From<String> for Box<Error> {
    fn from(str_err: String) -> Box<Error> {
        let err1: Box<Error + Send + Sync> = From::from(str_err);
        let err2: Box<Error> = err1;
        err2
    }
}

//#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, 'b> From<&'b str> for Box<Error + Send + Sync + 'a> {
    fn from(err: &'b str) -> Box<Error + Send + Sync + 'a> {
        From::from(String::from(err))
    }
}

//#[stable(feature = "string_box_error", since = "1.7.0")]
impl<'a> From<&'a str> for Box<Error> {
    fn from(err: &'a str) -> Box<Error> {
        From::from(String::from(err))
    }
}
