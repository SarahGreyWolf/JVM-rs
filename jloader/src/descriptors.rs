use crate::constants::Utf8;

#[derive(Debug, Clone, PartialEq)]
/// [FieldDescriptors](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A677%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C167%2Cnull%5D)
pub enum FieldDescriptor {
    BaseType(String),
    // Object Type with ClassName
    ObjectType(String),
    ArrayType(String),
}

impl From<FieldDescriptor> for String {
    fn from(desc: FieldDescriptor) -> Self {
        match desc {
            FieldDescriptor::BaseType(r#type) => r#type,
            FieldDescriptor::ObjectType(object) => object,
            FieldDescriptor::ArrayType(object) => format!("{object}[]"),
        }
    }
}

impl From<Utf8> for Option<Vec<FieldDescriptor>> {
    fn from(value: Utf8) -> Self {
        let mut descriptors = vec![];
        let mut peekable = value.bytes.iter().peekable();
        let mut in_object = false;
        let mut is_array = false;
        let mut name = String::new();
        while let Some(c) = peekable.peek() {
            let c = **c;
            if c == b';' {
                in_object = false;
                if is_array {
                    descriptors.push(FieldDescriptor::ArrayType(name));
                    is_array = false;
                } else {
                    descriptors.push(FieldDescriptor::ObjectType(name));
                }
                name = String::new();
                peekable.next();
                continue;
            }
            if in_object {
                name.push(c as char);
                peekable.next();
                continue;
            }
            match c {
                b'[' => is_array = true,
                b'L' => in_object = true,
                b'B' => {
                    if is_array {
                        is_array = false;
                        descriptors.push(FieldDescriptor::ArrayType("byte".into()))
                    } else {
                        descriptors.push(FieldDescriptor::BaseType("byte".into()))
                    }
                }
                b'C' => {
                    if is_array {
                        is_array = false;
                        descriptors.push(FieldDescriptor::ArrayType("char".into()))
                    } else {
                        descriptors.push(FieldDescriptor::BaseType("char".into()))
                    }
                }
                b'D' => {
                    if is_array {
                        is_array = false;
                        descriptors.push(FieldDescriptor::ArrayType("double".into()))
                    } else {
                        descriptors.push(FieldDescriptor::BaseType("double".into()))
                    }
                }
                b'F' => {
                    if is_array {
                        is_array = false;
                        descriptors.push(FieldDescriptor::ArrayType("float".into()))
                    } else {
                        descriptors.push(FieldDescriptor::BaseType("float".into()))
                    }
                }
                b'I' => {
                    if is_array {
                        is_array = false;
                        descriptors.push(FieldDescriptor::ArrayType("int".into()))
                    } else {
                        descriptors.push(FieldDescriptor::BaseType("int".into()))
                    }
                }
                b'J' => {
                    if is_array {
                        is_array = false;
                        descriptors.push(FieldDescriptor::ArrayType("long".into()))
                    } else {
                        descriptors.push(FieldDescriptor::BaseType("long".into()))
                    }
                }
                b'S' => {
                    if is_array {
                        is_array = false;
                        descriptors.push(FieldDescriptor::ArrayType("short".into()))
                    } else {
                        descriptors.push(FieldDescriptor::BaseType("short".into()))
                    }
                }
                b'Z' => {
                    if is_array {
                        is_array = false;
                        descriptors.push(FieldDescriptor::ArrayType("boolean".into()))
                    } else {
                        descriptors.push(FieldDescriptor::BaseType("boolean".into()))
                    }
                }
                _ => return None,
            }
            peekable.next();
        }

        Some(descriptors)
    }
}

#[derive(Debug, Clone, PartialEq)]
/// [MethodDescriptors](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A415%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C279.293%2Cnull%5D)
pub enum MethodDescriptor {
    ParameterDescriptor(FieldDescriptor),
    ReturnDescriptor(FieldDescriptor),
    VoidReturn,
}

impl From<MethodDescriptor> for String {
    fn from(desc: MethodDescriptor) -> String {
        match desc {
            MethodDescriptor::ParameterDescriptor(fd) => fd.into(),
            MethodDescriptor::ReturnDescriptor(fd) => fd.into(),
            MethodDescriptor::VoidReturn => "void".into(),
        }
    }
}

impl From<Utf8> for Option<Vec<MethodDescriptor>> {
    fn from(value: Utf8) -> Self {
        let mut descriptors = vec![];
        let mut peekable = value.bytes.iter().peekable();
        let mut in_params = false;
        let mut in_return = false;
        let mut collected = String::new();
        while let Some(c) = peekable.peek() {
            let c = **c;
            if c == b')' {
                in_params = false;
                let f_descriptors: Option<Vec<FieldDescriptor>> =
                    Option::from(Utf8::from(collected.as_str()));
                if let Some(f_descriptors) = f_descriptors {
                    for desc in f_descriptors {
                        descriptors.push(MethodDescriptor::ParameterDescriptor(desc));
                    }
                }
                collected = String::new();
                peekable.next();
                if peekable.peek() != Some(&&b'V') {
                    in_return = true;
                }
                continue;
            }
            if in_params || in_return {
                collected.push(c as char);
                peekable.next();
                continue;
            }
            match c {
                b'(' => {
                    in_params = true;
                }
                b'V' => descriptors.push(MethodDescriptor::VoidReturn),
                _ => return None,
            }
            peekable.next();
        }
        let f_descriptors: Option<Vec<FieldDescriptor>> =
            Option::from(Utf8::from(collected.as_str()));
        if let Some(f_descriptors) = f_descriptors {
            for desc in f_descriptors {
                descriptors.push(MethodDescriptor::ReturnDescriptor(desc));
            }
        }

        Some(descriptors)
    }
}
