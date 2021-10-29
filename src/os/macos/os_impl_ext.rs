use crate::macos::MacOS;
use crate::Result;
pub trait OsImplExt {
    fn model(&self) -> Result<String>;
}

impl OsImplExt for MacOS {
    fn model(&self) -> Result<String> {
        crate::macos::model()
    }
}
