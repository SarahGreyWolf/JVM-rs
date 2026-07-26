use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::Mutex;
use std::{error::Error, io::Read};

use jloader::attributes::AttributeInfo;
use jloader::class_file::ClassLoc;
use jloader::{class_file::Class, constants::PoolConstants};

use crate::class::load_class;
use crate::class_loader;
use crate::data_types::SymbolicRef;
use crate::errors::exceptions::{Exception, ExceptionError};
// use crate::errors::exceptions::{Exception, ExceptionError};
use crate::ops::mnemonics::Mnemonic;
use crate::ops::Instruction;
use crate::runtime_pool::{self, RuntimeConstant};
use crate::stack_frame::StackFrame;

// Where in the heap that method space sits
pub const METHOD_SPACE: usize = 1024 * 1024 * 5;

// https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A802%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C165%2Cnull%5D
// https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A62%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C286%2Cnull%5D
#[derive(Clone, Debug)]
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
    pub class_loader: Option<Box<dyn class_loader::ClassLoader>>,
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
            // FIXME: This should do more shit, like VMSettings things
            class_loader: None,
        }
    }

    pub fn run(&mut self) -> Result<(), Exception> {
        // FIXME: Is this the correct way to do this?
        //        Maybe we should be doing this as a loop that
        //        acts on the current active thread and it's current active frame?

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
        class_path: PathBuf,
        class_name: &str,
    ) -> Result<(), Exception> {
        // let class = load_class(&mut heap, &mut method_area, &path)
        //     .map_err(Exception::Other)?;
        self.class_path = Some(class_path.clone());
        self.class_loader = Some(Box::new(
            class_loader::BootstrapClassloader::new(class_path),
        ));

        let Some(ref mut class_loader) = self.class_loader else {
            return Err(Exception::ClassNotFound);
        };

        let class_loc = class_loader.load_class(
            class_name.to_string(),
            self.method_area.clone(),
            self.heap.clone(),
        )?;

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

        let class = crate::class::load_class_from_heap(&heap, &class_loc)
            .map_err(Exception::Other)?;

        let pool = runtime_pool::RuntimeConstant::from_constant_pool(
            &class.constant_pool,
        );
        dbg!(&pool);

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
            if let PoolConstants::Utf8(name) = class
                .get_from_constant_pool(method.name_index)
                .map_err(Exception::JLoader)?
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
