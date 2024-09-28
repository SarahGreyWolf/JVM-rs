use jloader::constants::PoolConstants;
use jloader::constants::{self, NameAndType};
use jloader::descriptors::FieldDescriptor;

#[derive(Clone, Debug, PartialEq)]
// https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A4175%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C295%2Cnull%5D
pub enum SymbolicRef {
    /*
    •   A symbolic reference to a class or interface is derived from a
        CONSTANT_Class_info structure (§4.4.1). Such a reference gives the name of
        the class or interface in the following form:
    –   For a nonarray class or an interface, the name is the binary name (§4.2.1) of
        the class or interface.
    –   For an array class of n dimensions, the name begins with n occurrences of the
        ASCII [ character followed by a representation of the element type:
        › If the element type is a primitive type, it is represented by the corresponding
          field descriptor (§4.3.2).
        › Otherwise, if the element type is a reference type, it is represented by the
          ASCII L character followed by the binary name of the element type followed
          by the ASCII ; character.
    */
    Class(String),
    Interface(String),
    Array(FieldDescriptor),
    /*
       A symbolic reference to a field of a class or an interface is derived from a
       CONSTANT_Fieldref_info structure (§4.4.2). Such a reference gives the name
       and descriptor of the field, as well as a symbolic reference to the class or interface
       in which the field is to be found.
    */
    // Name, Descriptor, index to SymbolicRef to Class/Interface
    Field(String, String, usize),
    // Name, Descriptor, index to SymbolicRef to Class
    ClassMethod(String, String, usize),
    // Name, Descriptor, index to SymbolicRef to Class/Interface
    InterfaceMethod(String, String, usize),
    // A field of a class or interface, or a method of a class, or a
    // method of an interface
    // index to SymbolicRef
    MethodHandle(usize),
    // Method Descriptor
    MethodType(String),
    /*
    • A symbolic reference to a dynamically-computed constant is derived from a
    CONSTANT_Dynamic_info structure (§4.4.10). Such a reference gives:
        – the index of a symbolic reference to a method handle, which will be invoked to compute
        the constant's value;
        – a sequence of symbolic references and static constants, which will serve as
        static arguments when the method handle is invoked;
        – an unqualified name and a field descriptor.
    */
    DynamicConst(usize, Vec<RuntimeConstant>, String, String),
    /*
    • A symbolic reference to a dynamically-computed call site is derived from a
    CONSTANT_InvokeDynamic_info structure (§4.4.10). Such a reference gives:
        – the index of a symbolic reference to a method handle, which will be invoked in the course
        of an invokedynamic instruction (§invokedynamic) to compute an instance of
        java.lang.invoke.CallSite;
        – a sequence of symbolic references and static constants, which will serve as
        static arguments when the method handle is invoked;
        – an unqualified name and a method descriptor.
    */
    DynamicCall(usize, Vec<RuntimeConstant>, String, String),
    Null,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StaticConstant {
    String(String),
    Integer(i32),
    Float(Decimal),
    Long(i64),
    Double(Decimal),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Decimal {
    Float(f32),
    Double(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum RuntimeConstant {
    SymbolicRef(SymbolicRef),
    StaticConstant(StaticConstant),
    Spacer,
}

impl RuntimeConstant {
    pub fn from_constant_pool(pool: &[PoolConstants]) -> Vec<RuntimeConstant> {
        let mut runtime_constants =
            vec![RuntimeConstant::SymbolicRef(SymbolicRef::Null); pool.len()];

        for (index, constant) in pool.iter().enumerate() {
            match constant {
                PoolConstants::String(constants::String { string_index }) => {
                    if let PoolConstants::Utf8(ref name) =
                        pool[*string_index as usize]
                    {
                        runtime_constants.insert(
                            index,
                            RuntimeConstant::StaticConstant(
                                StaticConstant::String(String::from(name)),
                            ),
                        );
                    } else {
                        panic!(
                            "Value at {string_index} in constant pool was a {:?} and not a Utf8",
                            pool[*string_index as usize]
                        );
                    }
                }
                PoolConstants::Integer(constants::Integer { bytes }) => {
                    runtime_constants.insert(
                        index,
                        RuntimeConstant::StaticConstant(
                            StaticConstant::Integer(*bytes as i32),
                        ),
                    )
                }
                PoolConstants::Float(constants::Float { bytes }) => {
                    if bytes & 0x7f800000 == 0x7f800000 {
                        runtime_constants.insert(
                            index,
                            RuntimeConstant::StaticConstant(
                                StaticConstant::Float(Decimal::Float(
                                    f32::INFINITY,
                                )),
                            ),
                        );
                    }

                    if bytes & 0xff800000 == 0xff800000 {
                        runtime_constants.insert(
                            index,
                            RuntimeConstant::StaticConstant(
                                StaticConstant::Float(Decimal::Float(
                                    f32::NEG_INFINITY,
                                )),
                            ),
                        );
                    }

                    let range1 = (0x7f800001..=0x7fffffff);
                    let range2 = (0xff800001..=0xffffffff);

                    if range1.contains(bytes) || range2.contains(bytes) {
                        runtime_constants.insert(
                            index,
                            RuntimeConstant::StaticConstant(
                                StaticConstant::Float(Decimal::Float(f32::NAN)),
                            ),
                        );
                    }

                    let s: i32 = if (bytes >> 31) == 0 { 1 } else { -1 };
                    let e: i32 = ((bytes >> 23) & 0xFF) as i32;
                    let m: i32 = if e == 0 {
                        ((bytes & 0x7fffff) << 1) as i32
                    } else {
                        ((bytes & 0x7fffff) | 0x800000) as i32
                    };

                    let float = s as f32 * m as f32 * 2f32.powi(e - 150);

                    runtime_constants.insert(
                        index,
                        RuntimeConstant::StaticConstant(StaticConstant::Float(
                            Decimal::Float(float),
                        )),
                    )
                }
                PoolConstants::Long(constants::Long {
                    high_bytes,
                    low_bytes,
                }) => runtime_constants.insert(
                    index,
                    RuntimeConstant::StaticConstant(StaticConstant::Long(
                        ((*high_bytes as u64).overflowing_shl(32).0
                            + *low_bytes as u64) as i64,
                    )),
                ),
                PoolConstants::Double(constants::Double {
                    high_bytes,
                    low_bytes,
                }) => {
                    let bytes = (*high_bytes as u64).overflowing_shl(32).0
                        + *low_bytes as u64;
                    if bytes & 0x7ff0000000000000 == 0x7ff0000000000000 {
                        runtime_constants.insert(
                            index,
                            RuntimeConstant::StaticConstant(
                                StaticConstant::Float(Decimal::Double(
                                    f64::INFINITY,
                                )),
                            ),
                        );
                    }

                    if bytes & 0xfff0000000000000 == 0xfff0000000000000 {
                        runtime_constants.insert(
                            index,
                            RuntimeConstant::StaticConstant(
                                StaticConstant::Float(Decimal::Double(
                                    f64::NEG_INFINITY,
                                )),
                            ),
                        );
                    }

                    let range1 = (0x7ff0000000000001..=0x7fffffffffffffff);
                    let range2 = (0xfff0000000000001..=0xffffffffffffffff);

                    if range1.contains(&bytes) || range2.contains(&bytes) {
                        runtime_constants.insert(
                            index,
                            RuntimeConstant::StaticConstant(
                                StaticConstant::Double(Decimal::Double(
                                    f64::NAN,
                                )),
                            ),
                        );
                    }

                    let s: i64 = if (bytes >> 63) == 0 { 1 } else { -1 };
                    let e: i64 = ((bytes >> 52) & 0x7ff) as i64;
                    let m: i64 = if e == 0 {
                        ((bytes & 0xfffffffffffff) << 1) as i64
                    } else {
                        ((bytes & 0xfffffffffffff) | 0x10000000000000) as i64
                    };

                    let double =
                        s as f64 * m as f64 * 2f64.powi((e - 1075) as i32);

                    runtime_constants.insert(
                        index,
                        RuntimeConstant::StaticConstant(
                            StaticConstant::Double(Decimal::Double(double)),
                        ),
                    )
                }
                PoolConstants::Class(constants::Class { name_index }) => {
                    if let PoolConstants::Utf8(ref name) =
                        pool[*name_index as usize]
                    {
                        runtime_constants.insert(
                            index,
                            RuntimeConstant::SymbolicRef(SymbolicRef::Class(
                                String::from(name),
                            )),
                        );
                    } else {
                        panic!(
                            "Value at {name_index} in constant pool was a {:?} and not a Utf8",
                            pool[*name_index as usize]
                        );
                    }
                }
                // PoolConstants::Fieldref(_) => todo!(),
                PoolConstants::Methodref(constants::Methodref {
                    class_index,
                    name_and_type_index,
                }) => {
                    let PoolConstants::Class(ref class) =
                        pool[*class_index as usize]
                    else {
                        panic!(
                            "PoolConstant at [{class_index}] was not a Class"
                        );
                    };
                    let PoolConstants::NameAndType(NameAndType {
                        name_index,
                        descriptor_index,
                    }) = pool[*name_and_type_index as usize]
                    else {
                        panic!(
                            "PoolConstant at [{name_and_type_index}] was not a NameAndType"
                        );
                    };
                    let PoolConstants::Utf8(ref name) =
                        pool[name_index as usize]
                    else {
                        panic!("PoolConstant at [{name_index}] was not a Utf8");
                    };
                    let PoolConstants::Utf8(ref descriptor) =
                        pool[descriptor_index as usize]
                    else {
                        panic!(
                            "PoolConstant at [{descriptor_index}] was not a Utf8"
                        );
                    };
                    runtime_constants.insert(
                        index,
                        RuntimeConstant::SymbolicRef(SymbolicRef::ClassMethod(
                            name.into(),
                            descriptor.into(),
                            *class_index as usize,
                        )),
                    )
                }
                // PoolConstants::InterfaceMethodref(_) => todo!(),
                // PoolConstants::MethodHandle(_) => todo!(),
                // PoolConstants::MethodType(_) => todo!(),
                // PoolConstants::Dynamic(_) => todo!(),
                // PoolConstants::InvokeDynamic(_) => todo!(),
                unknown => {
                    runtime_constants.insert(index, RuntimeConstant::Spacer);
                    println!("No runtime constant existed for {unknown:?} or is unimplemented")
                }
            }
        }

        runtime_constants
    }
}
