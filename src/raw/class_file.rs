use {Error, ErrorKind};
use raw;

use std::io::prelude::*;

use byteorder::{BigEndian, ReadBytesExt};

/// The class file magic number.
pub const MAGIC: u32 = 0xcafebabe;

#[derive(Debug)]
pub struct ClassFile
{
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: raw::OneBasedArray<raw::Constant, u16>,
    pub access_flags: raw::AccessFlags,
    pub this_class: raw::ConstantIndex,
    pub super_class: raw::ConstantIndex,
    pub interfaces: raw::Array<raw::Interface, u16>,
    pub fields: raw::Array<raw::Field, u16>,
    pub methods: raw::Array<raw::Method, u16>,
    pub attributes: raw::Array<raw::Attribute, u16>,
}

impl raw::Serializable for ClassFile
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        let magic = read.read_u32::<BigEndian>()?;

        if magic != MAGIC {
            return Err(ErrorKind::MalformedFile(
                format!("magic number was 0x{:x} but should be 0x{:x}",
                        magic, MAGIC)).into());
        }

        let minor = read.read_u16::<BigEndian>()?;
        let major = read.read_u16::<BigEndian>()?;
        let constant_pool = raw::OneBasedArray::read(read)?;
        let access_flags = read.read_u16::<BigEndian>()?;
        let this_class = read.read_u16::<BigEndian>()?;
        let super_class = read.read_u16::<BigEndian>()?;
        let interfaces = raw::Array::read(read)?;
        let fields = raw::Array::read(read)?;
        let methods = raw::Array::read(read)?;
        let attributes = raw::Array::read(read)?;

        Ok(ClassFile {
            magic: magic,
            minor_version: minor,
            major_version: major,
            constant_pool: constant_pool,
            access_flags: raw::AccessFlags::from_bits(access_flags).unwrap(),
            this_class: raw::ConstantIndex(this_class),
            super_class: raw::ConstantIndex(super_class),
            interfaces: interfaces,
            fields: fields,
            methods: methods,
            attributes: attributes,
        })
    }

    fn write(&self, _write: &mut Write) -> Result<(), Error> {
        unimplemented!();
    }
}
