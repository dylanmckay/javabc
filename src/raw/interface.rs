use Error;
use raw;

use std::io::prelude::*;

#[derive(Debug)]
pub struct Interface;

impl raw::Serializable for Interface
{
    fn read(_read: &mut Read) -> Result<Self, Error> {
        unimplemented!();
    }

    fn write(&self, _write: &mut Write) -> Result<(), Error> {
        unimplemented!();
    }
}
