pub use self::class_file::ClassFile;
pub use self::constant::{Constant, ConstantIndex};
pub use self::method::Method;
pub use self::field::Field;
pub use self::interface::Interface;
pub use self::attribute::Attribute;
pub use self::flags::AccessFlags;
pub use self::array::Array;

pub mod class_file;
pub mod constant;
pub mod method;
pub mod field;
pub mod interface;
pub mod attribute;
pub mod flags;
pub mod array;

use std::io;

pub trait Serializable : Sized
{
    fn read(read: &mut io::Read) -> Result<Self, ::Error>;
    fn write(&self, write: &mut io::Write) -> Result<(), ::Error>;
}
