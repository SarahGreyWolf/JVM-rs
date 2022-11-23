//FIXME: This isn't ideal
#![feature(cursor_remaining)]
use std::{
    env::args,
    fmt,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use crate::access_flags::MethodAccessFlags;

mod access_flags;
/// [Attributes](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A1244%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C564%2Cnull%5D)
mod attributes;
/// [Class File Format](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A376%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C590%2Cnull%5D)
mod class_file;
/// [Constants](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A2201%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C256%2Cnull%5D)
mod constants;
/// [Data Types](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A62%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C590%2Cnull%5D)
mod data_types;
/// [Descriptors](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A677%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C448%2Cnull%5D)
mod descriptors;
mod errors;


/// [JVM Spec](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf)
struct VirtualMachine {}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = args();
    args.next().unwrap();
    if args.len() != 1 {
        panic!("You must provide the path to a java classfile");
    }
    let file_path = PathBuf::from(args.next().unwrap());
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
    } else {
        panic!("File provided did not have an extension.");
    }
    Ok(())

}
