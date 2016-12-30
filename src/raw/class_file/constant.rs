use {Error, ErrorKind};
use raw;

use std::io::prelude::*;

use byteorder::{BigEndian, ReadBytesExt};

#[derive(Debug)]
#[repr(u8)]
pub enum ConstantTag
{
    Class = 7,
    FieldRef = 9,
    MethodRef = 10,
    InterfaceMethodRef = 11,
    String = 8,
    Integer = 3,
    Float = 4,
    Long = 5,
    Double = 6,
    NameAndType = 12,
    Utf8 = 1,
    MethodHandle = 15,
    MethodType = 16,
    InvokeDynamic = 18,
}

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
            1 => { // UTF-8
                let byte_count = read.read_u16::<BigEndian>()?;
                let bytes: Result<Vec<_>, _> = read.bytes().take(byte_count as _).collect();
                let s = String::from_utf8(bytes?).unwrap();

                Ok(Constant::Utf8 { text: s })
            }
            3 => { // Integer
                unimplemented!();
            },
            4 => { // Float
                unimplemented!();
            },
            5 => { // Long
                unimplemented!();
            },
            6 => { // Double
                unimplemented!();
            },
            7 => { // Class
                let name_index = read.read_u16::<BigEndian>()?;
                Ok(Constant::Class { name_index: name_index })
            },
            8 => { // String
                unimplemented!();
            },
            9 => { // Field reference
                let class_index = read.read_u16::<BigEndian>()?;
                let name_and_type_index = read.read_u16::<BigEndian>()?;

                Ok(Constant::FieldRef { class_index: class_index,
                    name_and_type_index: name_and_type_index })
            },
            10 => { // Method reference
                let class_index = read.read_u16::<BigEndian>()?;
                let name_and_type_index = read.read_u16::<BigEndian>()?;

                Ok(Constant::MethodRef { class_index: class_index,
                    name_and_type_index: name_and_type_index })
            },
            11 => { // Interface method reference
                unimplemented!();
            },
            12 => { // Name and type
                let name_index = read.read_u16::<BigEndian>()?;
                let descriptor_index = read.read_u16::<BigEndian>()?;

                Ok(Constant::NameAndType { name_index: name_index, descriptor_index: descriptor_index })
            },
            15 => { // Method handle
                unimplemented!();
            },
            16 => { // Method type
                unimplemented!();
            },
            18 => { // Invoke dynamic
                unimplemented!();
            },
            i => Err(ErrorKind::MalformedFile(format!("invalid constant tag id: {}", i)).into()),
        }
    }

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        unimplemented!();
    }
}

