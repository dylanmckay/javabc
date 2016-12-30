use Error;
use raw;

use std::io::prelude::*;

#[derive(Debug)]
pub struct Method {
    pub access_flags: raw::AccessFlags,
    pub name: raw::ConstantIndex,
    pub descriptor: raw::ConstantIndex,
    pub attributes: raw::Array<raw::Attribute, u16>,
}

impl raw::Serializable for Method
{
    fn read(_read: &mut Read) -> Result<Self, Error> {
        unimplemented!();
    }

    fn write(&self, _write: &mut Write) -> Result<(), Error> {
        unimplemented!();
    }
}

