use std::{
    error::Error,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use jloader::{
    class_file::{Class, ClassLoc},
    constants::PoolConstants,
};

use crate::{
    errors::exceptions::{Exception, ExceptionError},
    runtime_pool::RuntimeConstant,
    vm,
};

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
        if file_name == split.clone().next_back().unwrap() {
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
        dbg!(path);
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
            heap[vm::METHOD_SPACE..vm::METHOD_SPACE + contents.len()]
                .copy_from_slice(&contents);
            method_area.push(ClassLoc::new(
                class_name,
                vm::METHOD_SPACE..vm::METHOD_SPACE + contents.len(),
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

// https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A2326%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C545%2Cnull%5D
pub fn link_class(
    class: &Class,
    heap_ref: &mut Vec<u8>,
    method_area_ref: &mut Vec<ClassLoc>,
) {
    // Verification is already done by jloader

    // Throws: LinkageError

    // Preparation

    /*
     * 1. Let L1 be the defining loader of C. For each instance method m declared in
     *    C that can override (§5.4.5) an instance method declared in a superclass or
     *    superinterface <D, L2>, the Java Virtual Machine imposes loading constraints
     *    as follows.
     *    Given that the return type of m is Tr, and that the formal parameter types of m
     *    are Tf1, ..., Tfn:
     *      m = int test(int a, int b)
     *      Tr = int
     *      Tf1 = a, Tf2 = b
     *    If Tr not an array type, let T0 be Tr; otherwise, let T0 be the element type of Tr.
     *    For i = 1 to n: If Tfi is not an array type, let Ti be Tfi; otherwise, let Ti be the
     *    element type of Tfi.
     *    Then TiL1 = TiL2 for i = 0 to n.
     */

    // Resolution

    // Access Control

    // Method Overriding

    // Method Selection
}

pub struct VmClass {
    runtime_pool: Vec<RuntimeConstant>,
    constant_pool: Vec<PoolConstants>,
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
