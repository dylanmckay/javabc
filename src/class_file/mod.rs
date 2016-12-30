pub use self::constant::*;

pub mod constant;
pub mod as_raw;

use raw;

#[derive(Clone, Debug)]
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
}
