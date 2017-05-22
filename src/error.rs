use std::io;
use std::string;
use std::path;
use walkdir;

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

        StampNotFound {}

        HomeDirNotAccessible {}

        SourceNotProvided {}

        VarNotFound {}

        DirNotFound {}
    }
}
