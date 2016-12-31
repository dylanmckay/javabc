use {Error, ErrorKind};
use raw;

use std::io::prelude::*;

use byteorder::{BigEndian, ReadBytesExt};

pub const TAG_UTF8: u8 = 1;
pub const TAG_INTEGER: u8 = 3;
pub const TAG_FLOAT: u8 = 4;
pub const TAG_LONG: u8 = 5;
pub const TAG_DOUBLE: u8 = 6;
pub const TAG_CLASS: u8 = 7;
pub const TAG_STRING: u8 = 8;
pub const TAG_FIELD_REF: u8 = 9;
pub const TAG_METHOD_REF: u8 = 10;
pub const TAG_INTERFACE_METHOD_REF: u8 = 11;
pub const TAG_NAME_AND_TYPE: u8 = 12;
pub const TAG_METHOD_HANDLE: u8 = 15;
pub const TAG_METHOD_TYPE: u8 = 16;
pub const TAG_INVOKE_DYNAMIC: u8 = 18;

/// An index into the constant table.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ConstantIndex(pub u16);

#[derive(Debug)]
pub enum Constant
{
    MethodRef {
        class: ConstantIndex,
        name_and_type: ConstantIndex,
    },
    InterfaceMethodRef {
        class: ConstantIndex,
        name_and_type: ConstantIndex,
    },
    FieldRef {
        class: ConstantIndex,
        name_and_type: ConstantIndex,
    },
    NameAndType {
        name: ConstantIndex,
        descriptor: ConstantIndex,
    },
    Class {
        name: ConstantIndex,
    },
    Utf8 {
        text: String,
    },
    String {
        /// An index to a 'UTF-8' constant.
        index: ConstantIndex,
    },
    Integer(i32),
    Long(i64),
    Float(f32),
    Double(f64),
}

impl Constant
{
    pub fn expect_utf8(&self) -> Result<String, Error> {
        if let Constant::Utf8 { ref text } = *self {
            Ok(text.clone())
        } else {
            Err(ErrorKind::MalformedFile(format!("expected utf-8 but got {:?}", self)).into())
        }
    }
}

impl raw::Serializable for Constant
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        match read.read_u8()? {
            TAG_UTF8 => {
                let byte_count = read.read_u16::<BigEndian>()?;
                let bytes: Result<Vec<_>, _> = read.bytes().take(byte_count as _).collect();
                let s = String::from_utf8(bytes?).unwrap();

                Ok(Constant::Utf8 { text: s })
            }
            TAG_INTEGER => {
                let i = read.read_i32::<BigEndian>()?;
                Ok(Constant::Integer(i))
            },
            TAG_FLOAT => {
                let f = read.read_f32::<BigEndian>()?;
                Ok(Constant::Float(f))
            },
            TAG_LONG => {
                let i = read.read_i64::<BigEndian>()?;
                Ok(Constant::Long(i))
            },
            TAG_DOUBLE => {
                let f = read.read_f64::<BigEndian>()?;
                Ok(Constant::Double(f))
            },
            TAG_CLASS => {
                let name = ConstantIndex(read.read_u16::<BigEndian>()?);
                Ok(Constant::Class { name: name })
            },
            TAG_STRING => {
                let index = ConstantIndex(read.read_u16::<BigEndian>()?);
                Ok(Constant::String { index: index })
            },
            TAG_FIELD_REF => {
                let (class, name_and_type) = self::parse_reference(read)?;
                Ok(Constant::FieldRef { class: class,
                    name_and_type: name_and_type })
            },
            TAG_METHOD_REF => {
                let (class, name_and_type) = self::parse_reference(read)?;
                Ok(Constant::MethodRef { class: class,
                    name_and_type: name_and_type })
            },
            TAG_INTERFACE_METHOD_REF => {
                let (class, name_and_type) = self::parse_reference(read)?;
                Ok(Constant::InterfaceMethodRef { class: class,
                    name_and_type: name_and_type })
            },
            TAG_NAME_AND_TYPE => {
                let name = ConstantIndex(read.read_u16::<BigEndian>()?);
                let descriptor = ConstantIndex(read.read_u16::<BigEndian>()?);

                Ok(Constant::NameAndType { name: name, descriptor: descriptor })
            },
            TAG_METHOD_HANDLE => {
                unimplemented!();
            },
            TAG_METHOD_TYPE => {
                unimplemented!();
            },
            TAG_INVOKE_DYNAMIC => {
                unimplemented!();
            },
            i => Err(ErrorKind::MalformedFile(format!("invalid constant tag id: {}", i)).into()),
        }
    }

    fn write(&self, _write: &mut Write) -> Result<(), Error> {
        unimplemented!();
    }
}

/// Parses a reference.
/// These always have two u16s
fn parse_reference(read: &mut Read) -> Result<(ConstantIndex, ConstantIndex), Error> {
    let class_index = ConstantIndex(read.read_u16::<BigEndian>()?);
    let name_and_type_index = ConstantIndex(read.read_u16::<BigEndian>()?);
    Ok((class_index, name_and_type_index))
}
