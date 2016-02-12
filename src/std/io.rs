use alloc::boxed::Box;
use core::result;
use std::error;

/// A specialized [`Result`](../result/enum.Result.html) type for I/O
/// operations.
///
/// This type is broadly used across `std::io` for any operation which may
/// produce an error.
///
/// This typedef is generally used to avoid writing out `io::Error` directly and
/// is otherwise a direct mapping to `Result`.
///
/// While usual Rust style is to import types directly, aliases of `Result`
/// often are not, to make it easier to distinguish between them. `Result` is
/// generally assumed to be `std::result::Result`, and so users of this alias
/// will generally use `io::Result` instead of shadowing the prelude's import
/// of `std::result::Result`.
///
/// # Examples
///
/// A convenience function that bubbles an `io::Result` to its caller:
///
/// ```
/// use std::io;
///
/// fn get_string() -> io::Result<String> {
///     let mut buffer = String::new();
///
///     try!(io::stdin().read_line(&mut buffer));
///
///     Ok(buffer)
/// }
/// ```
//#[stable(feature = "rust1", since = "1.0.0")]
pub type Result<T> = result::Result<T, Error>;

/// The error type for I/O operations of the `Read`, `Write`, `Seek`, and
/// associated traits.
///
/// Errors mostly originate from the underlying OS, but custom instances of
/// `Error` can be created with crafted error messages and a particular value of
/// `ErrorKind`.
#[derive(Debug)]
//#[stable(feature = "rust1", since = "1.0.0")]
pub struct Error {
    repr: Repr,
}

// XXX manual addition
#[derive(Debug)]
enum Repr {
    Os(i32),
    Custom(Box<Custom>),
}

#[derive(Debug)]
struct Custom {
    kind: ErrorKind,
    error: Box<error::Error+Send+Sync>,
}

/// A list specifying general categories of I/O error.
///
/// This list is intended to grow over time and it is not recommended to
/// exhaustively match against it.
#[derive(Copy, PartialEq, Eq, Clone, Debug)]
//#[stable(feature = "rust1", since = "1.0.0")]
#[allow(deprecated)]
pub enum ErrorKind {
    /// An entity was not found, often a file.
    //#[stable(feature = "rust1", since = "1.0.0")]
    NotFound,
    /// The operation lacked the necessary privileges to complete.
    //#[stable(feature = "rust1", since = "1.0.0")]
    PermissionDenied,
    /// The connection was refused by the remote server.
    //#[stable(feature = "rust1", since = "1.0.0")]
    ConnectionRefused,
    /// The connection was reset by the remote server.
    //#[stable(feature = "rust1", since = "1.0.0")]
    ConnectionReset,
    /// The connection was aborted (terminated) by the remote server.
    //#[stable(feature = "rust1", since = "1.0.0")]
    ConnectionAborted,
    /// The network operation failed because it was not connected yet.
    //#[stable(feature = "rust1", since = "1.0.0")]
    NotConnected,
    /// A socket address could not be bound because the address is already in
    /// use elsewhere.
    //#[stable(feature = "rust1", since = "1.0.0")]
    AddrInUse,
    /// A nonexistent interface was requested or the requested address was not
    /// local.
    //#[stable(feature = "rust1", since = "1.0.0")]
    AddrNotAvailable,
    /// The operation failed because a pipe was closed.
    //#[stable(feature = "rust1", since = "1.0.0")]
    BrokenPipe,
    /// An entity already exists, often a file.
    //#[stable(feature = "rust1", since = "1.0.0")]
    AlreadyExists,
    /// The operation needs to block to complete, but the blocking operation was
    /// requested to not occur.
    //#[stable(feature = "rust1", since = "1.0.0")]
    WouldBlock,
    /// A parameter was incorrect.
    //#[stable(feature = "rust1", since = "1.0.0")]
    InvalidInput,
    /// Data not valid for the operation were encountered.
    ///
    /// Unlike `InvalidInput`, this typically means that the operation
    /// parameters were valid, however the error was caused by malformed
    /// input data.
    ///
    /// For example, a function that reads a file into a string will error with
    /// `InvalidData` if the file's contents are not valid UTF-8.
    //#[stable(feature = "io_invalid_data", since = "1.2.0")]
    InvalidData,
    /// The I/O operation's timeout expired, causing it to be canceled.
    //#[stable(feature = "rust1", since = "1.0.0")]
    TimedOut,
    /// An error returned when an operation could not be completed because a
    /// call to `write` returned `Ok(0)`.
    ///
    /// This typically means that an operation could only succeed if it wrote a
    /// particular number of bytes but only a smaller number of bytes could be
    /// written.
    //#[stable(feature = "rust1", since = "1.0.0")]
    WriteZero,
    /// This operation was interrupted.
    ///
    /// Interrupted operations can typically be retried.
    //#[stable(feature = "rust1", since = "1.0.0")]
    Interrupted,
    /// Any I/O error not part of this list.
    //#[stable(feature = "rust1", since = "1.0.0")]
    Other,

    #[allow(missing_docs)]
    /*
    #[unstable(feature = "read_exact_old", reason = "recently added",
               issue = "0")]
    */
    //#[rustc_deprecated(since = "1.6.0", reason = "renamed to UnexpectedEof")]
    UnexpectedEOF,

    /// An error returned when an operation could not be completed because an
    /// "end of file" was reached prematurely.
    ///
    /// This typically means that an operation could only succeed if it read a
    /// particular number of bytes but only a smaller number of bytes could be
    /// read.
    //#[stable(feature = "read_exact", since = "1.6.0")]
    UnexpectedEof,

    /// Any I/O error not part of this list.
    /*
    #[unstable(feature = "io_error_internals",
               reason = "better expressed through extensible enums that this \
                         enum cannot be exhaustively matched against",
               issue = "0")]
    */
    #[doc(hidden)]
    __Nonexhaustive,
}

impl Error {
    /// Creates a new I/O error from a known kind of error as well as an
    /// arbitrary error payload.
    ///
    /// This function is used to generically create I/O errors which do not
    /// originate from the OS itself. The `error` argument is an arbitrary
    /// payload which will be contained in this `Error`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::{Error, ErrorKind};
    ///
    /// // errors can be created from strings
    /// let custom_error = Error::new(ErrorKind::Other, "oh no!");
    ///
    /// // errors can also be created from other errors
    /// let custom_error2 = Error::new(ErrorKind::Interrupted, custom_error);
    /// ```
    //#[stable(feature = "rust1", since = "1.0.0")]
    pub fn new<E>(kind: ErrorKind, error: E) -> Error
        where E: Into<Box<error::Error+Send+Sync>>
    {
        Self::_new(kind, error.into())
    }

    fn _new(kind: ErrorKind, error: Box<error::Error+Send+Sync>) -> Error {
        Error {
            repr: Repr::Custom(Box::new(Custom {
                kind: kind,
                error: error,
            }))
        }
    }
}

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
}
