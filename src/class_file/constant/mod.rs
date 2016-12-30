pub use self::pool::ConstantPool;

pub mod pool;

use {Error, ErrorKind};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ConstantIndex(pub usize);

#[derive(Clone, Debug, PartialEq)]
pub enum Constant
{
    Utf8(String),
    Integer(i32),
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
