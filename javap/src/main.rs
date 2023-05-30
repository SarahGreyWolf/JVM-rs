use byteorder::ReadBytesExt;
use std::{
    fs::File,
    io::{Cursor, Read, Write},
    path::PathBuf,
};

use clap::Parser;
use jloader::{
    access_flags::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags},
    attributes::AttributeInfo,
    class_file::{self, MethodInfo},
    constants::ConstantPool,
    descriptors::FieldDescriptor,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[arg(value_name = "CLASSES", required = true)]
    class_file: Vec<PathBuf>,

    /// Print line number and local variable tables
    #[arg(short, long)]
    line: bool,

    /// Show only public classes and members
    #[arg(long)]
    public: bool,

    /// Show protected/public classes and members
    #[arg(long)]
    protected: bool,

    /// Show package/protected/public classes and members (default)
    #[arg(long, default_value_t = true)]
    package: bool,

    /// Show all classes and members
    #[arg(long)]
    private: bool,

    /// Disassemble the code
    #[arg(short = 'c', long)]
    disassemble: bool,

    /// Print internal type signatures
    #[arg(short, long)]
    signatures: bool,

    /// Show system info (path, size, date, MD5 hash) of class being processed
    #[arg(long)]
    sysinfo: bool,

    /// Show final constants
    #[arg(long)]
    constants: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let file_path = &args.class_file[0];

    if let Some(ext) = file_path.extension() {
        if ext != "class" {
            panic!("File provided was not a java class file");
        }
        let mut class_file: File = File::open(file_path).expect("Failed to open file");
        let mut contents = vec![00; class_file.metadata().unwrap().len() as usize];
        class_file
            .read_exact(&mut contents)
            .expect("Failed to read bytes");
        let class = class_file::Class::from_bytes(&contents)?;
        if !args.line || !args.signatures || !args.sysinfo {
            let class_output = output_class(class, &args)?;
            let mut stdout = std::io::stdout();
            stdout.write_all(&class_output)?;
        }
    } else {
        panic!("File provided did not have an extension.");
    }

    Ok(())
}

fn output_class(
    class: class_file::Class,
    args: &Args,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // FIXME: Bundle this up into a buffer to then push to stdout
    let mut output_buffer = vec![];

    const SPACING: &str = "    ";
    for attributes in &class.attributes {
        if let AttributeInfo::SourceFile(sf) = attributes {
            if let ConstantPool::Utf8(title) = &class.constant_pool[sf.sourcefile_index as usize] {
                writeln!(output_buffer, "Compiled from \"{}\"", String::from(title))?;
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
            if *flag != ClassAccessFlags::AccSuper {
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
    writeln!(output_buffer, "{class_def}")?;
    for field in &class.fields {
        let mut is_secret_private = false;
        if !field.access_flags.contains(&FieldAccessFlags::AccPublic)
            && !field.access_flags.contains(&FieldAccessFlags::AccProtected)
            && !field.access_flags.contains(&FieldAccessFlags::AccPrivate)
        {
            is_secret_private = true;
        }
        if args.public && (!args.protected && !args.private) {
            if field.access_flags.contains(&FieldAccessFlags::AccProtected)
                || field.access_flags.contains(&FieldAccessFlags::AccPrivate)
                || is_secret_private
            {
                continue;
            }
        }
        if (args.protected || args.package) && !args.private {
            if field.access_flags.contains(&FieldAccessFlags::AccPrivate) {
                continue;
            }
        }

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
            let type_descriptors = field.get_type(&class.constant_pool);
            let mut _type = String::new();
            for t in type_descriptors.iter() {
                if *t != FieldDescriptor::ArrayType {
                    _type = String::from(t.clone());
                } else {
                    continue;
                }
            }
            let field_def = if !args.constants {
                if type_descriptors[0] == FieldDescriptor::ArrayType {
                    format!("{access_flags} {_type}[] {field_name};")
                } else {
                    format!("{access_flags} {_type} {field_name};")
                }
            } else {
                if let AttributeInfo::ConstantValue(c) = attrib {
                    match class.constant_pool[c.constantvalue_index as usize] {
                        ConstantPool::Utf8(_) => todo!(),
                        ConstantPool::Integer(_) => todo!(),
                        ConstantPool::Float(_) => todo!(),
                        ConstantPool::Long(_) => todo!(),
                        ConstantPool::Double(_) => todo!(),
                        ConstantPool::Class(_) => todo!(),
                        ConstantPool::String(ref s) => {
                            if let ConstantPool::Utf8(ref s) =
                                class.constant_pool[s.string_index as usize]
                            {
                                if type_descriptors[0] == FieldDescriptor::ArrayType {
                                    format!(
                                        "{access_flags} {_type}[] {field_name} = \"{}\";",
                                        String::from(s)
                                    )
                                } else {
                                    format!(
                                        "{access_flags} {_type} {field_name} = \"{}\";",
                                        String::from(s)
                                    )
                                }
                            } else {
                                unreachable!(
                                    "ConstantPool String index {} did not point to a utf8",
                                    s.string_index
                                );
                            }
                        }
                        ConstantPool::Fieldref(_) => todo!(),
                        ConstantPool::Methodref(_) => todo!(),
                        ConstantPool::InterfaceMethodref(_) => todo!(),
                        ConstantPool::NameAndType(_) => todo!(),
                        ConstantPool::MethodHandle(_) => todo!(),
                        ConstantPool::MethodType(_) => todo!(),
                        ConstantPool::Dynamic(_) => todo!(),
                        ConstantPool::InvokeDynamic(_) => todo!(),
                        ConstantPool::Module(_) => todo!(),
                        ConstantPool::Package(_) => todo!(),
                        ConstantPool::Unknown => todo!(),
                    }
                } else {
                    if type_descriptors[0] == FieldDescriptor::ArrayType {
                        format!("{access_flags} {_type}[] {field_name};")
                    } else {
                        format!("{access_flags} {_type} {field_name};")
                    }
                }
            };
            writeln!(output_buffer, "{SPACING}{field_def}")?;
        }
    }
    for method in &class.methods {
        let mut is_secret_private = false;
        if !method.access_flags.contains(&MethodAccessFlags::AccPublic)
            && !method
                .access_flags
                .contains(&MethodAccessFlags::AccProtected)
            && !method.access_flags.contains(&MethodAccessFlags::AccPrivate)
        {
            is_secret_private = true;
        }
        if args.public && (!args.protected && !args.private) {
            if method
                .access_flags
                .contains(&MethodAccessFlags::AccProtected)
                || method.access_flags.contains(&MethodAccessFlags::AccPrivate)
                || is_secret_private
            {
                continue;
            }
        }
        if (args.protected || args.package) && !args.private {
            if method.access_flags.contains(&MethodAccessFlags::AccPrivate) {
                continue;
            }
        }
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
            writeln!(output_buffer, "{SPACING}{access_flags} {{}};")?;
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
            writeln!(output_buffer, "{SPACING}{method_def}")?;
        }
    }
    writeln!(output_buffer, "}}")?;
    Ok(output_buffer)
}
