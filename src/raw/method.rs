use Error;
use raw;

use std::io::prelude::*;

use byteorder::{BigEndian, ReadBytesExt};

#[derive(Debug)]
pub struct Method {
    pub access_flags: raw::AccessFlags,
    pub name: raw::ConstantIndex,
    pub descriptor: raw::ConstantIndex,
    pub attributes: raw::Array<raw::Attribute, u16>,
}

impl raw::Serializable for Method
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        let access_flags = read.read_u16::<BigEndian>()?;
        let name = read.read_u16::<BigEndian>()?;
        let descriptor = read.read_u16::<BigEndian>()?;
        let attributes = raw::Array::read(read)?;

        Ok(Method {
            access_flags: raw::AccessFlags::from_bits(access_flags).unwrap(),
            name: raw::ConstantIndex(name),
            descriptor: raw::ConstantIndex(descriptor),
            attributes: attributes,
        })
    }

    fn write(&self, _write: &mut Write) -> Result<(), Error> {
        unimplemented!();
    }
}
