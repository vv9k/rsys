use crate::macos::Macos;
use crate::Result;
pub trait OsImplExt {
    fn model(&self) -> Result<String>;
}

impl OsImplExt for Macos {
    fn model(&self) -> Result<String> {
        crate::macos::model()
    }
}
