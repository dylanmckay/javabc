pub use self::constant::*;
pub use self::method::Method;

pub mod constant;
pub mod method;

pub mod as_raw;
pub mod from_raw;

use Error;
use raw;

use std::fmt;

#[derive(Clone, Debug)]
pub struct ClassFile
{
    pub access_flags: raw::AccessFlags,
    pub constant_pool: ConstantPool,
    pub methods: Vec<Method>,
}

impl ClassFile
{
    pub fn as_raw(&self) -> ::raw::ClassFile {
        as_raw::as_raw(self)
    }

    pub fn from_raw(raw: ::raw::ClassFile) -> Result<Self, Error> {
        from_raw::from_raw(raw)
    }
}
