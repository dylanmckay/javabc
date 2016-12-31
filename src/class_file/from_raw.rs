use {ClassFile, ConstantPool, Constant, ConstantIndex, Method, Error};
use raw;

pub fn from_raw(raw: raw::ClassFile) -> Result<ClassFile, Error> {
    let methods = self::methods(&raw)?;

    Ok(ClassFile {
        access_flags: raw.access_flags,
        constant_pool: self::constant_pool(raw.constant_pool),
        methods: methods,
    })
}

fn methods(class_file: &raw::ClassFile) -> Result<Vec<Method>, Error> {
    class_file.methods.items.iter().map(|method| self::method(method, class_file)).collect()
}

fn method(method: &raw::Method,
          class_file: &raw::ClassFile) -> Result<Method, Error> {
    let name = class_file.get_constant(method.name).expect_utf8()?;

    Ok(Method {
        name: name,
    })
}

fn constant_pool(raw: raw::OneBasedArray<raw::Constant, u16>)
    -> ConstantPool {
    let mut pool = ConstantPool::new();

    for constant in raw.items {
        pool.insert(self::constant(constant));
    }

    pool
}

fn constant(raw: raw::Constant) -> Constant {
    match raw {
        raw::Constant::MethodRef { class, name_and_type } => {
            Constant::MethodRef { class: ConstantIndex(class.0 as _),
                                  name_and_type: ConstantIndex(name_and_type.0 as _) }
        },
        raw::Constant::InterfaceMethodRef { class, name_and_type } => unimplemented!(),
        raw::Constant::FieldRef { class, name_and_type } => unimplemented!(),
        raw::Constant::NameAndType { name, descriptor } => {
            Constant::NameAndType { name: ConstantIndex(name.0 as _),
                                    descriptor: ConstantIndex(descriptor.0 as _) }
        },
        raw::Constant::Class { name } => {
            Constant::Class { name: ConstantIndex(name.0 as _) }
        },
        raw::Constant::Utf8 { text } => Constant::Utf8(text),
        raw::Constant::String { index } => unimplemented!(),
        raw::Constant::Integer(i) => Constant::Integer(i),
        raw::Constant::Long(l) => unimplemented!(),
        raw::Constant::Float(f) => unimplemented!(),
        raw::Constant::Double(d) => unimplemented!(),
    }
}
