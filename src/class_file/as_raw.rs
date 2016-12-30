use {ClassFile, ConstantPool, ConstantIndex, Constant};
use raw;

pub fn as_raw(class_file: &ClassFile) -> raw::ClassFile {
    raw::ClassFile {
        magic: raw::class_file::MAGIC,
        // FIXME: use correct versions.
        minor_version: 0,
        major_version: 0,
        constant_pool: constant_pool(&class_file.constant_pool),
        access_flags: class_file.access_flags,
        attributes: Vec::new().into(),
        fields: Vec::new().into(),
        interfaces: Vec::new().into(),
        methods: Vec::new().into(),
        super_class: unimplemented!(),
        this_class: unimplemented!(),
    }
}

fn constant_pool(constant_pool: &ConstantPool)
    -> raw::OneBasedArray<raw::Constant, u16> {
    let mut sorted_pool: Vec<(ConstantIndex, raw::Constant)> = constant_pool.iter()
        .map(|(k,v)| (k.clone(), self::constant(v)))
        .collect();

    sorted_pool.sort_by_key(|&(k,_)| k.0);

    let values: Vec<raw::Constant> = sorted_pool.into_iter().map(|(_,v)| v).collect();
    values.into()
}

fn constant(constant: &Constant) -> raw::Constant {
    match *constant {
        Constant::Utf8(ref s) => raw::Constant::Utf8 { text: s.clone() },
        Constant::Integer(i) => raw::Constant::Integer(i),
    }
}
