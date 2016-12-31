use {AccessFlags, Type};

#[derive(Clone, Debug)]
pub struct Method
{
    pub access_flags: AccessFlags,
    pub name: String,
    pub signature: MethodSignature,
}

/// A method signature.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MethodSignature
{
    /// The return type.
    pub ret: Type,
    /// The parameter types.
    pub parameters: Vec<Type>,
}

impl MethodSignature
{
    pub fn parse_descriptor(s: &str) -> Option<Self> {
        let closing_paren_idx = s.chars().position(|c| c == ')').unwrap();
        let (mut before_paren, mut after_paren) = s.split_at(closing_paren_idx);

        before_paren = &before_paren[1..]; // Eat the opening parenthesis.
        after_paren = &after_paren[1..]; // Eat the closing parenthesis

        // Eat semicolons.
        if before_paren.chars().last() == Some(';') {
            before_paren = &before_paren[0..before_paren.len()-1];
        }
        if after_paren.chars().last() == Some(';') {
            after_paren = &after_paren[0..after_paren.len()-1];
        }

        let ret = match Type::parse_descriptor(after_paren) {
            Some(ty) => ty,
            None => return None,
        };

        let mut parameters = Vec::new();
        let mut chars_read = 0;

        while chars_read != before_paren.len() {
            let param_ty = Type::parse_descriptor(&before_paren[chars_read..]).unwrap();

            chars_read += param_ty.descriptor().len();
            parameters.push(param_ty);
        }

        Some(MethodSignature {
            ret: ret,
            parameters: parameters,
        })
    }

    pub fn descriptor(&self) -> String {
        let parameter_descriptors: Vec<_> = self.parameters.iter().map(Type::descriptor).collect();
        let param_descriptor = format!("{};", parameter_descriptors.join(""));
        format!("({}){};", param_descriptor, self.ret.descriptor())
    }
}

#[cfg(test)]
mod test
{
    use {Type, PrimitiveType};
    use super::*;

    #[test]
    fn basic_signature_has_correct_descriptor() {
        let sig = MethodSignature {
            ret: Type::Class { name: "java/lang/Object".to_owned() },
            parameters: vec![
                Type::Primitive(PrimitiveType::Integer),
                Type::Primitive(PrimitiveType::Double),
                Type::Class { name: "java/lang/Thread".to_owned() },
            ],
        };

        assert_eq!(sig.descriptor(), "(IDLjava/lang/Thread;)Ljava/lang/Object;");
    }

    #[test]
    fn can_parse_basic_sig() {
        let sig = MethodSignature::parse_descriptor("(IDLjava/lang/Thread;)Ljava/lang/Object;");
        assert_eq!(sig, Some(MethodSignature {
            ret: Type::Class { name: "java/lang/Object".to_owned() },
            parameters: vec![
                Type::Primitive(PrimitiveType::Integer),
                Type::Primitive(PrimitiveType::Double),
                Type::Class { name: "java/lang/Thread".to_owned() },
            ],
        }));
    }

    #[test]
    fn can_parse_void_sig() {
        let sig = MethodSignature::parse_descriptor("()V");
        assert_eq!(sig, Some(MethodSignature {
            ret: Type::Primitive(PrimitiveType::Void),
            parameters: Vec::new(),
        }));
    }
}
