//FIXME: This isn't ideal
#![feature(cursor_remaining)]
//FIXME: Remove This
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
use jloader::constants::ConstantPool;

/// [Data Types](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A62%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C590%2Cnull%5D)
mod data_types;

mod instructions;

// FIXME: Remove Later
mod temp_run;

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
        // javap(class);
        // let mut jvm = temp_run::BasicAssVM::new(class.constant_pool.clone());
        // jvm.run(class)?;
    } else {
        panic!("File provided did not have an extension.");
    }
    Ok(())
}

fn javap(class: class_file::ClassFile) {
    const SPACING: &str = "    ";
    for attributes in class.attributes {
        if let AttributeInfo::SourceFile(sf) = attributes {
            if let ConstantPool::Utf8(title) = &class.constant_pool[sf.sourcefile_index as usize] {
                println!("Compiled from \"{}\"", String::from(title));
            }
        }
    }
    let class_name = if let ConstantPool::Class(c) = &class.constant_pool[class.this_class as usize]
    {
        if let ConstantPool::Utf8(cn) = &class.constant_pool[c.name_index as usize] {
            String::from(cn)
        } else {
            unreachable!("Could not get class name from index {}", c.name_index);
        }
    } else {
        unreachable!("Could not get class from index {}", class.this_class);
    };
    let access_flags: String = class
        .access_flags
        .iter()
        .map(|flag| {
            if *flag != access_flags::ClassAccessFlags::AccSuper {
                String::from(flag)
            } else {
                "".into()
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
        .trim()
        .to_string();
    let mut class_def = format!("{access_flags} class {class_name} {{");
    class_def = class_def.trim().to_string();
    println!("{class_def}");
    for field in class.fields {
        for attrib in field.clone().attributes {
            let access_flags: String = field
                .access_flags
                .iter()
                .map(String::from)
                .collect::<Vec<String>>()
                .join(" ")
                .trim()
                .to_string();
            let field_name = if let ConstantPool::Utf8(field_name) =
                &class.constant_pool[field.name_index as usize]
            {
                String::from(field_name)
            } else {
                unreachable!("Could not get field name from index {}", field.name_index);
            };
            let mut _type = field.get_type(&class.constant_pool);
            _type = _type.trim_start_matches('[').to_string();
            _type = _type.trim_start_matches('L').to_string();
            let field_def = format!("{access_flags} {_type} {field_name};");
            println!("{SPACING}{field_def}");
            // if let AttributeInfo::ConstantValue(v) = attrib {

            // } else {continue;}
        }
    }
    for method in class.methods {
        let method_name =
            if let ConstantPool::Utf8(name) = &class.constant_pool[method.name_index as usize] {
                let mut name = String::from(name);
                if name == "<init>" {
                    name = class_name.clone();
                }
                name
            } else {
                unreachable!("Could not get method name from index {}", method.name_index);
            };
        let access_flags: String = method
            .access_flags
            .iter()
            .map(|flag| {
                if *flag == MethodAccessFlags::AccVarArgs
                    || *flag == MethodAccessFlags::AccSynthetic
                {
                    " ".into()
                } else {
                    flag.into()
                }
            })
            .collect::<Vec<String>>()
            .join(" ")
            .trim()
            .to_string();
        if method_name == "<clinit>" {
            println!("{SPACING}{access_flags} {{}};");
        } else {
            let params = method
                .get_params(&class.constant_pool)
                .iter()
                .filter(|param| !param.is_empty())
                .cloned()
                .collect::<Vec<String>>()
                .join(", ");
            let return_type = method.get_return(&class.constant_pool);
            let mut method_def = if method_name == class_name {
                format!(
                    "{access_flags} {method_name}({params});",
                    params = params.trim_matches(',').trim_start_matches('L')
                )
            } else {
                format!(
                    "{access_flags} {return_type} {method_name}({params});",
                    params = params.trim_matches(',').trim_start_matches('L')
                )
            };
            method_def = method_def.trim().to_string();
            println!("{SPACING}{method_def}");
        }
    }
    println!("}}");
}
