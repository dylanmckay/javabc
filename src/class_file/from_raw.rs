use {ClassFile, Method, Error};
use raw;

pub fn from_raw(raw: raw::ClassFile) -> Result<ClassFile, Error> {
    let methods = self::methods(&raw)?;

    Ok(ClassFile {
        access_flags: raw.access_flags,
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
