use std::io::Error as IOError;

mod mpak;
mod file_info;

pub use mpak::Mpak;
pub use file_info::FileInfo;

#[derive(Debug)]
pub enum Error {
    IO(IOError)
}

impl From<IOError> for Error {
    fn from(error: IOError) -> Self { Error::IO(error) }
}
