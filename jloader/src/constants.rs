use std::{error::Error, io::Cursor, str::from_utf8};

use byteorder::{ReadBytesExt, BE};

use crate::errors::class_loading::{LoadingCause, LoadingError};

/// [The Constant Pool](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A2201%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C256%2Cnull%5D)
#[derive(Clone, Debug)]
pub enum ConstantPool {
    Utf8(Utf8),
    Integer(Integer),
    Float(Float),
    Long(Long),
    Double(Double),
    Class(Class),
    String(String),
    Fieldref(Fieldref),
    Methodref(Methodref),
    InterfaceMethodref(InterfaceMethodref),
    NameAndType(NameAndType),
    MethodHandle(MethodHandle),
    MethodType(MethodType),
    Dynamic(Dynamic),
    InvokeDynamic(InvokeDynamic),
    Module(Module),
    Package(Package),
    Unknown,
}

#[repr(u8)]
pub enum Tags {
    Utf8 = 1,
    Integer = 3,
    Float = 4,
    Long = 5,
    Double = 6,
    Class = 7,
    String = 8,
    Fieldref = 9,
    Methodref = 10,
    InterfaceMethodref = 11,
    NameAndType = 12,
    MethodHandle = 15,
    MethodType = 16,
    Dynamic = 17,
    InvokeDynamic = 18,
    Module = 19,
    Package = 20,
    Unknown = 128,
}

impl From<u8> for Tags {
    fn from(value: u8) -> Self {
        match value {
            1 => Tags::Utf8,
            3 => Tags::Integer,
            4 => Tags::Float,
            5 => Tags::Long,
            6 => Tags::Double,
            7 => Tags::Class,
            8 => Tags::String,
            9 => Tags::Fieldref,
            10 => Tags::Methodref,
            11 => Tags::InterfaceMethodref,
            12 => Tags::NameAndType,
            15 => Tags::MethodHandle,
            16 => Tags::MethodType,
            17 => Tags::Dynamic,
            18 => Tags::InvokeDynamic,
            19 => Tags::Module,
            20 => Tags::Package,
            _ => Tags::Unknown,
        }
    }
}

#[derive(Clone)]
/// [Utf8 Constant](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A636%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C438%2Cnull%5D)
pub struct Utf8 {
    /** The value of the length item gives the number of bytes in the bytes array (not
     *  the length of the resulting string).
     */
    pub length: u16,
    /** The bytes array contains the bytes of the string.
     *  No byte may have the value (byte)0.
     *  No byte may lie in the range (byte)0xf0 to (byte)0xff.
     */
    pub bytes: Vec<u8>,
}

impl From<&str> for Utf8 {
    fn from(input: &str) -> Self {
        Utf8 {
            length: input.len() as u16,
            bytes: input.as_bytes().to_vec(),
        }
    }
}

impl Utf8 {
    pub fn new(cursor: &mut Cursor<&[u8]>) -> Utf8 {
        let length = cursor.read_u16::<BE>().unwrap();
        Utf8 {
            length,
            bytes: {
                let mut bytes = Vec::with_capacity(length as usize);
                for _ in 0..length {
                    bytes.push(cursor.read_u8().unwrap());
                }
                bytes
            },
        }
    }
}

impl From<&Utf8> for std::string::String {
    fn from(value: &Utf8) -> Self {
        std::string::String::from(from_utf8(&value.bytes).unwrap_or("Could not create from utf8"))
    }
}

impl std::fmt::Debug for Utf8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Kotlin had some weird UTF8 constant that couldn't actually be turned into UTF8 so
        write!(f, "\"{}\"", std::string::String::from(self))
    }
}

#[derive(Clone, Debug)]
/// [Integer Constant](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A653%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C136.8%2Cnull%5D)
pub struct Integer {
    /**
     * **bytes**\
     *  The bytes item of the CONSTANT_Integer_info structure represents the value
     *  of the int constant. The bytes of the value are stored in big-endian (high byte
     *  first) order.
     */
    pub bytes: u32,
}

impl Integer {
    pub fn new(bytes: u32) -> Integer { Integer { bytes } }
}

#[derive(Clone, Debug)]
/// [Float Constant](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A653%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C136.8%2Cnull%5D)
// TODO: There is a LOT of stuff I need to consider for the actual VM, but not now
pub struct Float {
    /**
     * **bytes**\
     *  The bytes item of the CONSTANT_Float_info structure represents the value of
     *  the float constant in IEEE 754 binary32 floating-point format (§2.3.2). The
     *  bytes of the item are stored in big-endian (high byte first) order.
     *  The value represented by the CONSTANT_Float_info structure is determined
     *  as follows. The bytes of the value are first converted into an int constant bits.
     *  Then:\
     *  • If bits is 0x7f800000, the float value will be positive infinity.\
     *  • If bits is 0xff800000, the float value will be negative infinity.\
     *  • If bits is in the range 0x7f800001 through 0x7fffffff or in the range
     *  0xff800001 through 0xffffffff, the float value will be NaN.\
     *  • In all other cases, let s, e, and m be three values that might be computed from
     *  bits:\
     *  int s = ((bits >> 31) == 0) ? 1 : -1;\
     *  int e = ((bits >> 23) & 0xff);\
     *  int m = (e == 0) ?\
     *  (bits & 0x7fffff) << 1 :\
     *  (bits & 0x7fffff) | 0x800000;
     */
    pub bytes: u32,
}

impl Float {
    pub fn new(bytes: u32) -> Float { Float { bytes } }
}

#[derive(Clone, Debug)]
/// [Long Constant](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A458%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C564%2Cnull%5D)
// TODO: There is a LOT of stuff I need to consider for the actual VM, but not now
pub struct Long {
    /**
     * **high_bytes**\
     *  The unsigned high_bytes and low_bytes items of the CONSTANT_Long_info
     *  structure together represent the value of the long constant\
     *  ((long) high_bytes << 32) + low_bytes\
     *  where the bytes of each of high_bytes and low_bytes are stored in big-endian\
     *  (high byte first) order.
     *
     *  The high_bytes and low_bytes items of the CONSTANT_Double_info
     *  structure together represent the double value in IEEE 754 binary64 floating-
     *  point format (§2.3.2). The bytes of each item are stored in big-endian (high
     *  byte first) order.
     *  The value represented by the CONSTANT_Double_info structure is determined
     *  as follows. The high_bytes and low_bytes items are converted into the long
     *  constant bits, which is equal to\
     *  ((long) high_bytes << 32) + low_bytes\
     *  Then:\
     *  • If bits is 0x7ff0000000000000L, the double value will be positive infinity.\
     *  • If bits is 0xfff0000000000000L, the double value will be negative infinity.\
     *  • If bits is in the range 0x7ff0000000000001L through 0x7fffffffffffffffL\
     *  or in the range 0xfff0000000000001L through 0xffffffffffffffffL, the
     *  double value will be NaN.\
     *  • In all other cases, let s, e, and m be three values that might be computed from\
     *  bits:\
     *  int s = ((bits >> 63) == 0) ? 1 : -1;\
     *  int e = (int)((bits >> 52) & 0x7ffL);\
     *  long m = (e == 0) ?\
     *  (bits & 0xfffffffffffffL) << 1 :\
     *  (bits & 0xfffffffffffffL) | 0x10000000000000L;\
     *  Then the floating-point value equals the double value of the mathematical
     *  expression s · m · 2e-1075.
     */
    pub high_bytes: u32,
    /// **low_bytes**
    pub low_bytes: u32,
}

impl Long {
    pub fn new(high_bytes: u32, low_bytes: u32) -> Long {
        Long {
            high_bytes,
            low_bytes,
        }
    }
}

#[derive(Clone, Debug)]
/// [Double Constant](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A458%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C564%2Cnull%5D)
// TODO: There is a LOT of stuff I need to consider for the actual VM, but not now
pub struct Double {
    /**
     * **high_bytes**\
     *  The unsigned high_bytes and low_bytes items of the CONSTANT_Long_info
     *  structure together represent the value of the long constant\
     *  ((long) high_bytes << 32) + low_bytes\
     *  where the bytes of each of high_bytes and low_bytes are stored in big-endian\
     *  (high byte first) order.
     *
     *  The high_bytes and low_bytes items of the CONSTANT_Double_info
     *  structure together represent the double value in IEEE 754 binary64 floating-
     *  point format (§2.3.2). The bytes of each item are stored in big-endian (high
     *  byte first) order.\
     *  The value represented by the CONSTANT_Double_info structure is determined
     *  as follows. The high_bytes and low_bytes items are converted into the long
     *  constant bits, which is equal to\
     *  ((long) high_bytes << 32) + low_bytes\
     *  Then:\
     *  • If bits is 0x7ff0000000000000L, the double value will be positive infinity.\
     *  • If bits is 0xfff0000000000000L, the double value will be negative infinity.\
     *  • If bits is in the range 0x7ff0000000000001L through 0x7fffffffffffffffL\
     *  or in the range 0xfff0000000000001L through 0xffffffffffffffffL, the
     *  double value will be NaN.\
     *  • In all other cases, let s, e, and m be three values that might be computed from
     *  bits:\
     *  int s = ((bits >> 63) == 0) ? 1 : -1;\
     *  int e = (int)((bits >> 52) & 0x7ffL);\
     *  long m = (e == 0) ?\
     *  (bits & 0xfffffffffffffL) << 1 :\
     *  (bits & 0xfffffffffffffL) | 0x10000000000000L;\
     *  Then the floating-point value equals the double value of the mathematical
     *  expression s · m · 2e-1075.
     */
    pub high_bytes: u32,
    /// **low_bytes**
    pub low_bytes: u32,
}

impl Double {
    pub fn new(high_bytes: u32, low_bytes: u32) -> Double {
        Double {
            high_bytes,
            low_bytes,
        }
    }
}

#[derive(Clone, Debug)]
/// [Class Constant](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A646%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C396%2Cnull%5D)
pub struct Class {
    /**
     * **name_index**\
     *  The value of the name_index item must be a valid index into the
     *  constant_pool table. The constant_pool entry at that index must be a
     *  CONSTANT_Utf8_info structure (§4.4.7) representing a valid binary class or
     *  interface name encoded in internal form (§4.2.1).
     */
    pub name_index: u16,
}

impl Class {
    pub fn new(index: u16) -> Class { Class { name_index: index } }
}

#[derive(Clone, Debug)]
/// [String Constant](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A653%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C388%2Cnull%5D)
pub struct String {
    /**
     * **string_index**\
     *  The value of the string_index item must be a valid index into the
     *  constant_pool table. The constant_pool entry at that index must be a
     *  CONSTANT_Utf8_info structure (§4.4.7) representing the sequence of Unicode
     *  code points to which the String object is to be initialized.
     */
    pub string_index: u16,
}

impl String {
    pub fn new(index: u16) -> String {
        String {
            string_index: index,
        }
    }
}

#[derive(Clone, Debug)]
/// [Fieldref Constant](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A450%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C577%2Cnull%5D)
pub struct Fieldref {
    /**
     * **class_index**\
     *  The value of the class_index item must be a valid index into the
     *  constant_pool table. The constant_pool entry at that index must be a
     *  CONSTANT_Class_info structure (§4.4.1) representing a class or interface type
     *  that has the field or method as a member.
     *  
     *  In a CONSTANT_Fieldref_info structure, the class_index item may be either
     *  a class type or an interface type.
     */
    pub class_index: u16,
    /**
     * **name_and_type_index**\
     *  The value of the name_and_type_index item must be a valid index into
     *  the constant_pool table. The constant_pool entry at that index must be a
     *  CONSTANT_NameAndType_info structure (§4.4.6). This constant_pool entry
     *  indicates the name and descriptor of the field or method.
     *  
     *  In a CONSTANT_Fieldref_info structure, the indicated descriptor must be a
     *  field descriptor (§4.3.2). Otherwise, the indicated descriptor must be a method
     *  descriptor (§4.3.3).
     */
    pub name_and_type_index: u16,
}

impl Fieldref {
    pub fn new(class_index: u16, name_and_type_index: u16) -> Fieldref {
        Fieldref {
            class_index,
            name_and_type_index,
        }
    }
}

#[derive(Clone, Debug)]
/// [Methodref Constant](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A450%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C577%2Cnull%5D)
pub struct Methodref {
    /**
     * **class_index**\
     *  The value of the class_index item must be a valid index into the
     *  constant_pool table. The constant_pool entry at that index must be a
     *  CONSTANT_Class_info structure (§4.4.1) representing a class or interface type
     *  that has the field or method as a member.
     */
    pub class_index: u16,
    /**
     * **name_and_type_index**\
     *   The value of the name_and_type_index item must be a valid index into
     *   the constant_pool table. The constant_pool entry at that index must be a
     *   CONSTANT_NameAndType_info structure (§4.4.6). This constant_pool entry
     *   indicates the name and descriptor of the field or method.
     *   
     *   If the name of the method in a CONSTANT_Methodref_info structure begins
     *   with a '<' ('\u003c'), then the name must be the special name <init>,
     *   representing an instance initialization method (§2.9.1). The return type of such
     *   a method must be void.
     */
    pub name_and_type_index: u16,
}

impl Methodref {
    pub fn new(class_index: u16, name_and_type_index: u16) -> Methodref {
        Methodref {
            class_index,
            name_and_type_index,
        }
    }
}

#[derive(Clone, Debug)]
/// [Methodref Constant](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A450%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C577%2Cnull%5D)
pub struct InterfaceMethodref {
    /**
     * **class_index**\
     *  The value of the class_index item must be a valid index into the
     *  constant_pool table. The constant_pool entry at that index must be a
     *  CONSTANT_Class_info structure (§4.4.1) representing a class or interface type
     *  that has the field or method as a member.
     *  
     *  In a CONSTANT_InterfaceMethodref_info structure, the class_index item
     *  must be an interface type, not a class type.
     */
    pub class_index: u16,
    /**
     * **name_and_type_index**\
     *  The value of the name_and_type_index item must be a valid index into
     *  the constant_pool table. The constant_pool entry at that index must be a
     *  CONSTANT_NameAndType_info structure (§4.4.6). This constant_pool entry
     *  indicates the name and descriptor of the field or method.
     */
    pub name_and_type_index: u16,
}

impl InterfaceMethodref {
    pub fn new(class_index: u16, name_and_type_index: u16) -> InterfaceMethodref {
        InterfaceMethodref {
            class_index,
            name_and_type_index,
        }
    }
}

#[derive(Clone, Debug)]
/// [NameAndType Constant](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A634%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C245%2Cnull%5D)
pub struct NameAndType {
    /**
     * **name_index**\
     *  The value of the name_index item must be a valid index into the
     *  constant_pool table. The constant_pool entry at that index must be a
     *  CONSTANT_Utf8_info structure (§4.4.7) representing either a valid unqualified
     *  name denoting a field or method (§4.2.2), or the special method name <init>
     *  (§2.9.1).
     */
    pub name_index: u16,
    /**
     * **descriptor_index**\
     *  The value of the descriptor_index item must be a valid index into the
     *  constant_pool table. The constant_pool entry at that index must be a
     *  CONSTANT_Utf8_info structure (§4.4.7) representing a valid field descriptor
     *  or method descriptor (§4.3.2, §4.3.3).
     */
    pub descriptor_index: u16,
}

impl NameAndType {
    pub fn new(name_index: u16, descriptor_index: u16) -> NameAndType {
        NameAndType {
            name_index,
            descriptor_index,
        }
    }
}

#[derive(Clone, Debug)]
/// [MethodType Constant](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A668%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C235.18%2Cnull%5D)
pub struct MethodType {
    /**
     * **descriptor_index**\
     *  The value of the descriptor_index item must be a valid index into the
     *  constant_pool table. The constant_pool entry at that index must be a
     *  CONSTANT_Utf8_info structure (§4.4.7) representing a method descriptor
     *  (§4.3.3).
     */
    pub descriptor_index: u16,
}

impl MethodType {
    pub fn new(descriptor_index: u16) -> MethodType { MethodType { descriptor_index } }
}

#[repr(u8)]
#[derive(Clone, Debug)]
/// [MethodHandleKinds](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A2552%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C278%2Cnull%5D)
pub enum MethodHandleKinds {
    GetField = 1,
    GetStatic,
    PutField,
    PutStatic,
    InvokeVirtual,
    InvokeStatic,
    InvokeSpecial,
    NewInvokeSpecial,
    InvokeInterface,
    Unknown,
}

impl From<u8> for MethodHandleKinds {
    fn from(value: u8) -> Self {
        match value {
            1 => MethodHandleKinds::GetField,
            2 => MethodHandleKinds::GetStatic,
            3 => MethodHandleKinds::PutField,
            4 => MethodHandleKinds::PutStatic,
            5 => MethodHandleKinds::InvokeVirtual,
            6 => MethodHandleKinds::InvokeStatic,
            7 => MethodHandleKinds::InvokeSpecial,
            8 => MethodHandleKinds::NewInvokeSpecial,
            9 => MethodHandleKinds::InvokeInterface,
            _ => MethodHandleKinds::Unknown,
        }
    }
}

#[derive(Clone, Debug)]
/// [MethodHandle constant](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A668%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C235.18%2Cnull%5D)
pub struct MethodHandle {
    /**
     * **reference_kind**\
     *   The value of the reference_kind item must be in the range 1 to 9. The
     *   value denotes the kind of this method handle, which characterizes its bytecode
     *   behavior (§5.4.3.5).
     */
    pub reference_kind: MethodHandleKinds,
    /**
     * **reference_index**\
     *   The value of the reference_index item must be a valid index into the
     *   constant_pool table. The constant_pool entry at that index must be as
     *   follows:
     *
     *   • If the value of the reference_kind item is 1 (REF_getField), 2
     *   (REF_getStatic), 3 (REF_putField), or 4 (REF_putStatic), then the
     *   constant_pool entry at that index must be a CONSTANT_Fieldref_info
     *   structure (§4.4.2) representing a field for which a method handle is to be
     *   created.
     *
     *   • If the value of the reference_kind item is 5 (REF_invokeVirtual) or 8
     *   (REF_newInvokeSpecial), then the constant_pool entry at that index must
     *   be a CONSTANT_Methodref_info structure (§4.4.2) representing a class's
     *   method or constructor (§2.9.1) for which a method handle is to be created.
     *
     *   • If the value of the reference_kind item is 6 (REF_invokeStatic)
     *   or 7 (REF_invokeSpecial), then if the class file version number
     *   is less than 52.0, the constant_pool entry at that index must be
     *   a CONSTANT_Methodref_info structure representing a class's method
     *   for which a method handle is to be created; if the class file
     *   version number is 52.0 or above, the constant_pool entry at that
     *   index must be either a CONSTANT_Methodref_info structure or a
     *   CONSTANT_InterfaceMethodref_info structure (§4.4.2) representing a
     *   class's or interface's method for which a method handle is to be created.
     *
     *   • If the value of the reference_kind item is 9 (REF_invokeInterface),
     *   then the constant_pool entry at that index must be a
     *   CONSTANT_InterfaceMethodref_info structure representing an interface's
     *   method for which a method handle is to be created.
     *
     *   If the value of the reference_kind item is 5 (REF_invokeVirtual), 6
     *   (REF_invokeStatic), 7 (REF_invokeSpecial), or 9 (REF_invokeInterface),
     *   the name of the method represented by a CONSTANT_Methodref_info structure
     *   or a CONSTANT_InterfaceMethodref_info structure must not be <init> or
     *   <clinit>.
     *
     *   If the value is 8 (REF_newInvokeSpecial), the name of the method represented
     *   by a CONSTANT_Methodref_info structure must be <init>.
     */
    pub reference_index: u16,
}

impl MethodHandle {
    pub fn new(reference_kind: u8, reference_index: u16) -> MethodHandle {
        MethodHandle {
            reference_kind: MethodHandleKinds::from(reference_kind),
            reference_index,
        }
    }
}

/// [Dynamic Constant](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A3782%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C370.8%2Cnull%5D)
#[derive(Clone, Debug)]
pub struct Dynamic {
    /**
     * **bootstrap_method_attr_index**\
     *  The value of the bootstrap_method_attr_index item must be a valid index
     *  into the bootstrap_methods array of the bootstrap method table of this class
     *  file (§4.7.23).
     *
     *  CONSTANT_Dynamic_info structures are unique in that they are syntactically allowed to
     *  refer to themselves via the bootstrap method table. Rather than mandating that such cycles
     *  are detected when classes are loaded (a potentially expensive check), we permit cycles
     *  initially but mandate a failure at resolution (§5.4.3.6)
     */
    pub bootstrap_method_attr_index: u16,
    /**
     * **name_and_type_index**\
     *  The value of the name_and_type_index item must be a valid index into
     *  the constant_pool table. The constant_pool entry at that index must be a
     *  CONSTANT_NameAndType_info structure (§4.4.6). This constant_pool entry
     *  indicates a name and descriptor.\
     *  In a CONSTANT_Dynamic_info structure, the indicated descriptor must be a field
     *  descriptor (§4.3.2).
     */
    pub name_and_type_index: u16,
}

impl Dynamic {
    pub fn new(bootstrap_method_attr_index: u16, name_and_type_index: u16) -> Dynamic {
        Dynamic {
            bootstrap_method_attr_index,
            name_and_type_index,
        }
    }
}

/// [InvokeDynamic Constant](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A3782%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C370.8%2Cnull%5D)
#[derive(Clone, Debug)]
pub struct InvokeDynamic {
    /**
     * **bootstrap_method_attr_index**\
     *  The value of the bootstrap_method_attr_index item must be a valid index
     *  into the bootstrap_methods array of the bootstrap method table of this class
     *  file (§4.7.23).
     */
    pub bootstrap_method_attr_index: u16,
    /**
     * **name_and_type_index**\
     *  The value of the name_and_type_index item must be a valid index into
     *  the constant_pool table. The constant_pool entry at that index must be a
     *  CONSTANT_NameAndType_info structure (§4.4.6). This constant_pool entry
     *  indicates a name and descriptor.
     *
     *  In a CONSTANT_InvokeDynamic_info structure, the indicated descriptor must
     *  be a method descriptor (§4.3.3).
     */
    pub name_and_type_index: u16,
}

impl InvokeDynamic {
    pub fn new(bootstrap_method_attr_index: u16, name_and_type_index: u16) -> InvokeDynamic {
        InvokeDynamic {
            bootstrap_method_attr_index,
            name_and_type_index,
        }
    }
}

#[derive(Clone, Debug)]
/// [Module Constant](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A2423%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C117.8%2Cnull%5D)
pub struct Module {
    /**
     * **name_index**\
     *  The value of the name_index item must be a valid index into the
     *  constant_pool table. The constant_pool entry at that index must be a
     *  CONSTANT_Utf8_info structure (§4.4.7) representing a valid module name
     *  (§4.2.3).
     *
     *  A CONSTANT_Module_info structure is permitted only in the constant pool of
     *  a class file that declares a module, that is, a ClassFile structure where the
     *  access_flags item has the ACC_MODULE flag set. In all other class files, a
     *  CONSTANT_Module_info structure is illegal.
     */
    pub name_index: u16,
}

impl Module {
    pub fn new(name_index: u16) -> Module { Module { name_index } }
}

#[derive(Clone, Debug)]
/// [Package Constant](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A676%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C348.6%2Cnull%5D)
pub struct Package {
    /**
     * **name_index**\
     *  The value of the name_index item must be a valid index into the
     *  constant_pool table. The constant_pool entry at that index must be a
     *  CONSTANT_Utf8_info structure (§4.4.7) representing a valid package name
     *  encoded in internal form (§4.2.3).
     *
     *  A CONSTANT_Package_info structure is permitted only in the constant pool of
     *  a class file that declares a module, that is, a ClassFile structure where the
     *  access_flags item has the ACC_MODULE flag set. In all other class files, a
     *  CONSTANT_Package_info structure is illegal.
     */
    pub name_index: u16,
}

impl Package {
    pub fn new(name_index: u16) -> Package { Package { name_index } }
}

pub fn read_constant_pool(
    pool: &mut Vec<ConstantPool>,
    cursor: &mut Cursor<&[u8]>,
) -> Result<(), Box<dyn Error>> {
    for _ in 0..pool.capacity() {
        let tag = cursor.read_u8()?;
        pool.push(match Tags::from(tag) {
            Tags::Utf8 => ConstantPool::Utf8(Utf8::new(cursor)),
            Tags::String => ConstantPool::String(String::new(cursor.read_u16::<BE>()?)),
            Tags::Integer => ConstantPool::Integer(Integer::new(cursor.read_u32::<BE>()?)),
            Tags::Float => ConstantPool::Float(Float::new(cursor.read_u32::<BE>()?)),
            Tags::Long => ConstantPool::Long(Long::new(
                cursor.read_u32::<BE>()?,
                cursor.read_u32::<BE>()?,
            )),
            Tags::Double => ConstantPool::Double(Double::new(
                cursor.read_u32::<BE>()?,
                cursor.read_u32::<BE>()?,
            )),
            Tags::Class => ConstantPool::Class(Class::new(cursor.read_u16::<BE>()?)),
            Tags::Fieldref => ConstantPool::Fieldref(Fieldref::new(
                cursor.read_u16::<BE>()?,
                cursor.read_u16::<BE>()?,
            )),
            Tags::Methodref => ConstantPool::Methodref(Methodref::new(
                cursor.read_u16::<BE>()?,
                cursor.read_u16::<BE>()?,
            )),
            Tags::InterfaceMethodref => ConstantPool::InterfaceMethodref(InterfaceMethodref::new(
                cursor.read_u16::<BE>()?,
                cursor.read_u16::<BE>()?,
            )),
            Tags::NameAndType => ConstantPool::NameAndType(NameAndType::new(
                cursor.read_u16::<BE>()?,
                cursor.read_u16::<BE>()?,
            )),
            Tags::MethodHandle => ConstantPool::MethodHandle(MethodHandle::new(
                cursor.read_u8()?,
                cursor.read_u16::<BE>()?,
            )),
            Tags::MethodType => ConstantPool::MethodType(MethodType::new(cursor.read_u16::<BE>()?)),
            Tags::Dynamic => ConstantPool::Dynamic(Dynamic::new(
                cursor.read_u16::<BE>()?,
                cursor.read_u16::<BE>()?,
            )),
            Tags::InvokeDynamic => ConstantPool::InvokeDynamic(InvokeDynamic::new(
                cursor.read_u16::<BE>()?,
                cursor.read_u16::<BE>()?,
            )),
            Tags::Module => ConstantPool::Module(Module::new(cursor.read_u16::<BE>()?)),
            Tags::Package => ConstantPool::Package(Package::new(cursor.read_u16::<BE>()?)),
            _ => {
                return Err(Box::new(LoadingError::new(
                    LoadingCause::InvalidConstantTag(tag),
                    &format!("Cursor Position: {:#04X?}", cursor.position() - 1),
                )))
            }
        });
    }
    Ok(())
}