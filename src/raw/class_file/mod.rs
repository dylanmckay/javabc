pub use self::constant::{Constant, ConstantIndex};

pub mod constant;

use {Error, ErrorKind};
use raw;

use std::io::prelude::*;
use std::marker;
use std::io;

use byteorder::{BigEndian, ReadBytesExt};

/// The class file magic number.
pub const MAGIC: u32 = 0xcafebabe;

bitflags! {
    pub flags AccessFlags: u16 {
        /// Declared public; may be accessed from outside its package.
        const ACC_PUBLIC = 0x0001,
        /// Declared final; no subclasses allowed.
        const ACC_FINAL = 0x0010,
        /// Treat superclass methods specially when invoked by the invokespecial instruction.
        const ACC_SUPER = 0x0020,
        /// Is an interface, not a class.
        const ACC_INTERFACE = 0x0200,
        /// Declared abstract; must not be instantiated.
        const ACC_ABSTRACT = 0x0400,
        /// Declared synthetic; not present in the source code.
        const ACC_SYNTHETIC = 0x1000,
        /// Declared as an annotation type.
        const ACC_ANNOTATION = 0x2000,
        /// Declared as an enum type.
        const ACC_ENUM = 0x4000
    }
}

#[derive(Debug)]
pub struct ClassFile
{
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: Array<Constant, u16>,
    pub access_flags: AccessFlags,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces: Array<Interface, u16>,
    pub fields: Array<Field, u16>,
    pub methods: Array<Method, u16>,
    pub attributes: Array<Attribute, u16>,
}

#[derive(Debug)]
pub struct Interface;
#[derive(Debug)]
pub struct Field;
#[derive(Debug)]
pub struct Method;
#[derive(Debug)]
pub struct Attribute;

impl raw::Serializable for Interface
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        unimplemented!();
    }

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        unimplemented!();
    }
}

impl raw::Serializable for Field
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        unimplemented!();
    }

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        unimplemented!();
    }
}

impl raw::Serializable for Method
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        unimplemented!();
    }

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        unimplemented!();
    }
}

impl raw::Serializable for Attribute
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        unimplemented!();
    }

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct Array<T, I>
{
    pub items: Vec<T>,
    phantom: marker::PhantomData<I>,
}

impl<T> raw::Serializable for Array<T,u16>
    where T: raw::Serializable
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        let count = read.read_u16::<BigEndian>()?;
        let mut items = Vec::new();

        for _ in 1..count {
            items.push(T::read(read)?);
        }

        Ok(Array { items: items, phantom: marker::PhantomData })
    }

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        unimplemented!();
    }
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
        let constant_pool = Array::read(read)?;
        let access_flags = read.read_u16::<BigEndian>()?;
        let this_class = read.read_u16::<BigEndian>()?;
        let super_class = read.read_u16::<BigEndian>()?;
        let interfaces = Array::read(read)?;
        let fields = Array::read(read)?;
        let methods = Array::read(read)?;
        let attributes = Array::read(read)?;

        Ok(ClassFile {
            magic: magic,
            minor_version: minor,
            major_version: major,
            constant_pool: constant_pool,
            access_flags: AccessFlags::from_bits(access_flags).unwrap(),
            this_class: this_class,
            super_class: super_class,
            interfaces: interfaces,
            fields: fields,
            methods: methods,
            attributes: attributes,
        })
    }

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        unimplemented!();
    }
}
