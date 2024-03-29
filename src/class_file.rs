use byteorder::{ReadBytesExt, BE};
use std::io::Cursor;

use std::error::Error;

use crate::access_flags::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use crate::attributes;
use crate::constants::{self, Utf8};
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
    Unknown,
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
    Unknown(String),
}

/// [Fields](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A721%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C564%2Cnull%5D)
#[derive(Clone, Debug, Default)]
pub struct FieldInfo {
    pub(crate) access_flags: Vec<FieldAccessFlags>,
    pub(crate) name_index: u16,
    pub(crate) descriptor_index: u16,
    pub(crate) attributes_count: u16,
    pub(crate) attributes: Vec<AttributeInfo>,
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

    pub fn get_type(&self, constant_pool: &[ConstantPool]) -> String {
        let mut descriptor = if let ConstantPool::Utf8(desc) =
            constant_pool[self.descriptor_index as usize].clone()
        {
            desc.get_string()
        } else {
            unreachable!(
                "Could not get descriptor for method at index {}",
                self.descriptor_index
            );
        };
        if descriptor == "V" {
            descriptor = "void".into()
        }
        if descriptor == "I" {
            descriptor = "int".into()
        }
        descriptor = descriptor.trim_matches(';').to_string();
        descriptor = descriptor.trim_matches('L').to_string();
        descriptor
    }
}

/// [Methods](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A777%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C282%2Cnull%5D)
#[derive(Clone, Debug, Default)]
pub struct MethodInfo {
    pub(crate) access_flags: Vec<MethodAccessFlags>,
    pub(crate) name_index: u16,
    pub(crate) descriptor_index: u16,
    pub(crate) attributes_count: u16,
    pub(crate) attributes: Vec<AttributeInfo>,
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
        // if let ConstantPool::Utf8(n) = &constant_pool[name_index as usize-1] {
        //     println!("Name: {}", n.get_string());
        // }
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

    pub fn to_pretty_fmt(self, constant_pool: &[ConstantPool]) -> String {
        let mut output = String::new();
        output.push_str(&format!("MethodInfo {{\n"));
        output.push_str(&format!("\tFlags: {:?}\n", self.access_flags));
        output.push_str(&format!(
            "\tName: {:?}\n",
            constant_pool[self.name_index as usize]
        ));
        output.push_str(&format!(
            "\tDescriptor: {:?}\n",
            constant_pool[self.descriptor_index as usize]
        ));
        output.push_str(&format!("\tAttribute Count: {:?}\n", self.attributes_count));
        output.push_str(&format!("\tAttributes: {:#?}\n", self.attributes));
        output.push_str(&format!("}}\n"));

        output
    }

    pub fn get_params(&self, constant_pool: &[ConstantPool]) -> Vec<String> {
        let descriptor = if let ConstantPool::Utf8(desc) =
            constant_pool[self.descriptor_index as usize].clone()
        {
            desc.get_string()
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
        let descriptor = if let ConstantPool::Utf8(desc) =
            constant_pool[self.descriptor_index as usize].clone()
        {
            desc.get_string()
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
        output.push_str(&format!("Fields:\n{:#?}\n", self.fields));
        output.push_str(&format!("Method Count: {:#}\n", self.methods_count));
        for m in self.methods.clone() {
            output.push_str(&m.to_pretty_fmt(&self.constant_pool));
        }
        output.push_str(&format!(
            "Attributes: {:#}\n{:#?}",
            self.attributes_count, self.attributes
        ));
        output
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

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;
    use std::{
        fs::{read_to_string, File},
        io::Read,
    };

    const TEST_PATH: &str = "test_verified_output/";

    fn load_class(path: &str) -> Result<ClassFile, Box<dyn Error>> {
        let mut class_file: File = File::open(path).expect("Failed to open file");
        let mut contents = vec![00; class_file.metadata().unwrap().len() as usize];
        class_file
            .read_exact(&mut contents)
            .expect("Failed to read bytes");
        ClassFile::from_bytes(&contents)
    }

    #[test]
    fn test_aiq() -> Result<(), Box<dyn Error>> {
        let output = read_to_string(TEST_PATH.to_string() + "aiq/aiq.class.txt")?;
        assert_eq!(
            load_class(&(TEST_PATH.to_string() + "aiq/aiq.class"))?.to_pretty_fmt(),
            output
        );
        Ok(())
    }

    #[test]
    fn test_java_basic_main() -> Result<(), Box<dyn Error>> {
        let output = read_to_string(TEST_PATH.to_string() + "basic_main_java_test/test.class.txt")?;
        assert_eq!(
            load_class(&(TEST_PATH.to_string() + "basic_main_java_test/test.class"))?
                .to_pretty_fmt(),
            output
        );
        Ok(())
    }

    #[test]
    fn test_kotlin_basic_main() -> Result<(), Box<dyn Error>> {
        let output =
            read_to_string(TEST_PATH.to_string() + "basic_main_kotlin_test/TestKt.class.txt")?;
        assert_eq!(
            load_class(&(TEST_PATH.to_string() + "basic_main_kotlin_test/TestKt.class"))?
                .to_pretty_fmt(),
            output
        );
        Ok(())
    }

    #[test]
    fn test_scala_basic_main() -> Result<(), Box<dyn Error>> {
        let output =
            read_to_string(TEST_PATH.to_string() + "basic_main_scala_test/test$.class.txt")?;
        assert_eq!(
            load_class(&(TEST_PATH.to_string() + "basic_main_scala_test/test$.class"))?
                .to_pretty_fmt(),
            output
        );
        Ok(())
    }

    #[test]
    fn test_java_annotations() -> Result<(), Box<dyn Error>> {
        let test_class_output =
            read_to_string(TEST_PATH.to_string() + "annotations_java_test/test.class.txt")?;
        let atRuntime_class_output =
            read_to_string(TEST_PATH.to_string() + "annotations_java_test/atRuntime.class.txt")?;
        let atCompile_class_output =
            read_to_string(TEST_PATH.to_string() + "annotations_java_test/atCompile.class.txt")?;
        let atRuntimeType_class_output = read_to_string(
            TEST_PATH.to_string() + "annotations_java_test/atRuntimeType.class.txt",
        )?;
        let atCompileType_class_output = read_to_string(
            TEST_PATH.to_string() + "annotations_java_test/atCompileType.class.txt",
        )?;
        let invisibleAnnotation_class_output = read_to_string(
            TEST_PATH.to_string() + "annotations_java_test/invisibleAnnotation.class.txt",
        )?;
        let visibleAnnotation_class_output = read_to_string(
            TEST_PATH.to_string() + "annotations_java_test/visibleAnnotation.class.txt",
        )?;
        assert_eq!(
            load_class(&(TEST_PATH.to_string() + "annotations_java_test/test.class"))?
                .to_pretty_fmt(),
            test_class_output
        );
        assert_eq!(
            load_class(&(TEST_PATH.to_string() + "annotations_java_test/atRuntime.class"))?
                .to_pretty_fmt(),
            atRuntime_class_output
        );
        assert_eq!(
            load_class(&(TEST_PATH.to_string() + "annotations_java_test/atCompile.class"))?
                .to_pretty_fmt(),
            atCompile_class_output
        );
        assert_eq!(
            load_class(&(TEST_PATH.to_string() + "annotations_java_test/atRuntimeType.class"))?
                .to_pretty_fmt(),
            atRuntimeType_class_output
        );
        assert_eq!(
            load_class(&(TEST_PATH.to_string() + "annotations_java_test/atCompileType.class"))?
                .to_pretty_fmt(),
            atCompileType_class_output
        );
        assert_eq!(
            load_class(
                &(TEST_PATH.to_string() + "annotations_java_test/invisibleAnnotation.class")
            )?
            .to_pretty_fmt(),
            invisibleAnnotation_class_output
        );
        assert_eq!(
            load_class(&(TEST_PATH.to_string() + "annotations_java_test/visibleAnnotation.class"))?
                .to_pretty_fmt(),
            visibleAnnotation_class_output
        );
        Ok(())
    }
}
