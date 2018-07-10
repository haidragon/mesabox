use clap;
use failure::Compat;
use nix;

use std::io;
use std::os::unix::io::RawFd;
use std::result::Result as StdResult;

use ::MesaError;
use ::error::LockError;

pub type Result<T> = StdResult<T, ShellError>;
pub type CmdResult<T> = StdResult<T, CommandError>;

#[derive(Fail, Debug)]
pub enum ShellError {
    /// Indicate that a command failed to start
    #[fail(display = "{}: {}", cmdname, err)]
    Command {
        #[cause] err: CommandError,
        cmdname: String,
    },
}

#[derive(Fail, Debug)]
pub enum CommandError {
    #[fail(display = "{}", _0)]
    StartRealCommand(#[cause] io::Error),

    #[fail(display = "could not get exit status: {}", _0)]
    RealCommandStatus(#[cause] io::Error),

    #[fail(display = "could not duplicate fd {}: {}", fd, err)]
    DupFd {
        #[cause] err: nix::Error,
        fd: RawFd,
    },

    #[fail(display = "bad fd number ({})", _0)]
    InvalidFd(u8),

    #[fail(display = "{}", _0)]
    Pipe(#[cause] nix::Error),

    #[fail(display = "{}", _0)]
    PipeIo(#[cause] io::Error),

    #[fail(display = "could not set up fd {} as file {}: {}", fd, filename, err)]
    FdAsFile {
        #[cause] err: io::Error,
        fd: RawFd,
        filename: String,
    },

    #[fail(display = "{}", _0)]
    Builtin(#[cause] BuiltinError),

    // XXX: depending on any features we decide to add, this may expand to include functions
}

#[derive(Fail, Debug)]
pub enum BuiltinError {
    #[fail(display = "{}", _0)]
    Clap(#[cause] clap::Error),

    /// Wrapper for a generic I/O error
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),

    /// Wrapper for a generic nix error (most likely I/O related)
    #[fail(display = "{}", _0)]
    Nix(#[cause] nix::Error),

    /// Wrapper for a LockError (note that this may be removed if UtilRead/UtilWrite are never
    /// implemented for something that can fail to lock)
    #[fail(display = "{}", _0)]
    Lock(#[cause] LockError),

    #[fail(display = "{}", _0)]
    Other(#[cause] Compat<MesaError>),
}

impl From<clap::Error> for BuiltinError {
    fn from(err: clap::Error) -> Self {
        BuiltinError::Clap(err)
    }
}

impl From<io::Error> for BuiltinError {
    fn from(err: io::Error) -> Self {
        BuiltinError::Io(err)
    }
}

impl From<nix::Error> for BuiltinError {
    fn from(err: nix::Error) -> Self {
        BuiltinError::Nix(err)
    }
}

impl From<LockError> for BuiltinError {
    fn from(err: LockError) -> Self {
        BuiltinError::Lock(err)
    }
}