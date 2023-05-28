use byteorder::{ReadBytesExt, BE};
use std::io::Cursor;

use std::error::Error;

use crate::access_flags::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use crate::attributes;
use crate::attributes::AttributeInfo;
use crate::constants::ConstantPool;
use crate::constants::{self, Utf8};
use crate::descriptors::{FieldDescriptor, MethodDescriptor};
use crate::errors::class_format_check::{FormatCause, FormatError};

/// [Fields](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A721%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C564%2Cnull%5D)
#[derive(Clone, Debug, Default)]
pub struct FieldInfo {
    pub access_flags: Vec<FieldAccessFlags>,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub attributes: Vec<AttributeInfo>,
}

impl FieldInfo {
    pub fn new(
        flags: u16,
        name_index: u16,
        descriptor_index: u16,
        attributes_count: u16,
        cursor: &mut Cursor<&[u8]>,
        constant_pool: &[ConstantPool],
    ) -> Result<FieldInfo, Box<dyn Error>> {
        let mut attributes = Vec::with_capacity(attributes_count as usize);
        attributes::read_attributes(constant_pool, &mut attributes, cursor, None)?;
        Ok(FieldInfo {
            access_flags: FieldAccessFlags::from_u16(flags),
            name_index,
            descriptor_index,
            attributes_count,
            attributes,
        })
    }

    pub fn pretty_fmt(self, constant_pool: &[ConstantPool]) -> String {
        let mut output = String::new();
        output.push_str("FieldInfo {\n");
        output.push_str(&format!("\tFlags: {:?}\n", self.access_flags));
        output.push_str(&format!(
            "\tName: {:?}\n",
            constant_pool[self.name_index as usize]
        ));
        if let ConstantPool::Utf8(desc) = &constant_pool[self.descriptor_index as usize] {
            let desc_option: Option<Vec<FieldDescriptor>> = Option::from(desc.to_owned());
            if let Some(descriptors) = desc_option {
                output.push_str(&format!("\tDescriptor: {descriptors:?}\n"));
            } else {
                output.push_str("\tDescriptor: []\n");
            }
        }
        output.push_str(&format!("\tAttribute Count: {:?}\n", self.attributes_count));
        output.push_str(&format!("\tAttributes: {:#?}\n", self.attributes));
        output.push_str("}\n");

        output
    }

    pub fn get_type(&self, constant_pool: &[ConstantPool]) -> Vec<FieldDescriptor> {
        let Some(ref descriptors): Option<Vec<FieldDescriptor>> = (
            if let ConstantPool::Utf8(desc) =
            constant_pool[self.descriptor_index as usize].clone()
            {
                Option::from(desc)
            } else {
                unreachable!(
                    "Could not get descriptor for method at index {}",
                    self.descriptor_index
                );
            }
        ) else {
            unreachable!(
                "Could not get descriptor for method at index {}",
                self.descriptor_index
            );
        };
        descriptors.to_vec()
    }
}

/// [Methods](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A777%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C282%2Cnull%5D)
#[derive(Clone, Debug, Default)]
pub struct MethodInfo {
    pub access_flags: Vec<MethodAccessFlags>,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub attributes: Vec<AttributeInfo>,
}

impl MethodInfo {
    pub fn new(
        flags: u16,
        name_index: u16,
        descriptor_index: u16,
        attributes_count: u16,
        cursor: &mut Cursor<&[u8]>,
        constant_pool: &[ConstantPool],
        major_version: Option<u16>,
    ) -> Result<MethodInfo, Box<dyn Error>> {
        let mut attributes = Vec::with_capacity(attributes_count as usize);
        attributes::read_attributes(constant_pool, &mut attributes, cursor, major_version)?;
        Ok(MethodInfo {
            access_flags: MethodAccessFlags::from_u16(flags),
            name_index,
            descriptor_index,
            attributes_count,
            attributes,
        })
    }

    pub fn pretty_fmt(self, constant_pool: &[ConstantPool]) -> String {
        let mut output = String::new();
        output.push_str("MethodInfo {\n");
        output.push_str(&format!("\tFlags: {:?}\n", self.access_flags));
        output.push_str(&format!(
            "\tName: {:?}\n",
            constant_pool[self.name_index as usize]
        ));
        if let ConstantPool::Utf8(desc) = &constant_pool[self.descriptor_index as usize] {
            let desc_option: Option<Vec<MethodDescriptor>> = Option::from(desc.to_owned());
            if let Some(descriptors) = desc_option {
                output.push_str(&format!("\tDescriptor: {descriptors:?}\n"));
            } else {
                output.push_str("\tDescriptor: []\n");
            }
        }
        output.push_str(&format!("\tAttribute Count: {:?}\n", self.attributes_count));
        output.push_str(&format!("\tAttributes: {:#?}\n", self.attributes));
        output.push_str("}\n");

        output
    }

    pub fn get_params(&self, constant_pool: &[ConstantPool]) -> Vec<String> {
        let descriptor: String = if let ConstantPool::Utf8(desc) =
            constant_pool[self.descriptor_index as usize].clone()
        {
            String::from(&desc)
        } else {
            unreachable!(
                "Could not get descriptor for method at index {}",
                self.descriptor_index
            );
        };
        let mut params = descriptor.split(')');
        let mut params = params
            .next()
            .expect("No parameters could be found")
            .to_string();
        params.remove(0);
        let params: Vec<String> = params
            .split(';')
            .map(|param| {
                if param == "I" {
                    "int".into()
                } else {
                    param.to_string()
                }
            })
            .collect();
        let mut new_params = vec![];
        for param in params {
            let mut split: Vec<String> = param.split('L').map(|dumb| dumb.to_string()).collect();
            if split.len() > 1 {
                new_params.append(&mut split);
            } else {
                new_params.push(param.to_string());
            }
        }
        for index in 0..new_params.len() - 1 {
            if new_params[index] == "[" {
                new_params.remove(index);
            }
            let mut param =
                new_params[index].trim_matches(|c| c == ')' || c == ']' || c == ';' || c == 'L');
            param = param.trim_start_matches('L');
            if param == "I" {
                new_params[index] = "int".into();
            }
        }
        new_params
    }

    pub fn get_return(&self, constant_pool: &[ConstantPool]) -> String {
        let descriptor: String = if let ConstantPool::Utf8(desc) =
            constant_pool[self.descriptor_index as usize].clone()
        {
            String::from(&desc)
        } else {
            unreachable!(
                "Could not get descriptor for method at index {}",
                self.descriptor_index
            );
        };
        let mut return_type = descriptor.split(')');
        return_type.next().unwrap_or_else(|| {
            panic!(
                "No return type exists for {:?}",
                constant_pool[self.name_index as usize]
            )
        });
        let mut r#type = return_type
            .next()
            .unwrap_or_else(|| {
                panic!(
                    "No return type exists for {:?}",
                    constant_pool[self.name_index as usize]
                )
            })
            .to_string();
        if r#type == "V" {
            r#type = "void".into()
        }
        if r#type == "I" {
            r#type = "int".into()
        }
        r#type = r#type.trim_matches(';').to_string();
        r#type = r#type.trim_matches('L').to_string();
        r#type
    }
}

#[derive(Clone)]
pub struct ClassFile {
    /**
     * **magic**\
     *  The magic item supplies the magic number identifying the class file format;\
     *  it has the value 0xCAFEBABE.
     */
    pub magic: u32,
    /**
     * **minor_version and major_version**\
     *  The values of the minor_version and major_version items are the minor\
     *  and major version numbers of this class file. Together, a major and a minor\
     *  version number determine the version of the class file format. If a class file\
     *  has major version number M and minor version number m, we denote the version\
     *  of its class file format as M.m.
     */
    pub minor_version: u16,
    pub major_version: u16,
    /**
     * **constant_pool_count**\
     *  The value of the constant_pool_count item is equal to the number of entries\
     *  in the constant_pool table plus one. A constant_pool index is considered\
     *  valid if it is greater than zero and less than constant_pool_count, with the\
     *  exception for constants of type long and double noted in §4.4.5.
     */
    pub constant_pool_count: u16,
    /**
     * **constant_pool**\
     *  The constant_pool is a table of structures (§4.4) representing various string\
     *  constants, class and interface names, field names, and other constants that are\
     *  referred to within the ClassFile structure and its substructures. The format of\
     *  each constant_pool table entry is indicated by its first "tag" byte.\
     *  The constant_pool table is indexed from 1 to constant_pool_count - 1.
     */
    pub constant_pool: Vec<ConstantPool>,
    /**
     * **access_flags**\
     *  The value of the access_flags item is a mask of flags used to denote access\
     *  permissions to and properties of this class or interface. The interpretation of\
     *  each flag, when set, is specified in Table 4.1-B.
     */
    pub access_flags: Vec<ClassAccessFlags>,
    /**
     * **this_class**\
     *  The value of the this_class item must be a valid index into the\
     *  constant_pool table. The constant_pool entry at that index must be a\
     *  CONSTANT_Class_info structure (§4.4.1) representing the class or interface\
     *  defined by this class file.
     */
    pub this_class: u16,
    /**
     * **super_class**\
     *  For a class, the value of the super_class item either must be zero or\
     *  must be a valid index into the constant_pool table. If the value of the\
     *  super_class item is nonzero, the constant_pool entry at that index must\
     *  be a CONSTANT_Class_info structure representing the direct superclass of the\
     *  class defined by this class file. Neither the direct superclass nor any of its\
     *  superclasses may have the ACC_FINAL flag set in the access_flags item of its\
     *  ClassFile structure.
     *
     *  If the value of the super_class item is zero, then this class file must represent\
     *  the class Object, the only class or interface without a direct superclass.\
     *  For an interface, the value of the super_class item must always be a valid\
     *  index into the constant_pool table. The constant_pool entry at that index\
     *  must be a CONSTANT_Class_info structure representing the class Object.
     */
    pub super_class: u16,
    /**
     * **interfaces_count**\
     *  The value of the interfaces_count item gives the number of direct\
     *  superinterfaces of this class or interface type.
     */
    pub interfaces_count: u16,
    /**
     * **interfaces**\
     *  Each value in the interfaces array must be a valid index into\
     *  the constant_pool table. The constant_pool entry at each value\
     *  of interfaces\[i\], where 0 ≤ i < interfaces_count, must be a\
     *  CONSTANT_Class_info structure representing an interface that is a direct\
     *  superinterface of this class or interface type, in the left-to-right order given in\
     *  the source for the type.
     */
    pub interfaces: Vec<u16>,
    /**
     * **fields_count**\
     *  The value of the fields_count item gives the number of field_info\
     *  structures in the fields table. The field_info structures represent all fields,\
     *  both class variables and instance variables, declared by this class or interface\
     *  type.
     */
    pub field_count: u16,
    /**
     * **fields**\
     *  Each value in the fields table must be a field_info structure (§4.5) giving\
     *  a complete description of a field in this class or interface. The fields table\
     *  includes only those fields that are declared by this class or interface. It does\
     *  not include items representing fields that are inherited from superclasses or\
     *  superinterfaces.
     */
    pub fields: Vec<FieldInfo>,
    /**
     * **methods_count**\
     *  The value of the methods_count item gives the number of method_info
     *  structures in the methods table.
     */
    pub methods_count: u16,
    /**
     * **methods**\
     *  Each value in the methods table must be a method_info structure (§4.6) giving
     *  a complete description of a method in this class or interface. If neither of the
     *  ACC_NATIVE and ACC_ABSTRACT flags are set in the access_flags item of a
     *  method_info structure, the Java Virtual Machine instructions implementing
     *  the method are also supplied.
     *
     *  The method_info structures represent all methods declared by this class
     *  or interface type, including instance methods, class methods, instance
     *  initialization methods (§2.9.1), and any class or interface initialization method
     *  (§2.9.2). The methods table does not include items representing methods that
     *  are inherited from superclasses or superinterfaces.
     */
    pub methods: Vec<MethodInfo>,
    /**
     * **attributes_count**\
     *  The value of the attributes_count item gives the number of attributes in the
     *  attributes table of this class.
     */
    pub attributes_count: u16,
    /**
     * **attributes**\
     *  Each value of the attributes table must be an attribute_info structure
     *  (§4.7).\
     *  The attributes defined by this specification as appearing in the attributes
     *  table of a ClassFile structure are listed in Table 4.7-C.\
     *  The rules concerning attributes defined to appear in the attributes table of a
     *  ClassFile structure are given in §4.7.\
     *  The rules concerning non-predefined attributes in the attributes table of a
     *  ClassFile structure are given in §4.7.1.
     */
    pub attributes: Vec<AttributeInfo>,
}

// FIXME: IMPORTANT RULES FOR MODULES
/*
 * If the ACC_MODULE flag is set in the access_flags item, then no other flag in the
 * access_flags item may be set, and the following rules apply to the rest of the
 * ClassFile structure:
 * • major_version, minor_version: ≥ 53.0 (i.e., Java SE 9 and above)
 * • this_class: module-info
 * • super_class, interfaces_count, fields_count, methods_count: zero
 * • attributes: One Module attribute must be present. Except
 * for Module, ModulePackages, ModuleMainClass, InnerClasses,
 * SourceFile, SourceDebugExtension, RuntimeVisibleAnnotations, and
 * RuntimeInvisibleAnnotations, none of the pre-defined attributes (§4.7) may
 * appear.
*/

impl ClassFile {
    pub fn from_bytes(bytes: &[u8]) -> Result<ClassFile, Box<dyn Error>> {
        let mut cursor = Cursor::new(bytes);
        let magic = cursor.read_u32::<BE>()?;
        let minor_version = cursor.read_u16::<BE>()?;
        let major_version = cursor.read_u16::<BE>()?;
        let constant_pool_count = cursor.read_u16::<BE>()?;
        let constant_pool = {
            let mut pool = Vec::with_capacity((constant_pool_count - 1) as usize);
            pool.push(ConstantPool::Unknown);
            constants::read_constant_pool(&mut pool, &mut cursor)?;
            pool.push(ConstantPool::Utf8(Utf8::from("StackMapTable")));
            pool
        };
        let access_flags = ClassAccessFlags::from_u16(cursor.read_u16::<BE>()?);
        let this_class = cursor.read_u16::<BE>()?;
        let super_class = cursor.read_u16::<BE>()?;
        let interfaces_count = cursor.read_u16::<BE>()?;
        let interfaces = {
            let mut interfaces: Vec<u16> = Vec::with_capacity(interfaces_count as usize);
            for _ in 0..interfaces_count {
                interfaces.push(cursor.read_u16::<BE>()?);
            }
            interfaces
        };
        let field_count = cursor.read_u16::<BE>()?;
        let fields = {
            let mut fields = Vec::with_capacity(field_count as usize);
            for _ in 0..fields.capacity() {
                fields.push(FieldInfo::new(
                    cursor.read_u16::<BE>()?,
                    cursor.read_u16::<BE>()?,
                    cursor.read_u16::<BE>()?,
                    cursor.read_u16::<BE>()?,
                    &mut cursor,
                    &constant_pool,
                )?);
            }
            fields
        };
        let methods_count = cursor.read_u16::<BE>()?;
        let methods = {
            let mut methods = Vec::with_capacity(methods_count as usize);
            for _ in 0..methods.capacity() {
                methods.push(MethodInfo::new(
                    cursor.read_u16::<BE>()?,
                    cursor.read_u16::<BE>()?,
                    cursor.read_u16::<BE>()?,
                    cursor.read_u16::<BE>()?,
                    &mut cursor,
                    &constant_pool,
                    Some(major_version),
                )?);
            }
            methods
        };
        let attributes_count = cursor.read_u16::<BE>()?;
        let attributes = {
            let mut attribs = Vec::with_capacity(attributes_count as usize);
            attributes::read_attributes(
                &constant_pool,
                &mut attribs,
                &mut cursor,
                Some(major_version),
            )?;
            attribs
        };
        //FIXME: This isn't ideal, is_empty is nightly and requires a feature flag
        // • The class file must not be truncated or have extra bytes at the end.
        if !cursor.is_empty() {
            return Err(Box::new(FormatError::new(
                FormatCause::ExtraBytes,
                "class file has leftover bytes",
            )));
        }
        let class = ClassFile {
            magic,
            minor_version,
            major_version,
            constant_pool_count,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces_count,
            interfaces,
            field_count,
            fields,
            methods_count,
            methods,
            attributes_count,
            attributes,
        };
        if let Err(e) = check_format(class.clone()) {
            Err(Box::new(e))
        } else {
            Ok(class)
        }
    }

    // TODO: Improve to_pretty_fmt to provide the value from index into constant pool
    pub fn to_pretty_fmt(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("Magic: {:#04X}\n", self.magic));
        output.push_str(&format!(
            "Java Version: {}.{}\n",
            self.major_version, self.minor_version
        ));
        output.push_str(&format!(
            "Constant Pool: Size {}\n[\n",
            self.constant_pool_count
        ));
        for i in 0..self.constant_pool.len() {
            if i != 0 {
                output.push_str(&format!("{i}: {:#?}\n", self.constant_pool[i]));
            }
        }
        output.push_str("]\n");
        output.push_str(&format!("Class Access Flags: {:?}\n", self.access_flags));
        output.push_str(&format!("This Class Index: {}\n", self.this_class));
        output.push_str(&format!("Super Class Index: {}\n", self.super_class));
        output.push_str(&format!(
            "Interfaces: Size {}\n\t{:?}\n",
            self.interfaces.len(),
            self.interfaces
        ));
        output.push_str(&format!("Fields: Count {}\n", self.field_count));
        for f in self.fields.clone() {
            output.push_str(&f.pretty_fmt(&self.constant_pool));
        }
        output.push_str(&format!("Method Count: {:#}\n", self.methods_count));
        for m in self.methods.clone() {
            output.push_str(&m.pretty_fmt(&self.constant_pool));
        }
        output.push_str(&format!(
            "Attributes: {:#}\n{:#?}",
            self.attributes_count, self.attributes
        ));
        output
    }

    pub fn get_from_constant_pool(&self, index: u16) -> Result<&ConstantPool, FormatError> {
        if index > self.constant_pool_count {
            return Err(FormatError::new(FormatCause::InvalidIndex(index), ""));
        }
        Ok(&self.constant_pool[index as usize])
    }
}

/// [Format Checking](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A2235%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C590%2Cnull%5D)
fn check_format(class: ClassFile) -> Result<(), FormatError> {
    // • The first four bytes must contain the right magic number.
    if class.magic != 0xCAFEBABE {
        return Err(FormatError::new(
            FormatCause::IncorrectMagic(0xCAFEBABE),
            &format!(
                "Magic value in class file was incorrect: {:#02X?}",
                class.magic
            ),
        ));
    }
    if class.access_flags.contains(&ClassAccessFlags::AccModule) && class.access_flags.len() > 1 {
        return Err(FormatError::new(
            FormatCause::TooManyFlags,
            "Too many flags for a Module class",
        ));
    }
    // • All predefined attributes (§4.7) must be of the proper
    //      length, except for StackMapTable, RuntimeVisibleAnnotations,
    //      RuntimeInvisibleAnnotations, RuntimeVisibleParameterAnnotations,
    //      RuntimeInvisibleParameterAnnotations,
    //      RuntimeVisibleTypeAnnotations, RuntimeInvisibleTypeAnnotations, and
    //      AnnotationDefault.
    // NOTE: Due to the nature of our implementation, attributes should not be able to
    //       be of incorrect length without there being an error elsewhere in the class loader

    // • The constant pool must satisfy the constraints documented throughout §4.4
    for constant in &class.constant_pool {
        match constant {
            ConstantPool::Class(c) => {
                let ConstantPool::Utf8(_) = class.get_from_constant_pool(c.name_index)? else {
                    return Err(FormatError::new(
                        FormatCause::InvalidIndex(c.name_index),
                        "Class name_index was not a Utf8 Constant"
                    ));
                };
            }
            ConstantPool::String(s) => {
                let ConstantPool::Utf8(_) = class.get_from_constant_pool(s.string_index)? else {
                    return Err(FormatError::new(
                        FormatCause::InvalidIndex(s.string_index),
                        "String string_index was not a Utf8 Constant"
                    ));
                };
            }
            ConstantPool::Fieldref(f) => {
                let ConstantPool::Class(_) = class.get_from_constant_pool(f.class_index)? else {
                    return Err(FormatError::new(
                        FormatCause::InvalidIndex(f.class_index),
                        "Fieldref class_index was not a Class Constant"
                    ));
                };
                let ConstantPool::NameAndType(nat) = class.get_from_constant_pool(f.name_and_type_index)? else {
                    return Err(FormatError::new(
                        FormatCause::InvalidIndex(f.name_and_type_index),
                        "Fieldref name_and_type_index was not a NameAndType Constant"
                    ));
                };
                let ConstantPool::Utf8(desc) = class.get_from_constant_pool(nat.descriptor_index)? else {
                    return Err(FormatError::new(
                        FormatCause::InvalidIndex(nat.descriptor_index),
                        "Fieldref name_and_type_index.descriptor_index was not a Utf8 Constant"
                    ));
                };
                let descriptor: Option<Vec<FieldDescriptor>> = Option::from(desc.clone());
                if descriptor.is_none() {
                    return Err(FormatError::new(
                        FormatCause::InvalidDescriptor(String::from(desc)),
                        "Fieldref name_and_type_index.descriptor_index was a MethodDescriptor",
                    ));
                }
            }
            ConstantPool::Methodref(m) => {
                let ConstantPool::Class(_) = class.get_from_constant_pool(m.class_index)? else {
                    return Err(FormatError::new(
                        FormatCause::InvalidIndex(m.class_index),
                        "MethodRef class_index was not a Class Constant"
                    ));
                };
                let ConstantPool::NameAndType(nat) = class.get_from_constant_pool(m.name_and_type_index)? else {
                    return Err(FormatError::new(
                        FormatCause::InvalidIndex(m.name_and_type_index),
                        "MethodRef name_and_type_index was not a NameAndType Constant"
                    ));
                };
                let ConstantPool::Utf8(name) = class.get_from_constant_pool(nat.name_index)? else {
                    return Err(FormatError::new(
                        FormatCause::InvalidIndex(nat.descriptor_index),
                        "MethodRef name_and_type_index.name_index was not a Utf8 Constant"
                    ));
                };
                let ConstantPool::Utf8(desc) = class.get_from_constant_pool(nat.descriptor_index)? else {
                    return Err(FormatError::new(
                        FormatCause::InvalidIndex(nat.descriptor_index),
                        "MethodRef name_and_type_index.descriptor_index was not a Utf8 Constant"
                    ));
                };
                let descriptor: Option<Vec<MethodDescriptor>> = Option::from(desc.clone());
                if let Some(descrip) = descriptor {
                    let name = String::from(name);
                    if name == "<init>" && !descrip.contains(&MethodDescriptor::VoidReturn) {
                        println!("{descrip:?}");
                        return Err(FormatError::new(
                            FormatCause::InvalidDescriptor(String::from(desc)),
                            "Methodref descriptor did not contain Void",
                        ));
                    }
                } else {
                    return Err(FormatError::new(
                        FormatCause::InvalidDescriptor(String::from(desc)),
                        "Methodref name_and_type_index.descriptor_index was a FieldDescriptor",
                    ));
                }
            }
            ConstantPool::InterfaceMethodref(im) => {
                let ConstantPool::Class(_) = class.get_from_constant_pool(im.class_index)? else {
                    return Err(FormatError::new(
                        FormatCause::InvalidIndex(im.class_index),
                        "InterfaceMethodref class_index was not a Class Constant"
                    ));
                };
                let ConstantPool::NameAndType(nat) = class.get_from_constant_pool(im.name_and_type_index)? else {
                    return Err(FormatError::new(
                        FormatCause::InvalidIndex(im.name_and_type_index),
                        "InterfaceMethodref name_and_type_index was not a NameAndType Constant"
                    ));
                };
                let ConstantPool::Utf8(desc) = class.get_from_constant_pool(nat.descriptor_index)? else {
                    return Err(FormatError::new(
                        FormatCause::InvalidIndex(nat.descriptor_index),
                        "InterfaceMethodref name_and_type_index.descriptor_index was not a Utf8 Constant"
                    ));
                };
                let descriptor: Option<Vec<MethodDescriptor>> = Option::from(desc.clone());
                if descriptor.is_none() {
                    return Err(FormatError::new(
                        FormatCause::InvalidDescriptor(String::from(desc)),
                        "InterfaceMethodref name_and_type_index.descriptor_index was a FieldDescriptor",
                    ));
                }
            }
            ConstantPool::NameAndType(nt) => {
                let ConstantPool::Utf8(_) = class.get_from_constant_pool(nt.name_index)? else {
                    return Err(FormatError::new(
                        FormatCause::InvalidIndex(nt.name_index),
                        "NameAndType name_index was not a Utf8 Constant"
                    ));
                };
                let ConstantPool::Utf8(_) = class.get_from_constant_pool(nt.descriptor_index)? else {
                    return Err(FormatError::new(
                        FormatCause::InvalidIndex(nt.descriptor_index),
                        "NameAndType descriptor_index was not a Utf8 Constant"
                    ));
                };
            }
            ConstantPool::MethodHandle(mh) => {
                let reference_kind_u8 = mh.reference_kind.clone() as u8;
                match reference_kind_u8 {
                    1..=4 => {
                        let ConstantPool::Fieldref(_) = class.get_from_constant_pool(mh.reference_index)? else {
                            return Err(FormatError::new(
                                FormatCause::InvalidIndex(mh.reference_index),
                                "MethodHandle reference_index was not a Fieldref Constant"
                            ));
                        };
                    }
                    5 | 8 => {
                        let ConstantPool::Methodref(_) = class.get_from_constant_pool(mh.reference_index)? else {
                            return Err(FormatError::new(
                                FormatCause::InvalidIndex(mh.reference_index),
                                "MethodHandle reference_index was not a Methodref Constant"
                            ));
                        };
                    }
                    6 | 7 => {
                        if class.major_version < 52 {
                            let ConstantPool::Methodref(_) = class.get_from_constant_pool(mh.reference_index)? else {
                                return Err(FormatError::new(
                                    FormatCause::InvalidIndex(mh.reference_index),
                                    "MethodHandle reference_index was not a Methodref Constant"
                                ));
                            };
                        } else {
                            match class.get_from_constant_pool(mh.reference_index)? {
                                ConstantPool::Methodref(_) => {}
                                ConstantPool::InterfaceMethodref(_) => {}
                                _ => {
                                    return Err(FormatError::new(
                                        FormatCause::InvalidIndex(
                                            mh.reference_index,
                                        ),
                                        "MethodHandle reference_index was neither a Methodref or InterfaceMethodref Constant",
                                    ));
                                }
                            }
                        }
                    }
                    9 => {
                        let ConstantPool::InterfaceMethodref(_) = class.get_from_constant_pool(mh.reference_index)? else {
                            return Err(FormatError::new(
                                FormatCause::InvalidIndex(mh.reference_index),
                                "MethodHandle reference_index was not a InterfaceMethodref Constant"
                            ));
                        };
                    }
                    _ => {
                        return Err(FormatError::new(
                            FormatCause::InvalidReferenceKind(reference_kind_u8),
                            "MethodHandle reference kind was invalid",
                        ));
                    }
                }
            }
            ConstantPool::MethodType(mt) => {
                let ConstantPool::Utf8(_) = class.get_from_constant_pool(mt.descriptor_index)? else {
                    return Err(FormatError::new(
                        FormatCause::InvalidIndex(mt.descriptor_index),
                        "MethodType name_index was not a Utf8 Constant"
                    ));
                };
            }
            ConstantPool::Dynamic(d) => {
                let ConstantPool::NameAndType(_) = class.get_from_constant_pool(d.name_and_type_index)? else {
                    return Err(FormatError::new(
                        FormatCause::InvalidIndex(d.name_and_type_index),
                        "Dynamic name_and_type_index was not a NameAndType Constant"
                    ));
                };
                let Some(AttributeInfo::BootstrapMethods(bm)) =
                    class.attributes.iter().find(|a| {
                        matches!(a, AttributeInfo::BootstrapMethods(_))
                    })
                else {
                    return Err(FormatError::new(
                        FormatCause::MissingAttribute,
                        "Missing BootstrapMethods attribute required by ConstantPool::Dynamic"
                    ));
                };
                if bm.bootstrap_methods.len() < d.bootstrap_method_attr_index as usize {
                    return Err(FormatError::new(
                        FormatCause::InvalidIndex(d.name_and_type_index),
                        "Dynamic bootstrap_method_attr_index was not a valid index into BootstrapMethods attribute",
                    ));
                }
            }
            ConstantPool::InvokeDynamic(id) => {
                let ConstantPool::NameAndType(_) = class.get_from_constant_pool(id.name_and_type_index)? else {
                    return Err(FormatError::new(
                        FormatCause::InvalidIndex(id.name_and_type_index),
                        "Dynamic name_and_type_index was not a NameAndType Constant"
                    ));
                };
                let Some(AttributeInfo::BootstrapMethods(bm)) =
                    class.attributes.iter().find(|a| {
                        matches!(a, AttributeInfo::BootstrapMethods(_))
                    })
                else {
                    return Err(FormatError::new(
                        FormatCause::MissingAttribute,
                        "Missing BootstrapMethods attribute required by ConstantPool::Dynamic"
                    ));
                };
                if bm.bootstrap_methods.len() < id.bootstrap_method_attr_index as usize {
                    return Err(FormatError::new(
                        FormatCause::InvalidIndex(id.name_and_type_index),
                        "Dynamic bootstrap_method_attr_index was not a valid index into BootstrapMethods attribute",
                    ));
                }
            }
            ConstantPool::Module(mo) => {
                if class.access_flags.contains(&ClassAccessFlags::AccModule) {
                    let ConstantPool::Utf8(_) = class.get_from_constant_pool(mo.name_index)? else {
                        return Err(FormatError::new(
                            FormatCause::InvalidIndex(mo.name_index),
                            "Module name_index was not a Utf8 Constant"
                        ));
                    };
                } else {
                    return Err(FormatError::new(
                        FormatCause::InvalidConstant(constant.clone()),
                        "Constant is not permitted when class is not a Module",
                    ));
                }
            }
            ConstantPool::Package(p) => {
                if class.access_flags.contains(&ClassAccessFlags::AccModule) {
                    let ConstantPool::Utf8(_) = class.get_from_constant_pool(p.name_index)? else {
                        return Err(FormatError::new(
                            FormatCause::InvalidIndex(p.name_index),
                            "Module name_index was not a Utf8 Constant"
                        ));
                    };
                } else {
                    return Err(FormatError::new(
                        FormatCause::InvalidConstant(constant.clone()),
                        "Constant is not permitted when class is not a Module",
                    ));
                }
            }
            _ => {}
        }
    }

    // • All field references and method references in the constant pool must have valid
    //      names, valid classes, and valid descriptors (§4.3).

    Ok(())
}
