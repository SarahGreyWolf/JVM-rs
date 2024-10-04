use jloader::descriptors::FieldDescriptor;

use crate::runtime_pool::RuntimeConstant;

trait Component {}

enum Primitives {
    // Numeric Types
    //     Integral Types
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    // Why is this a numeric type Java...
    Char(u16),

    //     Floating Point Types
    Float(f32),
    Double(f64),

    // Boolean
    Boolean(bool),

    /// [returnAddress](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A4269%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C135%2Cnull%5D)
    ///
    /// The address of the opcode of the instruction following the instruction that creates it
    ReturnAddress(u16),
}

impl Component for Primitives {}

/// [Reference Types and Values](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A4175%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C295%2Cnull%5D)
///
/// Their values are references to dynamically created class instances, arrays, or class instances or arrays that implement interfaces, respectively.
#[derive(Clone, Debug, PartialEq)]
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
