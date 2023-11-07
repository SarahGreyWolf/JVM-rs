use byteorder::ReadBytesExt;
use jvm_rs::ops::{mnemonics::Mnemonic, Instruction, OperandType};
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
    let mut output_buffer = vec![];

    for attributes in &class.attributes {
        if let AttributeInfo::SourceFile(sf) = attributes {
            if let ConstantPool::Utf8(title) = &class.constant_pool[sf.sourcefile_index as usize] {
                writeln!(output_buffer, "Compiled from \"{}\"", String::from(title))?;
            }
        }
    }
    let this_class_name =
        if let ConstantPool::Class(c) = &class.constant_pool[class.this_class as usize] {
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
    let mut class_def = format!("{access_flags} class {this_class_name} {{");
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
                || (is_secret_private && !args.package)
            {
                continue;
            }
        }
        if (args.protected || args.package) && !args.private {
            if field.access_flags.contains(&FieldAccessFlags::AccPrivate) {
                continue;
            }
        }
        let field_name = if let ConstantPool::Utf8(field_name) =
            &class.constant_pool[field.name_index as usize]
        {
            String::from(field_name)
        } else {
            unreachable!("Could not get field name from index {}", field.name_index);
        };
        let access_flags: String = field
            .access_flags
            .iter()
            .map(String::from)
            .collect::<Vec<String>>()
            .join(" ")
            .trim()
            .to_string();
        let type_descriptors = field.get_type(&class.constant_pool);
        let mut _type = String::new();
        for t in type_descriptors.iter() {
            if let FieldDescriptor::ArrayType(_) = *t {
                continue;
            }
            _type = String::from(t.clone());
        }
        if field.attributes_count == 0 || !args.constants {
            let field_def = if let FieldDescriptor::ArrayType(ref name) = type_descriptors[0] {
                format!("{access_flags} {name} {field_name};")
            } else {
                format!("{access_flags} {_type} {field_name};")
            };
            writeln!(output_buffer, "\t{field_def}")?;
            continue;
        }
        for attrib in field.clone().attributes {
            let field_def = if let AttributeInfo::ConstantValue(c) = attrib {
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
                            if let FieldDescriptor::ArrayType(ref name) = type_descriptors[0] {
                                format!(
                                    "{access_flags} {name} {field_name} = \"{}\";",
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
                if let FieldDescriptor::ArrayType(ref name) = type_descriptors[0] {
                    format!("{access_flags} {name} {field_name};")
                } else {
                    format!("{access_flags} {_type} {field_name};")
                }
            };
            writeln!(output_buffer, "\t{field_def}")?;
        }
    }
    if class.field_count > 0 {
        write!(output_buffer, "\n")?;
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
                    name = this_class_name.clone();
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
            writeln!(output_buffer, "\t{access_flags} {{}};")?;
        } else {
            method.get_params(&class.constant_pool);
            let params = method
                .get_params(&class.constant_pool)
                .iter()
                .filter(|param| !param.is_empty())
                .cloned()
                .collect::<Vec<String>>()
                .join(", ");
            let return_type = method.get_return(&class.constant_pool);
            let mut method_def = if method_name == this_class_name {
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
            writeln!(output_buffer, "\t {method_def}")?;
        }
        if args.disassemble {
            disassemble(
                &this_class_name,
                &method,
                &class.constant_pool,
                &mut output_buffer,
            )?;
        }
        writeln!(output_buffer, "")?;
    }
    writeln!(output_buffer, "}}")?;
    Ok(output_buffer)
}

fn disassemble(
    this_class_name: &str,
    method: &MethodInfo,
    constant_pool: &[ConstantPool],
    output_buffer: &mut Vec<u8>,
) -> Result<(), Box<dyn std::error::Error>> {
    for attrib in &method.attributes {
        let mut longest_mnemonic: usize = 0;
        let mut largest_code_length: usize = 0;
        if let AttributeInfo::Code(code) = attrib {
            let bytes = code.code.clone();
            let mut cursor = Cursor::new(bytes.as_slice());
            if code.code_length > largest_code_length as u32 {
                largest_code_length = code.code_length as usize;
            }
            while let Ok(byte) = cursor.read_u8() {
                let mnemonic = Mnemonic::from(byte);
                let size = String::from(mnemonic).len();
                if size > longest_mnemonic {
                    longest_mnemonic = size;
                }
            }
        }
        if let AttributeInfo::Code(code) = attrib {
            let bytes = code.code.clone();
            let mut cursor = Cursor::new(bytes.as_slice());
            while let Ok(byte) = cursor.read_u8() {
                let mnemonic = Mnemonic::from(byte);
                let instruction = Instruction::from_mnemonic_cursor(&mnemonic, &mut cursor)?;
                if instruction.get_const_operands().is_empty() {
                    writeln!(
                        output_buffer,
                        "\t\t{:in_width$}: {:m_width$}",
                        cursor.position() - 1,
                        String::from(mnemonic),
                        in_width = largest_code_length.checked_ilog10().unwrap_or(0) as usize,
                        m_width = longest_mnemonic
                    )?;
                    continue;
                }
                let mut result_pool_index: i32 = -1;
                let mut result_var_index: i32 = -1;
                let mut result_imm: Vec<u8> = vec![];
                let mut result_offset: i32 = -1;

                for op in instruction.get_const_operands() {
                    if let OperandType::PoolIndex(index) = op {
                        if result_pool_index == -1 {
                            if instruction.get_const_operands().len() == 1 {
                                result_pool_index = *index as i32;
                            } else {
                                result_pool_index = (*index as i32) << 8;
                            }
                        } else {
                            result_pool_index |= *index as i32;
                        }
                    }
                    if let OperandType::Offset(offset) = op {
                        if result_offset == -1 {
                            if instruction.get_const_operands().len() == 1 {
                                result_offset = *offset as i32;
                            } else {
                                result_offset = (*offset as i32) << 8;
                            }
                        } else {
                            result_offset = (result_offset as u32 | *offset as u32) as i32;
                        }
                    }
                    if let OperandType::VarIndex(index) = op {
                        if result_var_index == -1 {
                            if instruction.get_const_operands().len() == 1 {
                                result_var_index = *index as i32;
                            } else {
                                result_var_index = (*index as i32) << 8;
                            }
                        } else {
                            result_var_index |= *index as i32;
                        }
                    }
                    // This does not work for immediate values that need to be
                    // combined into anything bigger than a u8
                    if let OperandType::Immediate(imm) = op {
                        result_imm.push(*imm);
                    }
                }
                if result_pool_index == -1
                    && result_var_index == -1
                    && result_offset == -1
                    && result_imm.is_empty()
                {
                    writeln!(output_buffer, "\t\t{:?}", instruction)?;
                    continue;
                }
                write!(
                    output_buffer,
                    "\t\t{:in_width$}: {:m_width$}",
                    cursor.position() - instruction.get_const_operands().len() as u64 - 1,
                    String::from(mnemonic),
                    in_width = largest_code_length.checked_ilog10().unwrap_or(0) as usize,
                    m_width = longest_mnemonic
                )?;
                if result_pool_index > -1 {
                    write!(output_buffer, " #{result_pool_index}\t\t\t")?;
                }
                if result_var_index > -1 {
                    write!(output_buffer, " {result_var_index}",)?;
                }
                if result_offset > -1 {
                    let destination = ((cursor.position() - 1) as i32 + result_offset)
                        - instruction.get_const_operands().len() as i32;
                    write!(output_buffer, " {destination}",)?;
                }
                if !result_imm.is_empty() {
                    for imm in result_imm {
                        write!(output_buffer, " {imm}")?;
                    }
                }
                if result_pool_index > -1 {
                    write!(
                        output_buffer,
                        "{:1$}",
                        "",
                        (constant_pool.len().checked_ilog10().unwrap_or(0) as usize)
                    )?;
                    let constant = &constant_pool[result_pool_index as usize];
                    if get_data_from_ref(this_class_name, constant_pool, constant, output_buffer)?
                        == false
                    {
                        match constant {
                            ConstantPool::String(string) => {
                                write!(output_buffer, "// String ")?;
                                if let ConstantPool::Utf8(string) =
                                    &constant_pool[string.string_index as usize]
                                {
                                    let string = String::from(string);
                                    write!(output_buffer, "{string}")?;
                                }
                            }
                            ConstantPool::Class(class) => {
                                write!(output_buffer, "// class ")?;
                                if let ConstantPool::Utf8(string) =
                                    &constant_pool[class.name_index as usize]
                                {
                                    let string = String::from(string);
                                    write!(output_buffer, "{string}")?;
                                }
                            }
                            ConstantPool::InvokeDynamic(dynamic) => {
                                if let ConstantPool::NameAndType(nam_typ) =
                                    &constant_pool[dynamic.name_and_type_index as usize]
                                {
                                    let name = nam_typ.get_name(constant_pool)?;
                                    let desc = nam_typ.get_descriptor(constant_pool)?;
                                    write!(
                                        output_buffer,
                                        "// InvokeDynamic #{}:{name}:{desc}",
                                        result_imm[0]
                                    )?;
                                }
                            }
                            _ => {
                                //dbg!(constant);
                            }
                        }
                    }
                }
                write!(output_buffer, "\n")?;
            }
        }
    }
    Ok(())
}

fn get_data_from_ref(
    this_class_name: &str,
    constant_pool: &[ConstantPool],
    r#type: &ConstantPool,
    output_buffer: &mut Vec<u8>,
) -> Result<bool, Box<dyn std::error::Error>> {
    let mut affected = false;
    let mut class_index = 0;
    let mut name_type_index = 0;
    if let ConstantPool::Methodref(meth_ref) = r#type {
        class_index = meth_ref.class_index;
        name_type_index = meth_ref.name_and_type_index;
        write!(output_buffer, "// Method ")?;
        affected = true;
    }
    if let ConstantPool::Fieldref(field_ref) = r#type {
        class_index = field_ref.class_index;
        name_type_index = field_ref.name_and_type_index;
        write!(output_buffer, "// Field ")?;
        affected = true;
    }
    if let ConstantPool::InterfaceMethodref(int_meth_ref) = r#type {
        class_index = int_meth_ref.class_index;
        name_type_index = int_meth_ref.name_and_type_index;
        write!(output_buffer, "// InterfaceMethod ")?;
        affected = true;
    }
    if affected == false {
        return Ok(affected);
    }
    let class_const = &constant_pool[class_index as usize];
    if let ConstantPool::Class(c) = class_const {
        let class_name = &constant_pool[c.name_index as usize];
        if let ConstantPool::Utf8(name) = class_name {
            let name = String::from(name);
            if name != this_class_name {
                write!(output_buffer, "{name}.")?;
            }
            affected = true;
        }
    }
    let name_type_const = &constant_pool[name_type_index as usize];
    if let ConstantPool::NameAndType(nt) = name_type_const {
        let name = nt.get_name(constant_pool)?;
        if name == "<init>" {
            write!(output_buffer, "\"{name}\":")?;
            affected = true;
        } else {
            write!(output_buffer, "{name}:")?;
            affected = true;
        }
        let desc = nt.get_descriptor(constant_pool)?;
        write!(output_buffer, "{desc}")?;
        affected = true;
    }
    Ok(affected)
}
