use Error;
use raw;

use std::marker;
use std::io::prelude::*;

use byteorder::{BigEndian, ReadBytesExt};

#[derive(Debug)]
#[repr(C)]
pub struct Array<T, I>
{
    pub items: Vec<T>,
    phantom: marker::PhantomData<I>,
}

impl<T> raw::Serializable for Array<T,u16>
    where T: raw::Serializable
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        let count = read.read_u16::<BigEndian>()?;
        let mut items = Vec::new();

        for _ in 1..count {
            items.push(T::read(read)?);
        }

        Ok(Array { items: items, phantom: marker::PhantomData })
    }

    fn write(&self, _write: &mut Write) -> Result<(), Error> {
        unimplemented!();
    }
}
