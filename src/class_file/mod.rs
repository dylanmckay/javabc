pub use self::constant::*;

pub mod constant;
pub mod dump;

pub mod as_raw;
pub mod from_raw;

use raw;

use std::fmt;

#[derive(Clone)]
pub struct ClassFile
{
    pub access_flags: raw::AccessFlags,
    pub constant_pool: ConstantPool,
}

impl ClassFile
{
    pub fn as_raw(&self) -> ::raw::ClassFile {
        as_raw::as_raw(self)
    }

    pub fn from_raw(raw: ::raw::ClassFile) -> Self {
        from_raw::from_raw(raw)
    }
}

impl fmt::Debug for ClassFile
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        dump::class(self, fmt)
    }
}
