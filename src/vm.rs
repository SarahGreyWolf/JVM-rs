use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::Mutex;
use std::{error::Error, io::Read};

use jloader::attributes::AttributeInfo;
use jloader::class_file::ClassLoc;
use jloader::{class_file::Class, constants::ConstantPool};

use crate::ops::mnemonics::Mnemonic;
use crate::ops::Instruction;

// Where in the heap that method space sits
static METHOD_SPACE: usize = 1024 * 1024 * 5;

// https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A802%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C165%2Cnull%5D
#[derive(Clone, Copy, Debug)]
pub enum FrameValues {
    Boolean(bool),
    Byte(i8),
    Char(u8),
    Short(i16),
    Int(i32),
    Float(f32),
    Reference(u64),
    ReturnAddress(u64),
    Long(i64),
    Double(f64),
}

// https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A45%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C250%2Cnull%5D
#[derive(Debug)]
pub struct StackFrame {
    pub pc: Option<u64>,
    pub code: Vec<u8>,
    // https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A802%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C165%2Cnull%5D
    pub locals: Vec<FrameValues>,
    // https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A814%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C267%2Cnull%5D
    pub stack: Vec<FrameValues>,
    // https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A4314%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C325%2Cnull%5D
    pub pool: Vec<ConstantPool>,
}
// https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A2220%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C487%2Cnull%5D
struct NativeStack {}
pub struct Thread {
    // Stack
    // Can be variable length with min & max or can be fixed
    pub frames: Vec<StackFrame>,
    active_frame: usize,
    native_stack: Vec<NativeStack>,
    // Reference to the VM Heap
    heap_ref: Arc<Mutex<Vec<u8>>>,
    method_area_ref: Arc<Mutex<Vec<ClassLoc>>>,
    runtime_pool: Vec<ConstantPool>,
}

pub struct VM {
    pub threads: Vec<Thread>,
    // https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A38%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C345%2Cnull%5D
    heap: Arc<Mutex<Vec<u8>>>,
    // https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A2226%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C551%2Cnull%5D
    // This is a reference into the heap that stores the Class
    // This might need some kind of ID for identifying the class maybe?
    // TODO: Handle garbage collecting this
    //       Kinda thinking something like a time when the class was last accessed or something
    method_area: Arc<Mutex<Vec<ClassLoc>>>,
}
pub struct VMSettings {
    heap_max: usize,
    heap_min: usize,
    stack_max: usize,
    stack_min: usize,
}
impl Default for VMSettings {
    fn default() -> Self {
        Self {
            heap_max: 1024 * 1024 * 10,
            heap_min: 1024 * 1024,
            stack_max: 1024 * 1024 * 10,
            stack_min: 1024 * 1024,
        }
    }
}
