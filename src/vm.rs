use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::Mutex;
use std::{error::Error, io::Read};

use jloader::attributes::AttributeInfo;
use jloader::class_file::ClassLoc;
use jloader::{class_file::Class, constants::PoolConstants};

use crate::errors::exceptions::{Exception, ExceptionError};
// use crate::errors::exceptions::{Exception, ExceptionError};
use crate::ops::mnemonics::Mnemonic;
use crate::ops::Instruction;
use crate::runtime_pool::{self, RuntimeConstant, SymbolicRef};
use crate::stack_frame::StackFrame;

// Where in the heap that method space sits
static METHOD_SPACE: usize = 1024 * 1024 * 5;

// https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A802%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C165%2Cnull%5D
// https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A62%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C286%2Cnull%5D
#[derive(Clone, Debug, PartialEq)]
pub enum FrameValues {
    Boolean(bool),
    Byte(i8),
    Char(u8),
    Short(i16),
    Int(i32),
    Float(f32),
    Reference(SymbolicRef),
    ReturnAddress(u64),
    Long(i64),
    Double(f64),
    NaN,
}

// https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A2220%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C487%2Cnull%5D
#[derive(Clone)]
struct NativeStack {}

#[derive(Clone)]
pub struct Thread {
    // Stack
    // Can be variable length with min & max or can be fixed
    pub frames: Vec<StackFrame>,
    pub active_frame: usize,
    pub native_stack: Vec<NativeStack>,
    // Reference to the VM Heap
    pub heap_ref: Arc<Mutex<Vec<u8>>>,
    pub method_area_ref: Arc<Mutex<Vec<ClassLoc>>>,
}

pub struct VM {
    pub threads: Vec<Thread>,
    // https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A38%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C345%2Cnull%5D
    pub heap: Arc<Mutex<Vec<u8>>>,
    // https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A2226%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C551%2Cnull%5D
    // This is a reference into the heap that stores the Class
    // This might need some kind of ID for identifying the class maybe?
    // TODO: Handle garbage collecting this
    //       Kinda thinking something like a time when the class was last accessed or something
    pub method_area: Arc<Mutex<Vec<ClassLoc>>>,
    pub class_path: Option<PathBuf>,
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
        let settings = settings.unwrap_or_default();
        VM {
            threads: vec![],
            heap: Arc::new(Mutex::new(vec![0u8; settings.heap_max])),
            method_area: Arc::new(Mutex::new(vec![])),
            class_path: None,
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let thread = self.threads[0].clone();
        let mut first_frame = thread.frames[0].clone();
        let Some(class_path) = self.class_path.clone() else {
            panic!("Class Path was None!");
        };
        first_frame.run(
            self,
            &class_path,
            thread.heap_ref.clone(),
            thread.method_area_ref.clone(),
            None,
        )?;
        Ok(())
    }

    pub fn create_from_path(
        &mut self,
        path: PathBuf,
    ) -> Result<(), Box<dyn Error>> {
        let heap_clone = self.heap.clone();
        let mut heap = match heap_clone.lock() {
            Ok(heap) => heap,
            Err(e) => panic!("Failed to get heap lock: {e}"),
        };
        let method_area_clone = self.method_area.clone();
        let mut method_area = match method_area_clone.lock() {
            Ok(method_area) => method_area,
            Err(e) => panic!("Failed to get method_area lock: {e}"),
        };
        let class = load_class(&mut heap, &mut method_area, &path)?;

        let pool = runtime_pool::RuntimeConstant::from_constant_pool(
            &class.constant_pool,
        );
        dbg!(&pool);

        let mut class_path = path.clone();
        class_path.pop();
        self.class_path = Some(class_path);

        let mut thread = Thread {
            frames: vec![],
            active_frame: 0,
            native_stack: vec![],
            heap_ref: self.heap.clone(),
            method_area_ref: self.method_area.clone(),
        };

        let mut frame = StackFrame {
            pc: Some(0),
            code: vec![],
            locals: vec![],
            stack: vec![],
            pool,
            const_pool: class.constant_pool.clone(),
            thread_id: self.threads.len(),
        };

        for method in &class.methods {
            if let PoolConstants::Utf8(name) =
                class.get_from_constant_pool(method.name_index)?
            {
                if String::from(name) != "main" {
                    continue;
                }
                for attr in &method.attributes {
                    if let AttributeInfo::Code(code) = attr {
                        frame.code = code.code.clone();
                        // Fill frame.locals with null references
                        frame.locals =
                            vec![
                                FrameValues::Reference(SymbolicRef::Null);
                                code.max_locals as usize
                            ];
                        frame.stack =
                            Vec::with_capacity(code.max_stack as usize);
                    }
                }
            }
        }

        thread.frames.push(frame);
        self.threads.push(thread);

        Ok(())
    }
}

pub fn find_class(
    class_path: &Path,
    name: &str,
    sub_directory: Option<&Path>,
) -> Result<PathBuf, Box<dyn Error>> {
    let split = name.split('/');

    let dir = if let Some(sub) = sub_directory {
        sub.read_dir()?
    } else {
        class_path.read_dir()?
    };

    let mut directories = vec![];

    for entry in dir {
        let Ok(entry) = entry else { continue };
        let path = entry.path();
        if path.is_dir() {
            directories.push(entry.path());
        }
        let Some(ext) = path.extension() else {
            continue;
        };
        if ext != "class" {
            continue;
        }
        let file_name = path.file_prefix().unwrap();
        let Some(file_name) = file_name.to_str() else {
            panic!(
                "Filename for {:?} could not be converted to str",
                entry.file_name()
            );
        };
        if file_name == split.clone().last().unwrap() {
            return Ok(entry.path());
        }
    }

    if directories.is_empty() {
        return Err(Box::new(ExceptionError::new(
            Exception::ClassNotFound,
            "Class could not be found",
        )));
    }

    // FIXME: This should really check if any of the sub directories matches the
    //        first part of the class description.
    //        E.G: jloader then basic_main_java_test
    for (index, dir) in directories.iter().enumerate() {
        match find_class(class_path, name, Some(dir)) {
            Ok(r) => return Ok(r),
            Err(e) if index != directories.len() => continue,
            Err(e) => return Err(e),
        }
    }

    unreachable!("This should probably have an actual error, just like a lot of things in the VM")
}

pub fn load_class(
    heap: &mut Vec<u8>,
    method_area: &mut Vec<ClassLoc>,
    path: &Path,
) -> Result<Class, Box<dyn Error>> {
    if let Some(ext) = path.extension() {
        if ext != "class" {
            // FIXME: Handle all panics (get rid of them for proper errors)
            panic!("Provided file was not a class");
        }
        let mut class_file: File =
            File::open(path).expect("Failed to open file");
        let Some(metadata) = class_file.metadata().ok() else {
            panic!("Could not get metadata for class file");
        };
        let mut contents = vec![00; metadata.len() as usize];
        class_file.read_exact(&mut contents)?;
        let class = Class::from_bytes(&contents)?;
        let class_name = class.get_class_name()?;
        if method_area.is_empty() {
            heap[METHOD_SPACE..METHOD_SPACE + contents.len()]
                .copy_from_slice(&contents);
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
            heap[end_of_currents..end_of_currents + contents.len()]
                .copy_from_slice(&contents);
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

pub fn load_class_from_heap(
    heap: &[u8],
    loc: &ClassLoc,
) -> Result<Class, Box<dyn Error>> {
    if heap.len() < loc.1.end {
        panic!("Heap OOB");
    }
    let range = loc.1.clone();
    let class = Class::from_bytes(&heap[range])?;

    if class.get_class_name()? != loc.0 {
        panic!(
            "Heap Corruption: {} was not the correct class for {}",
            class.get_class_name()?,
            loc.0
        );
    }

    Ok(class)
}

// https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A2326%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C545%2Cnull%5D
pub fn link_class(
    class: &Class,
    heap_ref: &mut Vec<u8>,
    method_area_ref: &mut Vec<ClassLoc>,
) {
    // Verification is already done by jloader
    // Throws: LinkageError

    // Preparation

    // Resolution

    // Access Control

    // Method Overriding

    // Method Selection
}

pub struct VmClass {
    runtime_pool: Vec<RuntimeConstant>,
}
