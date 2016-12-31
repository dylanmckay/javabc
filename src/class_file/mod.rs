pub use self::ty::{Type, PrimitiveType};
pub use self::method::{Method, MethodSignature};

pub mod ty;
pub mod method;

pub mod from_raw;

use Error;
use raw;

#[derive(Clone, Debug)]
pub struct ClassFile
{
    pub access_flags: raw::AccessFlags,
    pub methods: Vec<Method>,
}

impl ClassFile
{
    pub fn from_raw(raw: ::raw::ClassFile) -> Result<Self, Error> {
        from_raw::from_raw(raw)
    }
}
