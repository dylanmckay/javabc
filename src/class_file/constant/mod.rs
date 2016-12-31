pub use self::pool::ConstantPool;

pub mod pool;

use {Error, ErrorKind};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ConstantIndex(pub usize);

#[derive(Clone, Debug, PartialEq)]
pub enum Constant
{
    Utf8(String),
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
    pub fn expect_utf8(&self) -> Result<&str, Error> {
        if let Constant::Utf8(ref s) = *self {
            Ok(s)
        } else {
            Err(ErrorKind::MalformedFile(format!("expected utf-8 constant byt got {:?}", self)).into())
        }
    }
}
