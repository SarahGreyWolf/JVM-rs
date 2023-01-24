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
enum References {
    Class,
    Array(Box<dyn Component>),
    Interface,
    Null,
}

impl Component for References {}

struct Class {}
