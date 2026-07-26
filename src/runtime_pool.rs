use jloader::constants::PoolConstants;
use jloader::constants::{self, NameAndType};
use jloader::descriptors::FieldDescriptor;

use crate::data_types::{Decimal, StaticConstant, SymbolicRef};

#[derive(Clone, Debug)]
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
                                None,
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
