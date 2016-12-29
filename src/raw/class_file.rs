use raw;

use std::io::prelude::*;
use std::marker;
use std::io;

use byteorder::{BigEndian, ReadBytesExt};

#[derive(Debug)]
pub struct ClassFile
{
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pools: Array<Constant, u16>,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces: Array<Interface, u16>,
    pub fields: Array<Field, u16>,
    pub methods: Array<Method, u16>,
    pub attributes: Array<Attribute, u16>,
}

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

#[derive(Debug)]
pub struct Interface;
#[derive(Debug)]
pub struct Field;
#[derive(Debug)]
pub struct Method;
#[derive(Debug)]
pub struct Attribute;

impl raw::Serializable for Constant
{
    fn read(read: &mut Read) -> Result<Self, io::Error> {
        match read.read_u8()? {
            1 => {
                let byte_count = read.read_u16::<BigEndian>()?;
                let bytes: Result<Vec<_>, _> = read.bytes().take(byte_count as _).collect();
                let s = String::from_utf8(bytes?).unwrap();

                Ok(Constant::Utf8 { text: s })
            }
            7 => {
                let name_index = read.read_u16::<BigEndian>()?;
                Ok(Constant::Class { name_index: name_index })
            },
            10 => {
                let class_index = read.read_u16::<BigEndian>()?;
                let name_and_type_index = read.read_u16::<BigEndian>()?;

                Ok(Constant::MethodRef { class_index: class_index,
                    name_and_type_index: name_and_type_index })
            },
            12 => {
                let name_index = read.read_u16::<BigEndian>()?;
                let descriptor_index = read.read_u16::<BigEndian>()?;

                Ok(Constant::NameAndType { name_index: name_index, descriptor_index: descriptor_index })
            },
            i => panic!("don't know how to handle: {}", i),
        }
    }

    fn write(&self, write: &mut Write) -> Result<(), io::Error> {
        unimplemented!();
    }
}

impl raw::Serializable for Interface
{
    fn read(read: &mut Read) -> Result<Self, io::Error> {
        unimplemented!();
    }

    fn write(&self, write: &mut Write) -> Result<(), io::Error> {
        unimplemented!();
    }
}

impl raw::Serializable for Field
{
    fn read(read: &mut Read) -> Result<Self, io::Error> {
        unimplemented!();
    }

    fn write(&self, write: &mut Write) -> Result<(), io::Error> {
        unimplemented!();
    }
}

impl raw::Serializable for Method
{
    fn read(read: &mut Read) -> Result<Self, io::Error> {
        unimplemented!();
    }

    fn write(&self, write: &mut Write) -> Result<(), io::Error> {
        unimplemented!();
    }
}

impl raw::Serializable for Attribute
{
    fn read(read: &mut Read) -> Result<Self, io::Error> {
        unimplemented!();
    }

    fn write(&self, write: &mut Write) -> Result<(), io::Error> {
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
    fn read(read: &mut Read) -> Result<Self, io::Error> {
        let count = read.read_u16::<BigEndian>()?;
        let mut items = Vec::new();

        for i in 1..count {
            items.push(T::read(read)?);
        }

        Ok(Array { items: items, phantom: marker::PhantomData })
    }

    fn write(&self, write: &mut Write) -> Result<(), io::Error> {
        unimplemented!();
    }
}

impl raw::Serializable for ClassFile
{
    fn read(read: &mut Read) -> Result<Self, io::Error> {
        let magic = read.read_u32::<BigEndian>()?;
        let minor = read.read_u16::<BigEndian>()?;
        let major = read.read_u16::<BigEndian>()?;
        let constant_pools = Array::read(read)?;
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
            constant_pools: constant_pools,
            access_flags: access_flags,
            this_class: this_class,
            super_class: super_class,
            interfaces: interfaces,
            fields: fields,
            methods: methods,
            attributes: attributes,
        })
    }

    fn write(&self, write: &mut Write) -> Result<(), io::Error> {
        unimplemented!();
    }
}
