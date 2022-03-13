use byteorder::{ReadBytesExt, BE};
use std::io::{Cursor, Seek};
use std::str::from_utf8;
use std::{error::Error, io::Read};

use crate::{attributes, access_flags};
use crate::constants::{self, Tags};
use crate::access_flags::{ClassAccessFlags, MethodAccessFlags, FieldAccessFlags};
use crate::errors::{
    class_format_check::{FormatCause, FormatError},
    class_loading::{LoadingCause, LoadingError},
};

/// [The Constant Pool](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A2201%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C256%2Cnull%5D)
#[derive(Clone, Debug)]
pub enum ConstantPool {
    Utf8(constants::Utf8),
    Integer(constants::Integer),
    Float(constants::Float),
    Long(constants::Long),
    Double(constants::Double),
    Class(constants::Class),
    String(constants::String),
    Fieldref(constants::Fieldref),
    Methodref(constants::Methodref),
    InterfaceMethodRef(constants::InterfaceMethodref),
    NameAndType(constants::NameAndType),
    MethodHandle(constants::MethodHandle),
    MethodType(constants::MethodType),
    Dynamic(constants::Dynamic),
    InvokeDynamic(constants::InvokeDynamic),
    Module(constants::Module),
    Package(constants::Package),
}

/// [Attributes](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A1244%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C564%2Cnull%5D)
#[derive(Clone, Debug)]
pub enum AttributeInfo {
    ConstantValue(attributes::ConstantValue),
    Code(attributes::Code),
    StackMapTable(attributes::StackMapTable),
    Exceptions(attributes::Exceptions),
    InnerClasses(attributes::InnerClasses),
    EnclosingMethod(attributes::EnclosingMethod),
    Synthetic(attributes::Synthetic),
    Signature(attributes::Signature),
    SourceFile(attributes::SourceFile),
    SourceDebugExtension(attributes::SourceDebugExtension),
    LineNumberTable(attributes::LineNumberTable),
    LocalVariableTable(attributes::LocalVariableTable),
    LocalVariableTypeTable(attributes::LocalVariableTypeTable),
    Deprecated(attributes::Deprecated),
    RuntimeVisibleAnnotations(attributes::RuntimeVisibleAnnotations),
    RuntimeInvisibleAnnotations(attributes::RuntimeInvisibleAnnotations),
    RuntimeVisibleParameterAnnotations(attributes::RuntimeVisibleParameterAnnotations),
    RuntimeInvisibleParameterAnnotations(attributes::RuntimeInvisibleParameterAnnotations),
    RuntimeVisibleTypeAnnotations(attributes::RuntimeVisibleTypeAnnotations),
    RuntimeInvisibleTypeAnnotations(attributes::RuntimeInvisibleTypeAnnotations),
    AnnotationDefault(attributes::AnnotationDefault),
    BootstrapMethods(attributes::BootstrapMethods),
    MethodParameters(attributes::MethodParameters),
    Module(attributes::Module),
    ModulePackages(attributes::ModulePackages),
    ModuleMainClass(attributes::ModuleMainClass),
    NestHost(attributes::NestHost),
    NestMembers(attributes::NestMembers),
    Record(attributes::Record),
    PermittedSubclasses(attributes::PermittedSubclasses),
    Unknown
}

/// [Fields](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A721%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C564%2Cnull%5D)
#[derive(Clone, Debug)]
struct FieldInfo {
    access_flags: Vec<FieldAccessFlags>,
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
    attributes: Vec<AttributeInfo>,
}

impl Default for FieldInfo {
    fn default() -> FieldInfo {
        FieldInfo {
            access_flags: vec![],
            name_index: 0,
            descriptor_index: 0,
            attributes_count: 0,
            attributes: vec![],
        }
    }
}

impl FieldInfo {
    pub fn new(flags: u16, name_index: u16, descriptor_index: u16, attributes_count: u16, cursor: &mut Cursor<&[u8]>, constant_pool: &Vec<ConstantPool>)
        -> Result<FieldInfo, Box<dyn Error>> {
        let mut attributes = Vec::with_capacity(attributes_count as usize);
        read_attributes(constant_pool, &mut attributes, cursor)?;
        Ok(FieldInfo {
            access_flags: FieldAccessFlags::from_u16(flags),
            name_index,
            descriptor_index,
            attributes_count,
            attributes
        })
    }
}

/// [Methods](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A777%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C282%2Cnull%5D)
#[derive(Clone, Debug)]
struct MethodInfo {
    access_flags: Vec<MethodAccessFlags>,
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
    attributes: Vec<AttributeInfo>,
}

impl Default for MethodInfo {
    fn default() -> MethodInfo {
        MethodInfo {
            access_flags: vec![],
            name_index: 0,
            descriptor_index: 0,
            attributes_count: 0,
            attributes: vec![],
        }
    }
}

impl MethodInfo {
    pub fn new(flags: u16, name_index: u16, descriptor_index: u16, attributes_count: u16, cursor: &mut Cursor<&[u8]>, constant_pool: &Vec<ConstantPool>)
        -> Result<MethodInfo, Box<dyn Error>> {
        println!("Index: {:#}", name_index);
        if let ConstantPool::Utf8(n) = &constant_pool[name_index as usize-1] {
            println!("Name: {}", n.get_string());
        }
        let mut attributes = Vec::with_capacity(attributes_count as usize);
        read_attributes(constant_pool, &mut attributes, cursor)?;
        Ok(MethodInfo {
            access_flags: MethodAccessFlags::from_u16(flags),
            name_index,
            descriptor_index,
            attributes_count,
            attributes
        })
    }

    pub fn print(self, constant_pool: &Vec<ConstantPool>) {
        println!("MethodInfo {{");
        println!("\tFlags: {:?}", self.access_flags);
        println!("\tName: {:?}", constant_pool[self.name_index as usize]);
        println!("\tDescriptor: {:?}", constant_pool[self.descriptor_index as usize]);
        println!("\tAttribute Count: {:?}", self.attributes_count);
        println!("\tAttributes: {:?}", self.attributes);
        println!("}}");
    }
}

#[derive(Clone)]
pub struct ClassFile {
    /**
     * **magic**\
     *  The magic item supplies the magic number identifying the class file format;\
     *  it has the value 0xCAFEBABE.
     */
    magic: u32,
    /**
     * **minor_version and major_version**\
     *  The values of the minor_version and major_version items are the minor\
     *  and major version numbers of this class file. Together, a major and a minor\
     *  version number determine the version of the class file format. If a class file\
     *  has major version number M and minor version number m, we denote the version\
     *  of its class file format as M.m.
     */
    minor_version: u16,
    major_version: u16,
    /**
     * **constant_pool_count**\
     *  The value of the constant_pool_count item is equal to the number of entries\
     *  in the constant_pool table plus one. A constant_pool index is considered\
     *  valid if it is greater than zero and less than constant_pool_count, with the\
     *  exception for constants of type long and double noted in §4.4.5.
     */
    constant_pool_count: u16,
    /**
     * **constant_pool**\
     *  The constant_pool is a table of structures (§4.4) representing various string\
     *  constants, class and interface names, field names, and other constants that are\
     *  referred to within the ClassFile structure and its substructures. The format of\
     *  each constant_pool table entry is indicated by its first "tag" byte.\
     *  The constant_pool table is indexed from 1 to constant_pool_count - 1.
     */
    constant_pool: Vec<ConstantPool>,
    /**
     * **access_flags**\
     *  The value of the access_flags item is a mask of flags used to denote access\
     *  permissions to and properties of this class or interface. The interpretation of\
     *  each flag, when set, is specified in Table 4.1-B.
     */
    access_flags: Vec<ClassAccessFlags>,
    /**
     * **this_class**\
     *  The value of the this_class item must be a valid index into the\
     *  constant_pool table. The constant_pool entry at that index must be a\
     *  CONSTANT_Class_info structure (§4.4.1) representing the class or interface\
     *  defined by this class file.
     */
    this_class: u16,
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
    super_class: u16,
    /**
     * **interfaces_count**\
     *  The value of the interfaces_count item gives the number of direct\
     *  superinterfaces of this class or interface type.
     */
    interfaces_count: u16,
    /**
     * **interfaces**\
     *  Each value in the interfaces array must be a valid index into\
     *  the constant_pool table. The constant_pool entry at each value\
     *  of interfaces[i], where 0 ≤ i < interfaces_count, must be a\
     *  CONSTANT_Class_info structure representing an interface that is a direct\
     *  superinterface of this class or interface type, in the left-to-right order given in\
     *  the source for the type.
     */
    interfaces: Vec<u16>,
    /**
     * **fields_count**\
     *  The value of the fields_count item gives the number of field_info\
     *  structures in the fields table. The field_info structures represent all fields,\
     *  both class variables and instance variables, declared by this class or interface\
     *  type.
     */
    field_count: u16,
    /**
     * **fields**\
     *  Each value in the fields table must be a field_info structure (§4.5) giving\
     *  a complete description of a field in this class or interface. The fields table\
     *  includes only those fields that are declared by this class or interface. It does\
     *  not include items representing fields that are inherited from superclasses or\
     *  superinterfaces.
     */
    fields: Vec<FieldInfo>,
    /**
     * **methods_count**\
     *  The value of the methods_count item gives the number of method_info
     *  structures in the methods table.
     */
    methods_count: u16,
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
    methods: Vec<MethodInfo>,
    /**
     * **attributes_count**\
     *  The value of the attributes_count item gives the number of attributes in the
     *  attributes table of this class.
     */
    attributes_count: u16,
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
    attributes: Vec<AttributeInfo>,
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
            constants::read_constant_pool(&mut pool, &mut cursor)?;
            println!("Size: {}\n{:#?}", pool.len(), pool);
            pool
        };
        let access_flags = ClassAccessFlags::from_u16(cursor.read_u16::<BE>()?);
        let this_class = cursor.read_u16::<BE>()?;
        let super_class = cursor.read_u16::<BE>()?;
        let interfaces_count = cursor.read_u16::<BE>()?;
        let interfaces = {
            let mut interfaces: Vec<u16> = vec![];
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
                    &mut cursor, &constant_pool
                )?);
            }
            fields
        };
        println!("Fields: {:?}", fields);
        let methods_count = cursor.read_u16::<BE>()?;
        println!("Method Count: {:#}", methods_count);
        let methods = {
            let mut methods = Vec::with_capacity(methods_count as usize);
            for _ in 0..methods.capacity() {
                println!("Cursor Pos: {:#04x}", cursor.position());
                methods.push(MethodInfo::new(
                    cursor.read_u16::<BE>()?,
                    cursor.read_u16::<BE>()?,
                    cursor.read_u16::<BE>()?,
                    cursor.read_u16::<BE>()?,
                    &mut cursor, &constant_pool
                )?);
            }
            methods
        };
        for m in methods.clone() {
            m.print(&constant_pool);
        }
        let attributes_count = cursor.read_u16::<BE>()?;
        let attributes = {
            let mut attribs = Vec::with_capacity(attributes_count as usize);
            read_attributes(&constant_pool, &mut attribs, &mut cursor)?;
            attribs
        };

        //FIXME: This isn't ideal, is_empty is nightly and requires a feature flag
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
}

/// [Format Checking](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A2235%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C590%2Cnull%5D)
fn check_format(class: ClassFile) -> Result<(), FormatError> {
    // • The first four bytes must contain the right magic number.
    if class.magic != 0xCAFEBABE {
        return Err(FormatError::new(
            FormatCause::MagicNotCorrect,
            "Magic value in class file was incorrect",
        ));
    }
    // • All predefined attributes (§4.7) must be of the proper
    //      length, except for StackMapTable, RuntimeVisibleAnnotations,
    //      RuntimeInvisibleAnnotations, RuntimeVisibleParameterAnnotations,
    //      RuntimeInvisibleParameterAnnotations,
    //      RuntimeVisibleTypeAnnotations, RuntimeInvisibleTypeAnnotations, and
    //      AnnotationDefault.

    // FIXME: Can't really check this in here
    // • The class file must not be truncated or have extra bytes at the end.

    // • The constant pool must satisfy the constraints documented throughout §4.4

    // • All field references and method references in the constant pool must have valid
    //      names, valid classes, and valid descriptors (§4.3).

    Ok(())
}

pub(crate) fn read_attributes(
    constant_pool: &Vec<ConstantPool>,
    attributes: &mut Vec<AttributeInfo>,
    cursor: &mut Cursor<&[u8]>
) -> Result<(), Box<dyn Error>> {
    let size = attributes.capacity();
    for _ in 0..size {
        let name_index = cursor.read_u16::<BE>()?;
        let name = &constant_pool[name_index as usize];
        if let ConstantPool::Utf8(n) = name {
            println!("{}", n.get_string());
            let attribute = match n.get_string().as_str() {
                "ConstantValue" => AttributeInfo::ConstantValue(
                    attributes::ConstantValue::new(name_index, cursor.read_u32::<BE>()?, cursor.read_u16::<BE>()?)
                ),
                "Code" => AttributeInfo::Code(
                    attributes::Code::new(name_index, &constant_pool, cursor)?
                ),
                // "StackMapTable" => AttributeInfo::StackMapTable(
                    
                // ),
                // "Exceptions" => AttributeInfo::Exceptions(

                // ),
                // "InnerClasses" => AttributeInfo::InnerClasses(

                // ),
                // "EnclosingMethod" => AttributeInfo::EnclosingMethod(

                // ),
                // "Synthetic" => AttributeInfo::Synthetic(

                // ),
                // "Signature" => AttributeInfo::Signature(

                // ),
                // "SourceFile" => AttributeInfo::SourceFile(

                // ),
                // "SourceDebugExtension" => AttributeInfo::SourceDebugExtension(

                // ),
                "LineNumberTable" => AttributeInfo::LineNumberTable(
                    attributes::LineNumberTable::new(
                        name_index,
                        cursor.read_u32::<BE>()?,
                        cursor.read_u16::<BE>()?,
                        cursor
                    )?
                ),
                // "LocalVariableTable" => AttributeInfo::LocalVariableTable(

                // ),
                // "LocalVariableTypeTable" => AttributeInfo::LocalVariableTypeTable(

                // ),
                // "Deprecated" => AttributeInfo::Deprecated(

                // ),
                // "RuntimeVisibleAnnotations" => AttributeInfo::RuntimeVisibleAnnotations(

                // ),
                // "RuntimeInvisibleAnnotations" => AttributeInfo::RuntimeInvisibleAnnotations(

                // ),
                // "RuntimeVisibleParameterAnnotations" => AttributeInfo::RuntimeVisibleParameterAnnotations(

                // ),
                // "RuntimeInvisibleParameterAnnotations" => AttributeInfo::RuntimeInvisibleParameterAnnotations(

                // ),
                // "RuntimeVisibleTypeAnnotations" => AttributeInfo::RuntimeVisibleTypeAnnotations(

                // ),
                // "RuntimeInvisibleTypeAnnotations" => AttributeInfo::RuntimeInvisibleTypeAnnotations(

                // ),
                // "AnnotationDefault" => AttributeInfo::AnnotationDefault(

                // ),
                // "BootstrapMethods" => AttributeInfo::BootstrapMethods(

                // ),
                // "MethodParameters" => AttributeInfo::MethodParameters(

                // ),
                // "Module" => AttributeInfo::Module(

                // ),
                // "ModulePackages" => AttributeInfo::ModulePackages(

                // ),
                // "ModuleMainClass" => AttributeInfo::ModuleMainClass(

                // ),
                // "NestHost" => AttributeInfo::NestHost(

                // ),
                // "NestMembers" => AttributeInfo::NestMembers(

                // ),
                // "Record" => AttributeInfo::Record(

                // ),
                // "PermittedSubclasses" => AttributeInfo::PermittedSubclasses(

                // ),
                _ => AttributeInfo::Unknown
            };
            attributes.push(attribute);
        } else {
            return Err(Box::new(LoadingError::new(
                LoadingCause::InvalidAttributeNameIndex(name.clone()),
                &format!("Cursor Position: {:#04x?}", cursor.position() - 1),
            )))
        }
    }

    Ok(())
}