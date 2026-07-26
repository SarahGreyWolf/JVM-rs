//FIXME: Remove This
#![feature(path_file_prefix)]
#![allow(unused)]

use std::{
    env::args,
    fmt,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use jloader::access_flags::{self, MethodAccessFlags};
use jloader::attributes::AttributeInfo;
use jloader::class_file;
use jloader::constants::PoolConstants;
use vm::VM;

use crate::errors::exceptions::Exception;

mod class;
mod class_loader;
/// [Data Types](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A62%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C590%2Cnull%5D)
mod data_types;
mod errors;
mod ops;

/// [Runtime Pool](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A2809%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C210.7%2Cnull%5D)
mod runtime_pool;
/// [Frames](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A802%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C590%2Cnull%5D)
mod stack_frame;
mod util;
/// [JVM Spec](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf)
mod vm;
// FIXME: Remove Later
mod temp_run;

fn main() -> Result<(), Exception> {
    let mut args = args();
    args.next().unwrap();
    if args.len() == 0 {
        panic!(
            "You must provide the classpath and the name of a java classfile"
        );
    }
    let class_path = PathBuf::from(args.next().unwrap());
    let class = args.next().unwrap();

    let mut jvm = VM::new(None);
    jvm.create_from_path(class_path.clone(), &class)?;
    jvm.run()?;

    // let mut method_area: Vec<class_file::ClassLoc> = vec![];

    /*
    if let Some(ext) = file_path.extension() {
        if ext != "class" {
            panic!("File provided was not a java class file");
        }
        let mut class_file: File = File::open(file_path).expect("Failed to open file");
        let mut contents = vec![00; class_file.metadata().unwrap().len() as usize];
        class_file
            .read_exact(&mut contents)
            .expect("Failed to read bytes");
        let class = class_file::ClassFile::from_bytes(&contents)?;
        println!("{}", class.to_pretty_fmt());
        // javap(class);
        // let mut jvm = temp_run::BasicAssVM::new(class.constant_pool.clone());
        // jvm.run(class)?;
    } else {
        panic!("File provided did not have an extension.");
    }
    */
    Ok(())
}
