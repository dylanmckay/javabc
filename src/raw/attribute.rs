use Error;
use raw;

use std::io::prelude::*;

#[derive(Debug)]
pub struct Attribute;

impl raw::Serializable for Attribute
{
    fn read(_read: &mut Read) -> Result<Self, Error> {
        unimplemented!();
    }

    fn write(&self, _write: &mut Write) -> Result<(), Error> {
        unimplemented!();
    }
}
