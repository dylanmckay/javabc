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

#[derive(Debug)]
pub enum Constant
{
    MethodRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    FieldRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    NameAndType {
        name_index: u16,
        descriptor_index: u16,
    },
    Class {
        name_index: u16,
    },
    Utf8 {
        text: String,
    },
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
                unimplemented!();
            },
            TAG_FLOAT => {
                unimplemented!();
            },
            TAG_LONG => {
                unimplemented!();
            },
            TAG_DOUBLE => {
                unimplemented!();
            },
            TAG_CLASS => {
                let name_index = read.read_u16::<BigEndian>()?;
                Ok(Constant::Class { name_index: name_index })
            },
            TAG_STRING => {
                unimplemented!();
            },
            TAG_FIELD_REF => {
                let class_index = read.read_u16::<BigEndian>()?;
                let name_and_type_index = read.read_u16::<BigEndian>()?;

                Ok(Constant::FieldRef { class_index: class_index,
                    name_and_type_index: name_and_type_index })
            },
            TAG_METHOD_REF => {
                let class_index = read.read_u16::<BigEndian>()?;
                let name_and_type_index = read.read_u16::<BigEndian>()?;

                Ok(Constant::MethodRef { class_index: class_index,
                    name_and_type_index: name_and_type_index })
            },
            TAG_INTERFACE_METHOD_REF => {
                unimplemented!();
            },
            TAG_NAME_AND_TYPE => {
                let name_index = read.read_u16::<BigEndian>()?;
                let descriptor_index = read.read_u16::<BigEndian>()?;

                Ok(Constant::NameAndType { name_index: name_index, descriptor_index: descriptor_index })
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

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        unimplemented!();
    }
}

