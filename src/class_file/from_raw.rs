use {ClassFile, Method, MethodSignature, Error, ErrorKind};
use raw;

use std::collections::HashMap;

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
    let descriptor = class_file.get_constant(method.descriptor).expect_utf8()?;

    let signature = match MethodSignature::parse_descriptor(&descriptor) {
        Some(sig) => sig,
        None => return Err(ErrorKind::MalformedFile(format!("'{}' is not a valid method descriptor", descriptor)).into()),
    };

    let attributes: Result<HashMap<String, Vec<u8>>, Error> = method.attributes.items.iter().map(|attr| {
        let name = class_file.get_constant(attr.name).expect_utf8()?;
        let data = attr.attribute.items.clone();
        Ok((name, data))
    }).collect();
    let mut attributes: HashMap<String, Vec<u8>> = attributes?;

    let bitcode = if let Some(bitcode) = attributes.remove("Code") {
        bitcode
    } else {
        return Err(ErrorKind::MalformedFile(format!("method '{}' with no bitcode", name)).into());
    };

    if !attributes.is_empty() {
        let attribute_names: Vec<_> = attributes.into_iter().map(|(k,_)| k).collect();
        let unknown_attributes = attribute_names.join(", ");
        return Err(ErrorKind::MalformedFile(format!("unknown attributes on method '{}': {}",
                                                    name, unknown_attributes)).into());
    }

    Ok(Method {
        access_flags: method.access_flags,
        name: name,
        signature: signature,
        bitcode: bitcode,
    })
}
