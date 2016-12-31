use {ClassFile, Constant};

use std::fmt;

pub fn class(class: &ClassFile,
             fmt: &mut fmt::Formatter) -> fmt::Result {
    for (_, constant) in class.constant_pool.iter() {
        self::constant(constant, class, fmt)?;
        writeln!(fmt, "")?;
    }

    Ok(())
}

pub fn constant(constant: &Constant,
                class_file: &ClassFile,
                fmt: &mut fmt::Formatter) -> fmt::Result {
    match *constant {
        Constant::Utf8(ref s) => write!(fmt, "Utf8({:?})", s),
        Constant::MethodRef { class, name_and_type } => {
            let class = class_file.constant_pool.get(class).unwrap();
            let name_and_type = class_file.constant_pool.get(name_and_type).unwrap();
            write!(fmt, "MethodRef {{ class: ")?;
            self::constant(class, class_file, fmt)?;
            write!(fmt, ", name_and_type: ")?;
            self::constant(name_and_type, class_file, fmt)?;
            write!(fmt, " }}")
        }
        Constant::InterfaceMethodRef { class, name_and_type } => {
            unimplemented!();
        },
        Constant::FieldRef { class, name_and_type } => {
            unimplemented!();
        },
        Constant::NameAndType { name, descriptor } => {
            let name = class_file.constant_pool.get(name).unwrap();
            let descriptor = class_file.constant_pool.get(descriptor).unwrap();
            write!(fmt, "NameAndType {{ name: ")?;
            self::constant(name, class_file, fmt)?;
            write!(fmt, ", descriptor: ")?;
            self::constant(descriptor, class_file, fmt)?;
            write!(fmt, " }}")
        },
        Constant::Class { name } => {
            let name = class_file.constant_pool.get(name).unwrap();
            write!(fmt, "Class({:?})", name)
        },
        Constant::String { index } => {
            unimplemented!();
        },
        Constant::Integer(i) => write!(fmt, "Integer({})", i),
        Constant::Long(l) => write!(fmt, "Long({})", l),
        Constant::Float(f) => write!(fmt, "Float({})", f),
        Constant::Double(d) => write!(fmt, "Double({})", d),
    }
}
