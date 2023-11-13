use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::Mutex;
use std::{error::Error, io::Read};

use jloader::attributes::AttributeInfo;
use jloader::class_file::ClassLoc;
use jloader::{class_file::Class, constants::PoolConstants};

use crate::ops::mnemonics::Mnemonic;
use crate::ops::Instruction;
use crate::stack_frame::StackFrame;

// Where in the heap that method space sits
static METHOD_SPACE: usize = 1024 * 1024 * 5;

// https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A802%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C165%2Cnull%5D
// https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A62%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C286%2Cnull%5D
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
impl VM {
    pub fn new(settings: Option<VMSettings>) -> VM {
        let settings = if let Some(settings) = settings {
            settings
        } else {
            VMSettings::default()
        };
        VM {
            threads: vec![],
            heap: Arc::new(Mutex::new(vec![0u8; settings.heap_max])),
            method_area: Arc::new(Mutex::new(vec![])),
        }
    }
}
fn load_class(
    heap: &mut Vec<u8>,
    method_area: &mut Vec<ClassLoc>,
    path: &Path,
) -> Result<Class, Box<dyn Error>> {
    if let Some(ext) = path.extension() {
        if ext != "class" {
            // FIXME: Handle all panics (get rid of them for proper errors)
            panic!("Provided file was not a class");
        }
        let mut class_file: File = File::open(path).expect("Failed to open file");
        let Some(metadata) = class_file.metadata().ok() else {
            panic!("Could not get metadata for class file");
        };
        let mut contents = vec![00; metadata.len() as usize];
        class_file.read_exact(&mut contents)?;
        let class = Class::from_bytes(&contents)?;
        let class_name = class.get_class_name()?;
        if method_area.is_empty() {
            heap[METHOD_SPACE..METHOD_SPACE + contents.len()].copy_from_slice(&contents);
            method_area.push(ClassLoc::new(
                class_name,
                METHOD_SPACE..METHOD_SPACE + contents.len(),
            ));
        } else {
            let mut end_of_currents: usize = 0;
            for ClassLoc(_, range) in method_area.iter() {
                if range.end > end_of_currents {
                    end_of_currents = range.end;
                }
            }
            if end_of_currents > heap.capacity()
                || end_of_currents + contents.len() > heap.capacity()
            {
                // FIXME: This should throw an `OutOfMemoryError` in the VM
                panic!("OUT OF MEMORY ERROR: Reached Heap Capacity");
            }
            heap[end_of_currents..end_of_currents + contents.len()].copy_from_slice(&contents);
            method_area.push(ClassLoc::new(
                class_name,
                end_of_currents..end_of_currents + contents.len(),
            ));
        }
        Ok(class)
    } else {
        panic!("Provided path was not a file!");
    }
}
