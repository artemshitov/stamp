use nom;
use walkdir;

use std::io;
use std::string;
use std::path;
use std::result;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        IOError(err: io::Error) {
            from()
        }

        WalkDirError(err: walkdir::Error) {
            from()
        }

        StripPrefixError(err: path::StripPrefixError) {
            from()
        }

        FromUtf8Error(err: string::FromUtf8Error) {
            from()
        }

        ParsingError(err: nom::IError) {
            from()
        }

        StampNotFound {}

        HomeDirNotAccessible {}

        SourceNotProvided {}

        VarNotFound {}

        DirNotFound {}
    }
}

pub type Result<T> = result::Result<T, Error>;
