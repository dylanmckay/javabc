use Error;
use raw;

use std::io::prelude::*;

#[derive(Debug)]
pub struct Field;

impl raw::Serializable for Field
{
    fn read(_read: &mut Read) -> Result<Self, Error> {
        unimplemented!();
    }

    fn write(&self, _write: &mut Write) -> Result<(), Error> {
        unimplemented!();
    }
}
