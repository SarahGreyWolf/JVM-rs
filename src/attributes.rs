use std::{io::Cursor, error::Error};

use byteorder::{ReadBytesExt, BE};

use crate::class_file::{AttributeInfo, ConstantPool};

#[derive(Clone, Debug)]
struct ExceptionTable {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16
}

impl ExceptionTable {
    pub fn new(start_pc: u16, end_pc: u16, handler_pc: u16, catch_type: u16) -> ExceptionTable {
        ExceptionTable {
            start_pc,
            end_pc,
            handler_pc,
            catch_type
        }
    }
}

/**
 * Common values:
 * attribute_name_index: u16,
 * attribute_length: u32
 */

 /// [Constant Value](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A2771%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C390%2Cnull%5D)
#[derive(Clone, Debug)]
pub struct ConstantValue(
    u16,
    u32,
    /**
     * **constantvalue_index**\
     *  The value of the constantvalue_index item must be a valid index into the
     *  constant_pool table. The constant_pool entry at that index gives the value
     *  represented by this attribute. The constant_pool entry must be of a type
     *  appropriate to the field, as specified in Table 4.7.2-A.
     */
    u16
);

impl ConstantValue {
    pub fn new(attribute_name_index: u16, attribute_length:u32,  constantvalue_index: u16) -> ConstantValue {
        ConstantValue(
            attribute_name_index,
            attribute_length,
            constantvalue_index
        )
    }
}

/// [Code](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A793%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C275%2Cnull%5D)
#[derive(Clone, Debug)]
pub struct Code(
    u16,
    u32,
    /**
     * **max_stack**\
     *  The value of the max_stack item gives the maximum depth of the operand
     *  stack of this method (§2.6.2) at any point during execution of the method
     */
    u16,
    /**
     * **max_locals**\
     *  The value of the max_locals item gives the number of local variables in
     *  the local variable array allocated upon invocation of this method (§2.6.1),
     *  including the local variables used to pass parameters to the method on its
     *  invocation.
     * 
     *  The greatest local variable index for a value of type long or double is
     *  max_locals - 2. The greatest local variable index for a value of any other
     *  type is max_locals - 1.
     */
    u16,
    u32,
    Vec<u8>,
    u16,
    Vec<ExceptionTable>,
    u16,
    Vec<AttributeInfo>
);


impl Code {
    pub fn new(attribute_name_index: u16, attribute_length: u32, constant_pool: &Vec<ConstantPool>, cursor: &mut Cursor<&[u8]>) -> Result<Code, Box<dyn Error>> {
        let max_stack = cursor.read_u16::<BE>()?;
        let max_locals = cursor.read_u16::<BE>()?;
        let code_length = cursor.read_u32::<BE>()?;
        let mut code: Vec<u8> = Vec::with_capacity(code_length as usize);
        for _ in 0..code_length as usize {
            code.push(cursor.read_u8()?);
        }
        let exception_table_length = cursor.read_u16::<BE>()?;
        let mut exception_table: Vec<ExceptionTable> = Vec::with_capacity(exception_table_length as usize);
        for _ in 0..exception_table_length as usize {
            exception_table.push(
                ExceptionTable::new(
                    cursor.read_u16::<BE>()?,
                     cursor.read_u16::<BE>()?,
                      cursor.read_u16::<BE>()?,
                       cursor.read_u16::<BE>()?
                )
            );
        }
        let attributes_count = cursor.read_u16::<BE>()?;
        let mut attributes = Vec::with_capacity(attributes_count as usize);
        crate::class_file::read_attributes(constant_pool, &mut attributes, cursor);
        Ok(Code(
            attribute_name_index,
            attribute_length,
            max_stack,
            max_locals,
            code_length,
            code,
            exception_table_length,
            exception_table,
            attributes_count,
            attributes
        ))
    }
}

#[derive(Clone, Debug)]
pub struct StackMapTable(
    u16,
    u32
);

#[derive(Clone, Debug)]
pub struct Exceptions(
    u16,
    u32
);

#[derive(Clone, Debug)]
pub struct InnerClasses(
    u16,
    u32
);

#[derive(Clone, Debug)]
pub struct EnclosingMethod(
    u16,
    u32
);

#[derive(Clone, Debug)]
pub struct Synthetic(
    u16,
    u32
);

#[derive(Clone, Debug)]
pub struct Signature(
    u16,
    u32
);

#[derive(Clone, Debug)]
pub struct SourceFile(
    u16,
    u32
);

#[derive(Clone, Debug)]
pub struct SourceDebugExtension(
    u16,
    u32
);

#[derive(Clone, Debug)]
struct LineNumberTableContents {
    start_pc: u16,
    line_number: u16
}

impl LineNumberTableContents {
    pub fn new(start_pc: u16, line_number: u16) -> LineNumberTableContents {
        LineNumberTableContents{
            start_pc,
            line_number
        }
    }
}

#[derive(Clone, Debug)]
pub struct LineNumberTable {
    attribute_name_index: u16,
    attribute_length: u32,
    line_number_table_length: u16,
    line_number_table: Vec<LineNumberTableContents>,
}

impl LineNumberTable {
    pub fn new(attribute_name_index: u16, attribute_length: u32, line_number_table_length: u16, cursor: &mut Cursor<&[u8]>) -> Result<LineNumberTable, Box<dyn Error>> {
        Ok(LineNumberTable {
            attribute_name_index,
            attribute_length,
            line_number_table_length,
            line_number_table: {
                let mut table_contents = Vec::with_capacity(line_number_table_length as usize);
                for _ in 0..table_contents.capacity() {
                    table_contents.push(
                        LineNumberTableContents::new(cursor.read_u16::<BE>()?, cursor.read_u16::<BE>()?)
                    );
                }
                table_contents
            },
        })
    }
}

#[derive(Clone, Debug)]
pub struct LocalVariableTable(
    u16,
    u32
);

#[derive(Clone, Debug)]
pub struct LocalVariableTypeTable(
    u16,
    u32
);

#[derive(Clone, Debug)]
pub struct Deprecated(
    u16,
    u32
);

#[derive(Clone, Debug)]
pub struct RuntimeVisibleAnnotations(
    u16,
    u32
);

#[derive(Clone, Debug)]
pub struct RuntimeInvisibleAnnotations(
    u16,
    u32
);

#[derive(Clone, Debug)]
pub struct RuntimeVisibleParameterAnnotations(
    u16,
    u32
);

#[derive(Clone, Debug)]
pub struct RuntimeInvisibleParameterAnnotations(
    u16,
    u32
);

#[derive(Clone, Debug)]
pub struct RuntimeVisibleTypeAnnotations(
    u16,
    u32
);

#[derive(Clone, Debug)]
pub struct RuntimeInvisibleTypeAnnotations(
    u16,
    u32
);

#[derive(Clone, Debug)]
pub struct AnnotationDefault(
    u16,
    u32
);

#[derive(Clone, Debug)]
pub struct BootstrapMethods(
    u16,
    u32
);

#[derive(Clone, Debug)]
pub struct MethodParameters(
    u16,
    u32
);

#[derive(Clone, Debug)]
pub struct Module(
    u16,
    u32
);

#[derive(Clone, Debug)]
pub struct ModulePackages(
    u16,
    u32
);

#[derive(Clone, Debug)]
pub struct ModuleMainClass(
    u16,
    u32
);

#[derive(Clone, Debug)]
pub struct NestHost(
    u16,
    u32
);

#[derive(Clone, Debug)]
pub struct NestMembers(
    u16,
    u32
);

#[derive(Clone, Debug)]
pub struct Record(
    u16,
    u32
);

#[derive(Clone, Debug)]
pub struct PermittedSubclasses(
    u16,
    u32
);

#[derive(Clone, Debug)]
pub struct Unknown(
    u16,
    u32
);