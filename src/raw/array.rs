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

#[derive(Debug)]
#[repr(C)]
pub struct OneBasedArray<T, I>
{
    pub items: Vec<T>,
    phantom: marker::PhantomData<I>,
}

macro_rules! impl_for_index_type {
    ($ty:ty, $read_method:ident) => {
        impl<T> raw::Serializable for OneBasedArray<T,$ty>
            where T: raw::Serializable
        {
            fn read(read: &mut Read) -> Result<Self, Error> {
                let count = read.$read_method::<BigEndian>()?;
                let mut items = Vec::new();

                for _ in 1..count {
                    items.push(T::read(read)?);
                }

                Ok(OneBasedArray { items: items, phantom: marker::PhantomData })
            }

            fn write(&self, _write: &mut Write) -> Result<(), Error> {
                unimplemented!();
            }
        }

        impl<T> raw::Serializable for Array<T,$ty>
            where T: raw::Serializable
        {
            fn read(read: &mut Read) -> Result<Self, Error> {
                let count = read.$read_method::<BigEndian>()?;
                let mut items = Vec::new();

                for _ in 0..count {
                    items.push(T::read(read)?);
                }

                Ok(Array { items: items, phantom: marker::PhantomData })
            }

            fn write(&self, _write: &mut Write) -> Result<(), Error> {
                unimplemented!();
            }
        }
    }
}

impl_for_index_type!(u16, read_u16);
impl_for_index_type!(u32, read_u32);
