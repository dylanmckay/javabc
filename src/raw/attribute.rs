use Error;
use raw;

use std::io::prelude::*;

use byteorder::{BigEndian, ReadBytesExt};

#[derive(Debug)]
pub struct Attribute {
    pub name: raw::ConstantIndex,
    pub attribute: raw::Array<u8, u32>,
}

impl raw::Serializable for Attribute
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        let name = read.read_u16::<BigEndian>()?;
        let attribute = raw::Array::read(read)?;

        Ok(Attribute {
            name: raw::ConstantIndex(name),
            attribute: attribute,
        })
    }

    fn write(&self, _write: &mut Write) -> Result<(), Error> {
        unimplemented!();
    }
}
