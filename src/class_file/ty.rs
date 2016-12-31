/// A type.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type
{
    /// A primitive type.
    Primitive(PrimitiveType),
    /// A class
    Class {
        /// The name of the class.
        name: String,
    },
    /// An array.
    Array {
        /// The underlying type.
        inner: Box<Type>,
    },
}

/// A primitive type.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PrimitiveType
{
    Void,
    Byte,
    Short,
    Integer,
    Long,
    Float,
    Double,
    Boolean,
    Char,
}

impl Type
{
    /// Parses a textual descriptor.
    pub fn parse_descriptor(d: &str) -> Option<Self> {
        let c = if let Some(c) = d.chars().next() {
            c
        } else {
            return None;
        };

        // Check if we are parsing an array.
        if c == '[' {
            let inner_descriptor = &d[1..];

            match Type::parse_descriptor(inner_descriptor) {
                Some(inner) => Some(Type::Array { inner: Box::new(inner) }),
                None => None,
            }
        } else if c == 'L' {
            let class_name = &d[1..];
            Some(Type::Class { name: class_name.to_owned() })
        } else {
            Type::parse_primitive_descriptor(d.chars().next().unwrap())
        }
    }

    fn parse_primitive_descriptor(d: char) -> Option<Self> {
        match d {
            'V' => Some(Type::Primitive(PrimitiveType::Void)),
            'B' => Some(Type::Primitive(PrimitiveType::Byte)),
            'S' => Some(Type::Primitive(PrimitiveType::Short)),
            'I' => Some(Type::Primitive(PrimitiveType::Integer)),
            'J' => Some(Type::Primitive(PrimitiveType::Long)),
            'F' => Some(Type::Primitive(PrimitiveType::Float)),
            'D' => Some(Type::Primitive(PrimitiveType::Double)),
            'Z' => Some(Type::Primitive(PrimitiveType::Boolean)),
            'C' => Some(Type::Primitive(PrimitiveType::Char)),
            _ => None,
        }
    }

    /// Gets the textual type descriptor.
    pub fn descriptor(&self) -> String {
        match *self {
            Type::Primitive(pt) => match pt {
                PrimitiveType::Void => "V".to_owned(),
                PrimitiveType::Byte => "B".to_owned(),
                PrimitiveType::Short => "S".to_owned(),
                PrimitiveType::Integer => "I".to_owned(),
                PrimitiveType::Long => "J".to_owned(),
                PrimitiveType::Float => "F".to_owned(),
                PrimitiveType::Double => "D".to_owned(),
                PrimitiveType::Boolean => "Z".to_owned(),
                PrimitiveType::Char => "C".to_owned(),
            },
            Type::Class { ref name } => format!("L{}", name),
            Type::Array { ref inner } => format!("[{}", inner.descriptor()),
        }
    }
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn write_descriptor_for_array_of_booleans_is_correct() {
        let ty = Type::Array { inner: Box::new(Type::Primitive(PrimitiveType::Boolean)) };
        assert_eq!(ty.descriptor(), "[Z");
    }

    #[test]
    fn read_descriptor_for_array_of_booleans_is_correct() {
        assert_eq!(Type::parse_descriptor("[Z"),
            Some(Type::Array { inner: Box::new(Type::Primitive(PrimitiveType::Boolean)) }))
    }

    #[test]
    fn read_descriptor_for_array_of_objects_is_correct() {
        assert_eq!(Type::parse_descriptor("[LFooBar"),
            Some(Type::Array { inner: Box::new(Type::Class { name: "FooBar".to_owned() }) }));
    }
}
