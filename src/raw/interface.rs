use Error;
use raw;

use std::io::prelude::*;
use byteorder::{BigEndian, ReadBytesExt};

#[derive(Debug)]
pub struct Interface {
    pub index: raw::ConstantIndex,
}

impl raw::Serializable for Interface
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        let index = read.read_u16::<BigEndian>()?;
        Ok(Interface { index: raw::ConstantIndex(index) })
    }

    fn write(&self, _write: &mut Write) -> Result<(), Error> {
        unimplemented!();
    }
}
