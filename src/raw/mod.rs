pub use self::class_file::*;

pub mod class_file;

use std::io;

pub trait Serializable : Sized
{
    fn read(read: &mut io::Read) -> Result<Self, ::Error>;
    fn write(&self, write: &mut io::Write) -> Result<(), ::Error>;
}
