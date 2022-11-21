#![allow(dead_code)]

use core::num;
use std::{error::Error, io::Cursor};

use byteorder::{ReadBytesExt, BE};

use crate::access_flags::{ModuleFlags, ParameterAccessFlags};
use crate::class_file::{AttributeInfo, ConstantPool};

use crate::errors::class_loading::{LoadingCause, LoadingError};

/*
 * Common values:\
 * **attribute_name_index**\
 *  The value of the attribute_name_index item must be a valid index
 *  into the constant_pool table. The constant_pool entry at that index
 *  must be a CONSTANT_Utf8_info structure (§4.4.7).\
 * attribute_name_index: u16,\
 * **attribute_length**\
 *  The value of the attribute_length item indicates the length of the attribute,
 *  excluding the initial six bytes.\
 * attribute_length: u32
 */

#[derive(Clone, Debug)]
pub struct ExceptionTable {
    /**
     * **start_pc**\
     *  start_pc, end_pc
     *  The values of the two items start_pc and end_pc indicate the ranges in the
     *  code array at which the exception handler is active. The value of start_pc
     *  must be a valid index into the code array of the opcode of an instruction.
     *  The value of end_pc either must be a valid index into the code array of the
     *  opcode of an instruction or must be equal to code_length, the length of the
     *  code array. The value of start_pc must be less than the value of end_pc.
     *  The start_pc is inclusive and end_pc is exclusive; that is, the exception
     *  handler must be active while the program counter is within the interval
     *  [start_pc, end_pc).
     *  The fact that end_pc is exclusive is a historical mistake in the design of the Java
     *  Virtual Machine: if the Java Virtual Machine code for a method is exactly 65535 bytes
     *  long and ends with an instruction that is 1 byte long, then that instruction cannot be
     *  protected by an exception handler. A compiler writer can work around this bug by
     *  limiting the maximum size of the generated Java Virtual Machine code for any method,
     *  instance initialization method, or static initializer (the size of any code array) to 65534
     *  bytes.
     */
    start_pc: u16,
    /// **end_pc**
    end_pc: u16,
    /**
     * **handler_pc**\
     *  The value of the handler_pc item indicates the start of the exception
     *  handler. The value of the item must be a valid index into the code array
     *  and must be the index of the opcode of an instruction.
     */
    handler_pc: u16,
    /**
     * **catch_type**\
     *  If the value of the catch_type item is nonzero, it must be a valid index
     *  into the constant_pool table. The constant_pool entry at that index
     *  must be a CONSTANT_Class_info structure (§4.4.1) representing a class of
     *  exceptions that this exception handler is designated to catch. The exception
     *  handler will be called only if the thrown exception is an instance of the
     *  given class or one of its subclasses.
     *
     *  The verifier checks that the class is Throwable or a subclass of Throwable (§4.9.2).
     *  If the value of the catch_type item is zero, this exception handler is called
     *  for all exceptions.
     *
     *  This is used to implement finally (§3.13).
     */
    catch_type: u16,
}

impl ExceptionTable {
    pub fn new(start_pc: u16, end_pc: u16, handler_pc: u16, catch_type: u16) -> ExceptionTable {
        ExceptionTable {
            start_pc,
            end_pc,
            handler_pc,
            catch_type,
        }
    }
}

/// [Constant Value](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A2771%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C390%2Cnull%5D)
#[derive(Clone, Debug)]
pub struct ConstantValue {
    attribute_name_index: u16,
    attribute_length: u32,
    /**
     * **constantvalue_index**\
     *  The value of the constantvalue_index item must be a valid index into the
     *  constant_pool table. The constant_pool entry at that index gives the value
     *  represented by this attribute. The constant_pool entry must be of a type
     *  appropriate to the field, as specified in Table 4.7.2-A.
     */
    pub(crate) constantvalue_index: u16,
}

impl ConstantValue {
    pub fn new(
        attribute_name_index: u16,
        attribute_length: u32,
        constantvalue_index: u16,
    ) -> ConstantValue {
        ConstantValue {
            attribute_name_index,
            attribute_length,
            constantvalue_index,
        }
    }
}

#[derive(Clone, Debug)]
/// [Code](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A793%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C275%2Cnull%5D)\
/**
 * In a class file whose version number is 50.0 or above, if a method's Code attribute
 * does not have a StackMapTable attribute, it has an implicit stack map attribute
 * (§4.10.1). This implicit stack map attribute is equivalent to a StackMapTable
 * attribute with number_of_entries equal to zero.
 */
pub struct Code {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    /**
     * **max_stack**\
     *  The value of the max_stack item gives the maximum depth of the operand
     *  stack of this method (§2.6.2) at any point during execution of the method
     */
    pub max_stack: u16,
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
    pub max_locals: u16,
    /**
     * **code_length**\
     *  The value of the code_length item gives the number of bytes in the code array
     *  for this method.\
     *  The value of code_length must be greater than zero (as the code array must
     *  not be empty) and less than 65536.
     */
    pub code_length: u32,
    /**
     * **code**\
     *  The code array gives the actual bytes of Java Virtual Machine code that
     *  implement the method.
     *
     *  When the code array is read into memory on a byte-addressable machine, if
     *  the first byte of the array is aligned on a 4-byte boundary, the tableswitch and
     *  lookupswitch 32-bit offsets will be 4-byte aligned. (Refer to the descriptions
     *  of those instructions for more information on the consequences of code array
     *  alignment.)
     *
     *  The detailed constraints on the contents of the code array are extensive and are
     *  given in a separate section (§4.9).
     */
    pub code: Vec<u8>,
    /**
     * **exception_table_length**\
     *  The value of the exception_table_length item gives the number of entries
     *  in the exception_table array.
     */
    pub exception_table_length: u16,
    /**
     * **exception_table**\
     *  Each entry in the exception_table array describes one exception handler in
     *  the code array. The order of the handlers in the [exception_table](ExceptionTable) array is
     *  significant (§2.10).\
     */
    pub exception_tables: Vec<ExceptionTable>,
    /**
     * **attributes_count**\
     *  The value of the attributes_count item indicates the number of attributes of
     *  the Code attribute.
     */
    pub attributes_count: u16,
    /**
     * **attributes**\
     *  Each value of the attributes table must be an [attribute_info](AttributeInfo) structure
     *  (§4.7).
     *
     *  A Code attribute can have any number of optional attributes associated with it.
     *  The attributes defined by this specification as appearing in the attributes
     *  table of a Code attribute are listed in Table 4.7-C.
     *
     *  The rules concerning attributes defined to appear in the attributes table of
     *  a Code attribute are given in §4.7.
     *
     *  The rules concerning non-predefined attributes in the attributes table of a
     *  Code attribute are given in §4.7.1.
     */
    pub attributes: Vec<crate::class_file::AttributeInfo>,
}

impl Code {
    pub fn new(
        attribute_name_index: u16,
        attribute_length: u32,
        constant_pool: &[ConstantPool],
        cursor: &mut Cursor<&[u8]>,
        version: u16,
    ) -> Result<Code, Box<dyn Error>> {
        let max_stack = cursor.read_u16::<BE>()?;
        let max_locals = cursor.read_u16::<BE>()?;
        let code_length = cursor.read_u32::<BE>()?;
        let mut code: Vec<u8> = Vec::with_capacity(code_length as usize);
        for _ in 0..code_length as usize {
            code.push(cursor.read_u8()?);
        }
        let exception_table_length = cursor.read_u16::<BE>()?;
        let mut exception_tables: Vec<ExceptionTable> =
            Vec::with_capacity(exception_table_length as usize);
        for _ in 0..exception_table_length as usize {
            exception_tables.push(ExceptionTable::new(
                cursor.read_u16::<BE>()?,
                cursor.read_u16::<BE>()?,
                cursor.read_u16::<BE>()?,
                cursor.read_u16::<BE>()?,
            ));
        }
        let mut attributes_count = cursor.read_u16::<BE>()?;
        let mut attributes = Vec::with_capacity(attributes_count as usize);
        read_attributes(constant_pool, &mut attributes, cursor, None)?;
        if version >= 50 {
            let mut has_stackmap = false;
            for attrib in &attributes {
                if let AttributeInfo::StackMapTable(_stack_map) = attrib {
                    has_stackmap = true;
                    break;
                }
            }
            if !has_stackmap {
                attributes.push(AttributeInfo::StackMapTable(StackMapTable::implicit(
                    constant_pool.len() as u16,
                )));
                attributes_count += 1;
            }
        }
        Ok(Code {
            attribute_name_index,
            attribute_length,
            max_stack,
            max_locals,
            code_length,
            code,
            exception_table_length,
            exception_tables,
            attributes_count,
            attributes,
        })
    }
}

/// [VerificationTypeInfo](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#page=129)
/**
 * The Long_variable_info and Double_variable_info items indicate the
 * verification type of the second of two locations as follows:
 * 1. If the first of the two locations is a local variable, then:
 *  - It must not be the local variable with the highest index.
 *  - The next higher numbered local variable has the verification type top.
 * 2. If the first of the two locations is an operand stack entry, then:
 *  - It must not be the topmost location of the operand stack.
 *  - The next location closer to the top of the operand stack has the verification
 * type top.
 */
#[derive(Clone, Debug)]
enum VerificationTypeInfo {
    TopVariable,
    IntegerVariable,
    FloatVariable,
    /**
     * **Long_variable_info**\
     *  The Long_variable_info item indicates that the first of two locations has the
     *  verification type long.
     */
    LongVariable,
    /**
     * **Double_variable_info**\
     *  The Double_variable_info item indicates that the first of two locations has the
     *  verification type double.
     */
    DoubleVariable,
    NullVariable,
    UnitializedThisVariable,
    /**
     * **object_variable_info**\
     *  The Object_variable_info item indicates that the location has the verification
     *  type which is the class represented by the CONSTANT_Class_info structure
     *  (§4.4.1) found in the constant_pool table at the index given by cpool_index.
     */
    ObjectVariable(u16),
    /**
     * **unitialized_variable_info**\
     *  he Uninitialized_variable_info item indicates that the location has the
     *  verification type uninitialized(Offset). The Offset item indicates the offset,
     *  in the code array of the Code attribute that contains this StackMapTable attribute,
     *  of the new instruction (§new) that created the object being stored in the location.
     */
    UnintializedVariable(u16),
}

impl VerificationTypeInfo {
    fn get_tag(&self) -> u8 {
        match self {
            VerificationTypeInfo::TopVariable => 0,
            VerificationTypeInfo::IntegerVariable => 1,
            VerificationTypeInfo::FloatVariable => 2,
            VerificationTypeInfo::DoubleVariable => 3,
            VerificationTypeInfo::LongVariable => 4,
            VerificationTypeInfo::NullVariable => 5,
            VerificationTypeInfo::UnitializedThisVariable => 6,
            VerificationTypeInfo::ObjectVariable(_) => 7,
            VerificationTypeInfo::UnintializedVariable(_) => 8,
        }
    }

    pub fn from_byte(tag: u8, data: Option<u16>) -> VerificationTypeInfo {
        match tag {
            0 => VerificationTypeInfo::TopVariable,
            1 => VerificationTypeInfo::IntegerVariable,
            2 => VerificationTypeInfo::FloatVariable,
            3 => VerificationTypeInfo::DoubleVariable,
            4 => VerificationTypeInfo::LongVariable,
            5 => VerificationTypeInfo::NullVariable,
            6 => VerificationTypeInfo::UnitializedThisVariable,
            7 => {
                if let Some(value) = data {
                    VerificationTypeInfo::ObjectVariable(value)
                } else {
                    unreachable!("Invalid Verification Type: {}", tag)
                }
            }
            8 => {
                if let Some(value) = data {
                    VerificationTypeInfo::UnintializedVariable(value)
                } else {
                    unreachable!("Invalid Verification Type: {}", tag)
                }
            }
            _ => unreachable!("Invalid Verification Type: {}", tag),
        }
    }
}

/// [StackMapFrame](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#page=131)
#[derive(Clone, Debug)]
enum StackMapFrame {
    /**
     * **stack_frame**\
     *  The frame type same_frame is represented by tags in the range \[0-63\]. This frame
     *  type indicates that the frame has exactly the same local variables as the previous
     *  frame and that the operand stack is empty. The offset_delta value for the frame
     *  is the value of the tag item, frame_type.
     */
    SameFrame {
        /// 0-63
        frame_type: u8,
    },
    /**
     * **same_locals_1_stack_item_frame**\
     *  The frame type same_locals_1_stack_item_frame is represented by tags in
     *  the range \[64, 127\]. This frame type indicates that the frame has exactly the same
     *  local variables as the previous frame and that the operand stack has one entry.
     *  The offset_delta value for the frame is given by the formula frame_type -
     *  64. The verification type of the one stack entry appears after the frame type.
     */
    SameLocals1StackItemFrame {
        /// 64-127
        frame_type: u8,
        /**
         * Assume the verification types of local variables are
         * given by locals, an array structured as in the full_frame frame type. If
         * locals\[M-1\] in the previous frame represented local variable X and locals\[M\]
         * represented local variable Y, then the effect of removing one local variable is
         * that locals\[M-1\] in the new frame represents local variable X and locals\[M\] is undefined.
         * It is an error if k is larger than the number of local variables in locals for the
         * previous frame, that is, if the number of local variables in the new frame would
         * be less than zero.
         */
        stack: Vec<VerificationTypeInfo>, // [1]
    },
    // Tags in the range [128-246] are reserved for future use.
    /**
     * **same_locals_1_stack_item_frame_extended**\
     *  The frame type same_locals_1_stack_item_frame_extended is represented
     *  by the tag 247. This frame type indicates that the frame has exactly the same
     *  local variables as the previous frame and that the operand stack has one entry.
     *  The offset_delta value for the frame is given explicitly, unlike in the frame
     *  type same_locals_1_stack_item_frame. The verification type of the one stack
     *  entry appears after offset_delta.
     */
    SameLocals1StackItemFrameExtended {
        /// 247
        frame_type: u8,
        offset_delta: u16,
        /**
         * Assume the verification types of local variables are
         * given by locals, an array structured as in the full_frame frame type. If
         * locals\[M-1\] in the previous frame represented local variable X and locals\[M\]
         * represented local variable Y, then the effect of removing one local variable is
         * that locals\[M-1\] in the new frame represents local variable X and locals\[M\] is undefined.
         * It is an error if k is larger than the number of local variables in locals for the
         * previous frame, that is, if the number of local variables in the new frame would
         * be less than zero.
         */
        stack: Vec<VerificationTypeInfo>, // [1]
    },
    /**
     * **chop_frame**\
     *  The frame type chop_frame is represented by tags in the range \[248-250\]. This
     *  frame type indicates that the frame has the same local variables as the previous
     *  frame except that the last k local variables are absent, and that the operand stack
     *  is empty. The value of k is given by the formula 251 - frame_type. The
     *  offset_delta value for the frame is given explicitly.
     */
    ChopFrame {
        /// 248-250
        frame_type: u8,
        offset_delta: u16,
    },
    /**
     * **same_frame_extended**\
     *  The frame type same_frame_extended is represented by the tag 251. This frame
     *  type indicates that the frame has exactly the same local variables as the previous
     *  frame and that the operand stack is empty. The offset_delta value for the frame
     *  is given explicitly, unlike in the frame type same_frame.
     */
    SameFrameExtended {
        /// 251
        frame_type: u8,
        offset_delta: u16,
    },
    /**
     * **append_frame**\
     *  The frame type append_frame is represented by tags in the range [252-254]. This
     *  frame type indicates that the frame has the same locals as the previous frame
     *  except that k additional locals are defined, and that the operand stack is empty.
     *  The value of k is given by the formula frame_type - 251. The offset_delta
     *  value for the frame is given explicitly.
     */
    AppendFrame {
        /// 252-254
        frame_type: u8,
        offset_delta: u16,
        /**
         * The 0th entry in locals represents the verification type of the first additional
         * local variable. If locals\[M\] represents local variable N, then:\
         * -  locals\[M+1\] represents local variable N+1 if locals\[M\] is one
         * of Top_variable_info, Integer_variable_info, Float_variable_info,
         * Null_variable_info, UninitializedThis_variable_info,
         * Object_variable_info, or Uninitialized_variable_info; and\
         * -  locals\[M+1\] represents local variable N+2 if locals\[M\] is either
         * Long_variable_info or Double_variable_info.
         * It is an error if, for any index i, locals\[i\] represents a local variable whose
         * index is greater than the maximum number of local variables for the method.
         */
        locals: Vec<VerificationTypeInfo>, // [frame_type - 251]
    },
    /**
     * **full_frame**\
     *  The frame type full_frame is represented by the tag 255. The offset_delta
     *  value for the frame is given explicitly.
     */
    FullFrame {
        /// 255
        frame_type: u8,
        offset_delta: u16,
        number_of_locals: u16,
        /**
         * The 0th entry in locals represents the verification type of the first additional
         * local variable. If locals\[M\] represents local variable N, then:\
         * -  locals\[M+1\] represents local variable N+1 if locals\[M\] is one
         * of Top_variable_info, Integer_variable_info, Float_variable_info,
         * Null_variable_info, UninitializedThis_variable_info,
         * Object_variable_info, or Uninitialized_variable_info; and\
         * -  locals\[M+1\] represents local variable N+2 if locals\[M\] is either
         * Long_variable_info or Double_variable_info.
         * It is an error if, for any index i, locals\[i\] represents a local variable whose
         * index is greater than the maximum number of local variables for the method.
         */
        locals: Vec<VerificationTypeInfo>, // [number_of_locals]
        number_of_stack_items: u16,
        /**
         * The 0th entry in stack represents the verification type of the bottom of the
         * operand stack, and subsequent entries in stack represent the verification types
         * of stack entries closer to the top of the operand stack. We refer to the bottom of
         * the operand stack as stack entry 0, and to subsequent entries of the operand stack
         * as stack entry 1, 2, etc. If stack\[M\] represents stack entry N, then:
         * –  stack\[M+1\] represents stack entry N+1 if stack\[M\] is one of
         * Top_variable_info, Integer_variable_info, Float_variable_info,
         * Null_variable_info, UninitializedThis_variable_info,
         * Object_variable_info, or Uninitialized_variable_info; and
         * –  stack\[M+1\] represents stack entry N+2 if stack\[M\] is either
         * Long_variable_info or Double_variable_info.
         * It is an error if, for any index i, stack\[i\] represents a stack entry whose index
         * is greater than the maximum operand stack size for the method.
         */
        stack: Vec<VerificationTypeInfo>, // [number_of_stack_items]
    },
    Unused {
        frame_type: u8,
    },
}

#[derive(Clone, Debug)]
/// [StackMapTable](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A1597%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C142%2Cnull%5D)
pub struct StackMapTable {
    attribute_name_index: u16,
    attribute_length: u32,
    /**
     * **number_of_entries**\
     *  The value of the number_of_entries item gives the number of
     *  stack_map_frame entries in the entries table
     *
     */
    number_of_entries: u16,
    /**
     * **entries**\
     *  Each entry in the entries table describes one stack map frame of the method.
     *  The order of the [stack map frames](StackMapFrame) in the entries table is significant.
     */
    entries: Vec<StackMapFrame>,
}

impl StackMapTable {
    pub fn new(
        attribute_name_index: u16,
        attribute_length: u32,
        cursor: &mut Cursor<&[u8]>,
    ) -> Result<StackMapTable, Box<dyn Error>> {
        let start = cursor.position();
        let entry_count = cursor.read_u16::<BE>()?;
        let stackmap = StackMapTable {
            attribute_name_index,
            attribute_length,
            number_of_entries: entry_count,
            entries: {
                let mut entries = Vec::with_capacity(entry_count as usize);
                // What am I even doing with this?
                let mut offset_delta: u16 = 0;
                for _ in 0..entry_count {
                    let _type = cursor.read_u8()?;
                    entries.push(match _type {
                        0..=63 => {
                            offset_delta = _type as u16;
                            StackMapFrame::SameFrame { frame_type: _type }
                        }
                        64..=127 => {
                            offset_delta = _type as u16 - 64;
                            let ver_tag = cursor.read_u8()?;
                            let data = if ver_tag == 7 || ver_tag == 8 {
                                Some(cursor.read_u16::<BE>()?)
                            } else {
                                None
                            };
                            StackMapFrame::SameLocals1StackItemFrame {
                                frame_type: _type,
                                stack: vec![VerificationTypeInfo::from_byte(ver_tag, data)],
                            }
                        }
                        247 => {
                            // offset_delta += 1;
                            let offset_delta = cursor.read_u16::<BE>()?;
                            StackMapFrame::SameLocals1StackItemFrameExtended {
                                frame_type: _type,
                                offset_delta,
                                stack: {
                                    let ver_tag = cursor.read_u8()?;
                                    let data = if ver_tag == 7 || ver_tag == 8 {
                                        Some(cursor.read_u16::<BE>()?)
                                    } else {
                                        None
                                    };
                                    vec![VerificationTypeInfo::from_byte(ver_tag, data)]
                                },
                            }
                        }
                        248..=250 => StackMapFrame::ChopFrame {
                            frame_type: _type,
                            offset_delta: cursor.read_u16::<BE>()?,
                        },
                        251 => StackMapFrame::SameFrameExtended {
                            frame_type: _type,
                            offset_delta: cursor.read_u16::<BE>()?,
                        },
                        252..=254 => {
                            // offset_delta += 1;
                            let offset_delta = cursor.read_u16::<BE>()?;
                            StackMapFrame::AppendFrame {
                                frame_type: _type,
                                offset_delta,
                                locals: {
                                    let mut locals = Vec::with_capacity((_type - 251) as usize);
                                    for _ in 0..locals.capacity() {
                                        let ver_tag = cursor.read_u8()?;
                                        let data = if ver_tag == 7 || ver_tag == 8 {
                                            Some(cursor.read_u16::<BE>()?)
                                        } else {
                                            None
                                        };
                                        locals.push(VerificationTypeInfo::from_byte(ver_tag, data));
                                    }
                                    locals
                                },
                            }
                        }
                        255 => {
                            // offset_delta += 1;
                            let offset_delta = cursor.read_u16::<BE>()?;
                            let number_of_locals = cursor.read_u16::<BE>()?;
                            let mut locals = Vec::with_capacity(number_of_locals as usize);
                            for _ in 0..locals.capacity() {
                                let ver_tag = cursor.read_u8()?;
                                let data = if ver_tag == 7 || ver_tag == 8 {
                                    Some(cursor.read_u16::<BE>()?)
                                } else {
                                    None
                                };
                                locals.push(VerificationTypeInfo::from_byte(ver_tag, data));
                            }
                            let number_of_stack_items = cursor.read_u16::<BE>()?;
                            let mut stack = Vec::with_capacity(number_of_stack_items as usize);
                            for _ in 0..stack.capacity() {
                                let ver_tag = cursor.read_u8()?;
                                let data = if ver_tag == 7 || ver_tag == 8 {
                                    Some(cursor.read_u16::<BE>()?)
                                } else {
                                    None
                                };
                                stack.push(VerificationTypeInfo::from_byte(ver_tag, data));
                            }
                            StackMapFrame::FullFrame {
                                frame_type: _type,
                                offset_delta,
                                number_of_locals,
                                locals,
                                number_of_stack_items,
                                stack,
                            }
                        }
                        _ => StackMapFrame::Unused { frame_type: _type },
                    });
                }
                entries
            },
        };
        if cursor.position() - start < attribute_length as u64 {
            unreachable!("Full Attribute Length was not correctly handled")
        }
        Ok(stackmap)
    }
    fn implicit(index: u16) -> StackMapTable {
        StackMapTable {
            attribute_name_index: index,
            attribute_length: 2,
            number_of_entries: 0,
            entries: vec![],
        }
    }
}

#[derive(Clone, Debug)]
/// [Exceptions](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A865%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C590%2Cnull%5D)
pub struct Exceptions {
    attribute_name_index: u16,
    attribute_length: u32,
    /**
     * *number_of_exceptions*\
     *  The value of the number_of_exceptions item indicates the number of entries
     *  in the exception_index_table.
     */
    number_of_exceptions: u16,
    /**
     * *exception_index_table*\
     *  Each value in the exception_index_table array must be a valid index into
     *  the constant_pool table. The constant_pool entry at that index must be a
     *  CONSTANT_Class_info structure (§4.4.1) representing a class type that this
     *  method is declared to throw.
     *  A method should throw an exception only if at least one of the following three criteria is
     *  met:
     *  - The exception is an instance of RuntimeException or one of its subclasses.
     *  - The exception is an instance of Error or one of its subclasses.
     *  - The exception is an instance of one of the exception classes specified in the exception_index_table just described, or one of their subclasses.
     *  These requirements are not enforced in the Java Virtual Machine; they are enforced only at compile time.
     */
    exception_index_table: Vec<u16>,
}

impl Exceptions {
    pub fn new(
        attribute_name_index: u16,
        attribute_length: u32,
        constant_pool: &[ConstantPool],
        cursor: &mut Cursor<&[u8]>,
    ) -> Result<Exceptions, Box<dyn Error>> {
        let exception_count = cursor.read_u16::<BE>()?;
        Ok(Exceptions {
            attribute_name_index,
            attribute_length,
            number_of_exceptions: exception_count,
            exception_index_table: {
                let mut exceptions = Vec::with_capacity(exception_count as usize);
                for _ in 0..exceptions.capacity() {
                    let index = cursor.read_u16::<BE>()?;
                    if let ConstantPool::Class(class) = &constant_pool[index as usize] {
                        // if let ConstantPool::Utf8(name) = &constant_pool[class.1 as usize] {
                        //     match name.get_string() {
                        //         ""
                        //         _ => unreachable!("Class was not a valid type")
                        //     }
                        // }
                        exceptions.push(index);
                    } else {
                        unreachable!("Index into ConstantPool was not a Class Constant");
                    }
                }
                exceptions
            },
        })
    }
}

/// [InnerClassInfo](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#page=137)
#[derive(Clone, Debug)]
pub struct InnerClassInfo {
    /**
     * *inner_class_info_index*\
     *  The value of the inner_class_info_index item must be a valid index into
     *  the constant_pool table. The constant_pool entry at that index must be
     *  a CONSTANT_Class_info structure representing C.
     */
    inner_class_info_index: u16,
    /**
     * *outer_class_info_index*\
     *  If C is not a member of a class or an interface - that is, if C is a top-level
     *  class or interface (JLS §7.6) or a local class (JLS §14.3) or an anonymous
     *  class (JLS §15.9.5) - then the value of the outer_class_info_index item
     *  must be zero.\
     *  Otherwise, the value of the outer_class_info_index item must be a valid
     *  index into the constant_pool table, and the entry at that index must be
     *  a CONSTANT_Class_info structure representing the class or interface of
     *  which C is a member. The value of the outer_class_info_index item
     *  must not equal the the value of the inner_class_info_index item.
     */
    outer_class_info_index: u16,
    /**
     * *inner_name_index*\
     *  If C is anonymous (JLS §15.9.5), the value of the inner_name_index item
     *  must be zero.\
     *  Otherwise, the value of the inner_name_index item must be a valid index
     *  into the constant_pool table, and the entry at that index must be a
     *  CONSTANT_Utf8_info structure that represents the original simple name of
     *  C, as given in the source code from which this class file was compiled.
     */
    inner_name_index: u16,
    /**
     * *inner_class_access_flags*\
     *  The value of the inner_class_access_flags item is a mask of flags used
     *  to denote access permissions to and properties of class or interface C as
     *  declared in the source code from which this class file was compiled. It is
     *  used by a compiler to recover the original information when source code
     *  is not available. The flags are specified in Table 4.7.6-A.
     */
    inner_class_access_flags: u16,
}

impl InnerClassInfo {
    pub fn new(
        inner_info: u16,
        outer_info: u16,
        inner_name: u16,
        inner_access: u16,
        constant_pool: &[ConstantPool],
    ) -> InnerClassInfo {
        if let ConstantPool::Class(_) = &constant_pool[inner_info as usize] {
        } else {
            unreachable!(
                "inner_class_info_index {} did not reference a class object",
                inner_info
            );
        }
        if let ConstantPool::Class(_) = &constant_pool[outer_info as usize] {
        } else {
            unreachable!(
                "outer_class_info_index {} did not reference a class object",
                inner_info
            );
        }
        if let ConstantPool::Utf8(_) = &constant_pool[inner_name as usize] {
        } else {
            unreachable!(
                "inner_name_index {} did not reference a utf8 object",
                inner_info
            );
        }
        InnerClassInfo {
            inner_class_info_index: inner_info,
            outer_class_info_index: outer_info,
            inner_name_index: inner_name,
            inner_class_access_flags: inner_access,
        }
    }
}

/// [InnerClasses](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A872%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C570%2Cnull%5D)
#[derive(Clone, Debug)]
pub struct InnerClasses {
    attribute_name_index: u16,
    attribute_length: u32,
    /**
     * *number_of_classes*\
     *  The value of the number_of_classes item indicates the number of entries in
     *  the classes array.
     */
    number_of_classes: u16,
    /**
     * *classes*\
     *  Every CONSTANT_Class_info entry in the constant_pool table which
     *  represents a class or interface C that is not a package member must have exactly
     *  one corresponding entry in the classes array.
     */
    classes: Vec<InnerClassInfo>,
}

impl InnerClasses {
    pub fn new(
        attribute_name_index: u16,
        attribute_length: u32,
        constant_pool: &[ConstantPool],
        cursor: &mut Cursor<&[u8]>,
    ) -> Result<InnerClasses, Box<dyn Error>> {
        let classes_size = cursor.read_u16::<BE>()?;
        let mut classes = Vec::with_capacity(classes_size as usize);
        for _ in 0..classes.capacity() {
            classes.push(InnerClassInfo::new(
                cursor.read_u16::<BE>()?,
                cursor.read_u16::<BE>()?,
                cursor.read_u16::<BE>()?,
                cursor.read_u16::<BE>()?,
                constant_pool,
            ));
        }
        Ok(InnerClasses {
            attribute_name_index,
            attribute_length,
            number_of_classes: classes_size,
            classes,
        })
    }
}

#[derive(Clone, Debug)]
/// [EnclosingMethod](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A874%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C199%2Cnull%5D)
pub struct EnclosingMethod {
    attribute_name_index: u16,
    attribute_length: u32,
    /**
     * *class_index*\
     *  The value of the class_index item must be a valid index into the
     *  constant_pool table. The constant_pool entry at that index must be a
     *  CONSTANT_Class_info structure (§4.4.1) representing the innermost class that
     *  encloses the declaration of the current class.
     */
    class_index: u16,
    /**
     * *method_index*\
     *  If the current class is not immediately enclosed by a method or constructor,
     *  then the value of the method_index item must be zero.\
     *  In particular, method_index must be zero if the current class was immediately enclosed
     *  in source code by an instance initializer, static initializer, instance variable initializer, or
     *  class variable initializer. (The first two concern both local classes and anonymous classes,
     *  while the last two concern anonymous classes declared on the right hand side of a field
     *  assignment.)\
     *  Otherwise, the value of the method_index item must be a valid index into
     *  the constant_pool table. The constant_pool entry at that index must be a
     *  CONSTANT_NameAndType_info structure (§4.4.6) representing the name and
     *  type of a method in the class referenced by the class_index attribute above.\
     *  It is the responsibility of a Java compiler to ensure that the method identified via the
     *  method_index is indeed the closest lexically enclosing method of the class that contains
     *  this EnclosingMethod attribute.
     */
    method_index: u16,
}

impl EnclosingMethod {
    pub fn new(
        attribute_name_index: u16,
        attribute_length: u32,
        class_index: u16,
        method_index: u16,
    ) -> Result<EnclosingMethod, Box<dyn Error>> {
        Ok(EnclosingMethod {
            attribute_name_index,
            attribute_length,
            class_index,
            method_index,
        })
    }
}

#[derive(Clone, Debug)]
/// [Synthetic](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A1185%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C590%2Cnull%5D)
pub struct Synthetic {
    attribute_name_index: u16,
    attribute_length: u32,
}

impl Synthetic {
    pub fn new(
        attribute_name_index: u16,
        attribute_length: u32,
    ) -> Result<Synthetic, Box<dyn Error>> {
        Ok(Synthetic {
            attribute_name_index,
            attribute_length,
        })
    }
}

#[derive(Clone, Debug)]
/// [Signature](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A1272%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C590%2Cnull%5D)
pub struct Signature {
    attribute_name_index: u16,
    attribute_length: u32,
    /**
     * *signature_index*\
     *  The value of the signature_index item must be a valid index into the
     *  constant_pool table. The constant_pool entry at that index must be a
     *  CONSTANT_Utf8_info structure (§4.4.7) representing a class signature if this
     *  Signature attribute is an attribute of a ClassFile structure; a method
     *  signature if this Signature attribute is an attribute of a method_info structure;
     *  or a field signature otherwise.
     */
    signature_index: u16,
}

impl Signature {
    pub fn new(
        attribute_name_index: u16,
        attribute_length: u32,
        signature_index: u16,
    ) -> Result<Signature, Box<dyn Error>> {
        Ok(Signature {
            attribute_name_index,
            attribute_length,
            signature_index,
        })
    }
}

#[derive(Clone, Debug)]
/// [SourceFile](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A1069%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C403%2Cnull%5D)
pub struct SourceFile {
    attribute_name_index: u16,
    attribute_length: u32,
    /**
     * **sourcefile_index**\
     *  The value of the sourcefile_index item must be a valid index into the
     *  constant_pool table. The constant_pool entry at that index must be a
     *  CONSTANT_Utf8_info structure representing a string.\
     *  The string referenced by the sourcefile_index item will be interpreted as indicating the
     *  name of the source file from which this class file was compiled. It will not be interpreted
     *  as indicating the name of a directory containing the file or an absolute path name for the file;
     *  such platform-specific additional information must be supplied by the run-time interpreter
     *  or development tool at the time the file name is actually used.
     */
    pub(crate) sourcefile_index: u16,
}

impl SourceFile {
    pub fn new(
        attribute_name_index: u16,
        attribute_length: u32,
        sourcefile_index: u16,
    ) -> Result<SourceFile, Box<dyn Error>> {
        Ok(SourceFile {
            attribute_name_index,
            attribute_length,
            sourcefile_index,
        })
    }
}

#[derive(Clone, Debug)]
/// [SourceDebugExtension](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A985%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C475%2Cnull%5D)
pub struct SourceDebugExtension {
    attribute_name_index: u16,
    attribute_length: u32,
    /**
     * *debug_extension*\
     *  The debug_extension array holds extended debugging information which has
     *  no semantic effect on the Java Virtual Machine. The information is represented
     *  using a modified UTF-8 string (§4.4.7) with no terminating zero byte.
     *  Note that the debug_extension array may denote a string longer than that which can be
     *  represented with an instance of class String.
     */
    debug_extension: String,
}

impl SourceDebugExtension {
    pub fn new(
        attribute_name_index: u16,
        attribute_length: u32,
        cursor: &mut Cursor<&[u8]>,
    ) -> Result<SourceDebugExtension, Box<dyn Error>> {
        let mut characters: Vec<u8> = Vec::with_capacity(attribute_length as usize);
        for _ in 0..attribute_length {
            characters.push(cursor.read_u8()?);
        }
        Ok(SourceDebugExtension {
            attribute_name_index,
            attribute_length,
            debug_extension: String::from_utf8(characters)?,
        })
    }
}

#[derive(Clone, Debug)]
struct LineNumber {
    start_pc: u16,
    line_number: u16,
}

impl LineNumber {
    pub fn new(start_pc: u16, line_number: u16) -> LineNumber {
        LineNumber {
            /**
             * **start_pc**\
             *  The value of the start_pc item must be a valid index into the code array
             *  of this Code attribute. The item indicates the index into the code array at
             *  which the code for a new line in the original source file begins.
             */
            start_pc,
            /**
             * **line_number**\
             *  The value of the line_number item gives the corresponding line number
             *  in the original source file.
             */
            line_number,
        }
    }
}

#[derive(Clone, Debug)]
/// [LineNumberTable](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A991%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C590%2Cnull%5D)
pub struct LineNumberTable {
    attribute_name_index: u16,
    attribute_length: u32,
    /**
     * **line_number_table_length**\
     *  The value of the line_number_table_length item indicates the number of
     *  entries in the line_number_table array.
     */
    line_number_table_length: u16,
    /**
     * **line_number_table**\
     *  Each entry in the [line_number_table](LineNumberTableContents) array indicates that the line number
     *  in the original source file changes at a given point in the code array.
     */
    line_number_table: Vec<LineNumber>,
}

impl LineNumberTable {
    pub fn new(
        attribute_name_index: u16,
        attribute_length: u32,
        line_number_table_length: u16,
        cursor: &mut Cursor<&[u8]>,
    ) -> Result<LineNumberTable, Box<dyn Error>> {
        Ok(LineNumberTable {
            attribute_name_index,
            attribute_length,
            line_number_table_length,
            line_number_table: {
                let mut table_contents = Vec::with_capacity(line_number_table_length as usize);
                for _ in 0..table_contents.capacity() {
                    table_contents.push(LineNumber::new(
                        cursor.read_u16::<BE>()?,
                        cursor.read_u16::<BE>()?,
                    ));
                }
                table_contents
            },
        })
    }
}

#[derive(Clone, Debug)]
struct LocalVariable {
    /**
     * *start_pc*\
     *  The value of the start_pc item must be a valid index into the code array
     *  of this Code attribute and must be the index of the opcode of an instruction.\
     *  The value of start_pc + length must either be a valid index into the code
     *  array of this Code attribute and be the index of the opcode of an instruction,
     *  or it must be the first index beyond the end of that code array.\
     *  The start_pc and length items indicate that the given local variable has
     *  a value at indices into the code array in the interval [start_pc, start_pc
     *  + length), that is, between start_pc inclusive and start_pc + length
     *  exclusive.
     */
    start_pc: u16,
    length: u16,
    /**
     * *name_index*\
     *  The value of the name_index item must be a valid index into the
     *  constant_pool table. The constant_pool entry at that index must contain
     *  a CONSTANT_Utf8_info structure representing a valid unqualified name
     *  denoting a local variable (§4.2.2).
     */
    name_index: u16,
    /**
     * *descriptor_index*\
     *  The value of the descriptor_index item must be a valid index into the
     *  constant_pool table. The constant_pool entry at that index must contain
     *  a CONSTANT_Utf8_info structure representing a field descriptor which
     *  encodes the type of a local variable in the source program (§4.3.2).
     */
    descriptor_index: u16,
    /**
     * *index*\
     *  The value of the index item must be a valid index into the local variable
     *  array of the current frame. The given local variable is at index in the local
     *  variable array of the current frame.\
     *  If the given local variable is of type double or long, it occupies both index
     *  and index + 1.
     */
    index: u16,
}

impl LocalVariable {
    pub fn new(
        start_pc: u16,
        length: u16,
        name_index: u16,
        descriptor_index: u16,
        index: u16,
    ) -> Result<LocalVariable, Box<dyn Error>> {
        Ok(LocalVariable {
            start_pc,
            length,
            name_index,
            descriptor_index,
            index,
        })
    }
}

#[derive(Clone, Debug)]
/// [LocalVariableTable](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A997%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C490%2Cnull%5D)
pub struct LocalVariableTable {
    attribute_name_index: u16,
    attribute_length: u32,
    /**
     * *local_variable_table_length*\
     *  The value of the local_variable_table_length item indicates the number
     *  of entries in the local_variable_table array.
     */
    local_variable_table_length: u16,
    /**
     * *local_variable_table*\
     *  Each entry in the local_variable_table array indicates a range of code array
     *  offsets within which a local variable has a value, and indicates the index into
     *  the local variable array of the current frame at which that local variable can be
     *  found.
     */
    local_variable_table: Vec<LocalVariable>,
}

impl LocalVariableTable {
    pub fn new(
        attribute_name_index: u16,
        attribute_length: u32,
        local_variable_table_length: u16,
        cursor: &mut Cursor<&[u8]>,
    ) -> Result<LocalVariableTable, Box<dyn Error>> {
        let mut local_variables = Vec::with_capacity(local_variable_table_length.into());
        for _ in 0..local_variable_table_length {
            local_variables.push(LocalVariable::new(
                cursor.read_u16::<BE>()?,
                cursor.read_u16::<BE>()?,
                cursor.read_u16::<BE>()?,
                cursor.read_u16::<BE>()?,
                cursor.read_u16::<BE>()?,
            )?);
        }
        Ok(LocalVariableTable {
            attribute_name_index,
            attribute_length,
            local_variable_table_length,
            local_variable_table: local_variables,
        })
    }
}

#[derive(Clone, Debug)]
struct LocalVariableType {
    /**
     * *start_pc*\
     *  The value of the start_pc item must be a valid index into the code array
     *  of this Code attribute and must be the index of the opcode of an instruction.\
     *  The value of start_pc + length must either be a valid index into the code
     *  array of this Code attribute and be the index of the opcode of an instruction,
     *  or it must be the first index beyond the end of that code array.\
     *  The start_pc and length items indicate that the given local variable has
     *  a value at indices into the code array in the interval [start_pc, start_pc
     *  + length), that is, between start_pc inclusive and start_pc + length
     *  exclusive.
     */
    start_pc: u16,
    length: u16,
    /**
     * *name_index*\
     *  The value of the name_index item must be a valid index into the
     *  constant_pool table. The constant_pool entry at that index must contain
     *  a CONSTANT_Utf8_info structure representing a valid unqualified name
     *  denoting a local variable (§4.2.2).
     */
    name_index: u16,
    /**
     * *signature_index*\
     *  The value of the signature_index item must be a valid index into the
     *  constant_pool table. The constant_pool entry at that index must contain
     *  a CONSTANT_Utf8_info structure representing a field signature which
     *  encodes the type of a local variable in the source program (§4.7.9.1).
     */
    signature_index: u16,
    /**
     * *index*\
     *  The value of the index item must be a valid index into the local variable
     *  array of the current frame. The given local variable is at index in the local
     *  variable array of the current frame.\
     *  If the given local variable is of type double or long, it occupies both index
     *  and index + 1.
     */
    index: u16,
}

impl LocalVariableType {
    pub fn new(
        start_pc: u16,
        length: u16,
        name_index: u16,
        signature_index: u16,
        index: u16,
    ) -> Result<LocalVariableType, Box<dyn Error>> {
        Ok(LocalVariableType {
            start_pc,
            length,
            name_index,
            signature_index,
            index,
        })
    }
}

#[derive(Clone, Debug)]
/// [LocalVariableTypeTable](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A1011%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C590%2Cnull%5D)
pub struct LocalVariableTypeTable {
    attribute_name_index: u16,
    attribute_length: u32,
    /**
     * *local_variable_type_table_length*\
     *  The value of the local_variable_type_table_length item indicates the
     *  number of entries in the local_variable_type_table array.
     */
    local_variable_type_table_length: u16,
    /**
     * *local_variable_type_table*\
     *  Each entry in the local_variable_type_table array indicates a range of code
     *  array offsets within which a local variable has a value, and indicates the index
     *  into the local variable array of the current frame at which that local variable
     *  can be found.
     */
    local_variable_type_table: Vec<LocalVariableType>,
}

impl LocalVariableTypeTable {
    pub fn new(
        attribute_name_index: u16,
        attribute_length: u32,
        local_variable_type_table_length: u16,
        cursor: &mut Cursor<&[u8]>,
    ) -> Result<LocalVariableTypeTable, Box<dyn Error>> {
        let mut local_variable_types = Vec::with_capacity(local_variable_type_table_length.into());
        for _ in 0..local_variable_type_table_length {
            local_variable_types.push(LocalVariableType::new(
                cursor.read_u16::<BE>()?,
                cursor.read_u16::<BE>()?,
                cursor.read_u16::<BE>()?,
                cursor.read_u16::<BE>()?,
                cursor.read_u16::<BE>()?,
            )?);
        }
        Ok(LocalVariableTypeTable {
            attribute_name_index,
            attribute_length,
            local_variable_type_table_length,
            local_variable_type_table: local_variable_types,
        })
    }
}

#[derive(Clone, Debug)]
/// [Deprecated](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A1021%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C136%2Cnull%5D)
pub struct Deprecated {
    attribute_name_index: u16,
    attribute_length: u32,
}

impl Deprecated {
    pub fn new(
        attribute_name_index: u16,
        attribute_length: u32,
    ) -> Result<Deprecated, Box<dyn Error>> {
        Ok(Deprecated {
            attribute_name_index,
            attribute_length,
        })
    }
}

#[derive(Clone, Debug)]
/// [ElementValueStructure](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A1041%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C367%2Cnull%5D)
enum Element {
    /**
     * *const_value_index*\
     *  The const_value_index item denotes a constant of either a primitive type or
     *  the type String as the value of this element-value pair.
     *  The value of the const_value_index item must be a valid index into the
     *  constant_pool table. The constant_pool entry at that index must be of a type
     *  appropriate to the tag item, as specified in the fourth column of Table 4.7.16.1-
     *  A.
     */
    ConstValueIndex(char, u16),
    /**
     * *enum_const_value*\
     *  The enum_const_value item denotes an enum constant as the value of this
     *  element-value pair.
     */
    EnumConstValue {
        /**
         * *type_name_index*\
         *  The value of the type_name_index item must be a valid index into the
         *  constant_pool table. The constant_pool entry at that index must be
         *  a CONSTANT_Utf8_info structure (§4.4.7) representing a field descriptor
         *  (§4.3.2). The constant_pool entry gives the internal form of the binary
         *  name of the type of the enum constant represented by this element_value
         *  structure (§4.2.1).
         */
        type_name_index: u16,
        /**
         * *const_name_index*\
         *  The value of the const_name_index item must be a valid index into the
         *  constant_pool table. The constant_pool entry at that index must be a
         *  CONSTANT_Utf8_info structure (§4.4.7). The constant_pool entry gives
         *  the simple name of the enum constant represented by this element_value
         *  structure.
         */
        const_name_index: u16,
    },
    /**
     * *class_info_index*\
     *  The class_info_index item denotes a class literal as the value of this element-
     *  value pair.\
     *  The class_info_index item must be a valid index into the constant_pool
     *  table. The constant_pool entry at that index must be a CONSTANT_Utf8_info
     *  structure (§4.4.7) representing a return descriptor (§4.3.3). The return
     *  descriptor gives the type corresponding to the class literal represented by this
     *  element_value structure. Types correspond to class literals as follows:\
     *  - For a class literal C.class, where C is the name of a class, interface,
     *  or array type, the corresponding type is C. The return descriptor in the
     *  constant_pool will be an ObjectType or an ArrayType.
     *  - For a class literal p.class, where p is the name of a primitive type, the
     *  corresponding type is p. The return descriptor in the constant_pool will be
     *  a BaseType character.
     *  - For a class literal void.class, the corresponding type is void. The return
     *  descriptor in the constant_pool will be V.\
     *  For example, the class literal Object.class corresponds to the type Object, so the
     *  constant_pool entry is Ljava/lang/Object;, whereas the class literal int.class
     *  corresponds to the type int, so the constant_pool entry is I.\
     *  The class literal void.class corresponds to void, so the constant_pool entry
     *  is V, whereas the class literal Void.class corresponds to the type Void, so the
     *  constant_pool entry is Ljava/lang/Void;.
     */
    ClassInfoIndex(u16),
    /**
     * *annotation_value*\
     *  The annotation_value item denotes a "nested" annotation as the value of this
     *  element-value pair.
     *  The value of the annotation_value item is an annotation structure (§4.7.16)
     *  that gives the annotation represented by this element_value structure.
     */
    Annotation(Annotation),
    /**
     * *array_value*\
     *  The array_value item denotes an array as the value of this element-value pair.
     */
    ArrayValue {
        /**
         * *num_values*\
         *  The value of the num_values item gives the number of elements in the
         *  array represented by this element_value structure.
         */
        num_values: u16,
        /**
         * *values*\
         *  Each value in the values table gives the corresponding element of the array
         *  represented by this element_value structure.
         */
        values: Vec<Self>,
    },
    Unknown(char),
}

impl Element {
    pub fn get_element(tag: u8, cursor: &mut Cursor<&[u8]>) -> Result<Element, Box<dyn Error>> {
        Ok(match tag as char {
            'B' | 'C' | 'D' | 'F' | 'I' | 'J' | 'S' | 'Z' | 's' => {
                Element::ConstValueIndex(tag as char, cursor.read_u16::<BE>()?)
            }
            'e' => Element::EnumConstValue {
                type_name_index: cursor.read_u16::<BE>()?,
                const_name_index: cursor.read_u16::<BE>()?,
            },
            'c' => Element::ClassInfoIndex(cursor.read_u16::<BE>()?),
            '@' => Element::Annotation(Annotation::new(
                cursor.read_u16::<BE>()?,
                cursor.read_u16::<BE>()?,
                cursor,
            )?),
            '[' => {
                let num_values = cursor.read_u16::<BE>()?;
                let mut values = Vec::with_capacity(num_values.into());
                for _ in 0..num_values {
                    values.push(Element::get_element(cursor.read_u8()?, cursor)?);
                }
                Element::ArrayValue { num_values, values }
            }
            _ => Element::Unknown(tag as char),
        })
    }
}

#[derive(Clone, Debug)]
struct ElementPairs {
    /**
     * *element_name_index*\
     *  The value of the element_name_index item must be a valid index into
     *  the constant_pool table. The constant_pool entry at that index must
     *  be a CONSTANT_Utf8_info structure (§4.4.7). The constant_pool
     *  entry denotes the name of the element of the element-value pair
     *  represented by this element_value_pairs entry.
     *  In other words, the entry denotes an element of the annotation interface specified
     *  by type_index.
     */
    element_name_index: u16,
    /**
     * *value*\
     *  The value of the value item represents the value of the element-value
     *  pair represented by this element_value_pairs entry.
     */
    value: Element,
}

impl ElementPairs {
    fn new(
        element_name_index: u16,
        cursor: &mut Cursor<&[u8]>,
    ) -> Result<ElementPairs, Box<dyn Error>> {
        Ok(ElementPairs {
            element_name_index,
            value: Element::get_element(cursor.read_u8()?, cursor)?,
        })
    }
}

#[derive(Clone, Debug)]
struct Annotation {
    /**
     * *type_index*\
     *  The value of the type_index item must be a valid index into the
     *  constant_pool table. The constant_pool entry at that index must be
     *  a CONSTANT_Utf8_info structure (§4.4.7) representing a field descriptor
     *  (§4.3.2). The field descriptor denotes the type of the annotation represented
     *  by this annotation structure.
     */
    type_index: u16,
    /**
     * *num_element_value_pairs*\
     *  The value of the num_element_value_pairs item gives the number of
     *  element-value pairs of the annotation represented by this annotation
     *  structure.
     */
    num_element_value_pairs: u16,
    /**
     * *element_value_pairs*\
     *  Each value of the element_value_pairs table represents a single element-
     *  value pair in the annotation represented by this annotation structure.
     */
    element_value_pairs: Vec<ElementPairs>,
}

impl Annotation {
    fn new(
        type_index: u16,
        num_element_value_pairs: u16,
        cursor: &mut Cursor<&[u8]>,
    ) -> Result<Annotation, Box<dyn Error>> {
        let mut element_value_pairs = Vec::with_capacity(num_element_value_pairs.into());
        for _ in 0..num_element_value_pairs {
            element_value_pairs.push(ElementPairs::new(cursor.read_u16::<BE>()?, cursor)?);
        }
        Ok(Annotation {
            type_index,
            num_element_value_pairs,
            element_value_pairs,
        })
    }
}

#[derive(Clone, Debug)]
/// [RuntimeVisibleAnnotations](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A1273%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C314.8%2Cnull%5D)
pub struct RuntimeVisibleAnnotations {
    attribute_name_index: u16,
    attribute_length: u32,
    /**
     * *num_annotations*\
     *  The value of the num_annotations item gives the number of run-time visible
     *  annotations represented by the structure.
     */
    num_annotations: u16,
    /**
     * *annotations*\
     *  Each entry in the annotations table represents a single run-time visible
     *  annotation on a declaration.
     */
    annotations: Vec<Annotation>,
}

impl RuntimeVisibleAnnotations {
    pub fn new(
        attribute_name_index: u16,
        attribute_length: u32,
        num_annotations: u16,
        cursor: &mut Cursor<&[u8]>,
    ) -> Result<RuntimeVisibleAnnotations, Box<dyn Error>> {
        let mut annotations = Vec::with_capacity(num_annotations.into());
        for _ in 0..num_annotations {
            annotations.push(Annotation::new(
                cursor.read_u16::<BE>()?,
                cursor.read_u16::<BE>()?,
                cursor,
            )?);
        }
        Ok(RuntimeVisibleAnnotations {
            attribute_name_index,
            attribute_length,
            num_annotations,
            annotations,
        })
    }
}

#[derive(Clone, Debug)]
/// [RuntimeInvisibleAnnotations](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A1312%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C590%2Cnull%5D)
pub struct RuntimeInvisibleAnnotations {
    attribute_name_index: u16,
    attribute_length: u32,
    /**
     * *num_annotations*\
     *  The value of the num_annotations item gives the number of run-time visible
     *  annotations represented by the structure.
     */
    num_annotations: u16,
    /**
     * *annotations*\
     *  Each entry in the annotations table represents a single run-time visible
     *  annotation on a declaration.
     */
    annotations: Vec<Annotation>,
}

impl RuntimeInvisibleAnnotations {
    pub fn new(
        attribute_name_index: u16,
        attribute_length: u32,
        num_annotations: u16,
        cursor: &mut Cursor<&[u8]>,
    ) -> Result<RuntimeInvisibleAnnotations, Box<dyn Error>> {
        let mut annotations = Vec::with_capacity(num_annotations.into());
        for _ in 0..num_annotations {
            annotations.push(Annotation::new(
                cursor.read_u16::<BE>()?,
                cursor.read_u16::<BE>()?,
                cursor,
            )?);
        }
        Ok(RuntimeInvisibleAnnotations {
            attribute_name_index,
            attribute_length,
            num_annotations,
            annotations,
        })
    }
}

#[derive(Clone, Debug)]
struct ParamAnnotation {
    /**
     * *num_annotations*\
     *  The value of the num_annotations item indicates the number of run-
     *  time visible annotations on the declaration of the formal parameter
     *  corresponding to the parameter_annotations entry.
     */
    num_annotations: u16,
    /**
     * *annotations[]*\
     *  Each entry in the annotations table represents a single run-time visible
     *  annotation on the declaration of the formal parameter corresponding to the
     *  parameter_annotations entry. The annotation structure is specified in
     *  §4.7.16.
     */
    annotations: Vec<Annotation>,
}

impl ParamAnnotation {
    fn new(
        num_annotations: u16,
        cursor: &mut Cursor<&[u8]>,
    ) -> Result<ParamAnnotation, Box<dyn Error>> {
        let mut annotations = Vec::with_capacity(num_annotations.into());
        for _ in 0..num_annotations {
            annotations.push(Annotation::new(
                cursor.read_u16::<BE>()?,
                cursor.read_u16::<BE>()?,
                cursor,
            )?);
        }
        Ok(ParamAnnotation {
            num_annotations,
            annotations,
        })
    }
}

#[derive(Clone, Debug)]
/// [RuntimeVisibleParameterAnnotations](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A1059%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C273.8%2Cnull%5D)
pub struct RuntimeVisibleParameterAnnotations {
    attribute_name_index: u16,
    attribute_length: u32,
    /**
     * *num_parameters*\
     *  The value of the num_parameters item gives the number of run-time visible
     *  parameter annotations represented by this structure.\
     *  There is no assurance that this number is the same as the number of parameter descriptors
     *  in the method descriptor.
     */
    num_parameters: u8,
    /**
     * *parameter_annotations*\
     *  Each entry in the parameter_annotations table represents all of the run-
     *  time visible annotations on the declaration of a single formal parameter.
     */
    parameter_annotations: Vec<ParamAnnotation>,
}

impl RuntimeVisibleParameterAnnotations {
    pub fn new(
        attribute_name_index: u16,
        attribute_length: u32,
        num_parameters: u8,
        cursor: &mut Cursor<&[u8]>,
    ) -> Result<RuntimeVisibleParameterAnnotations, Box<dyn Error>> {
        let mut parameter_annotations = Vec::with_capacity(num_parameters.into());
        for _ in 0..num_parameters {
            parameter_annotations.push(ParamAnnotation::new(cursor.read_u16::<BE>()?, cursor)?);
        }
        Ok(RuntimeVisibleParameterAnnotations {
            attribute_name_index,
            attribute_length,
            num_parameters,
            parameter_annotations,
        })
    }
}

#[derive(Clone, Debug)]
/// [RuntimeInvisibleParameterAnnotations](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A1082%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C462%2Cnull%5D)
pub struct RuntimeInvisibleParameterAnnotations {
    attribute_name_index: u16,
    attribute_length: u32,
    /**
     * *num_parameters*\
     *  The value of the num_parameters item gives the number of run-time visible
     *  parameter annotations represented by this structure.\
     *  There is no assurance that this number is the same as the number of parameter descriptors
     *  in the method descriptor.
     */
    num_parameters: u8,
    /**
     * *parameter_annotations*\
     *  Each entry in the parameter_annotations table represents all of the run-
     *  time visible annotations on the declaration of a single formal parameter.
     */
    parameter_annotations: Vec<ParamAnnotation>,
}

impl RuntimeInvisibleParameterAnnotations {
    pub fn new(
        attribute_name_index: u16,
        attribute_length: u32,
        num_parameters: u8,
        cursor: &mut Cursor<&[u8]>,
    ) -> Result<RuntimeInvisibleParameterAnnotations, Box<dyn Error>> {
        let mut parameter_annotations = Vec::with_capacity(num_parameters.into());
        for _ in 0..num_parameters {
            parameter_annotations.push(ParamAnnotation::new(cursor.read_u16::<BE>()?, cursor)?);
        }
        Ok(RuntimeInvisibleParameterAnnotations {
            attribute_name_index,
            attribute_length,
            num_parameters,
            parameter_annotations,
        })
    }
}

#[derive(Clone, Debug)]
/**
* *table*\
*  Each entry indicates a range of code array offsets within which a local
*  variable has a value. It also indicates the index into the local variable array of
*  the current frame at which that local variable can be found.
*/
struct LocalVarTargetTable {
    /**
     * *start_pc & length*\
     *  The given local variable has a value at indices into the code array in
     *  the interval [start_pc, start_pc + length), that is, between start_pc
     *  inclusive and start_pc + length exclusive.
     */
    start_pc: u16,
    length: u16,
    /**
     * *index*\
     *  The given local variable must be at index in the local variable array of the
     *  current frame.
     *
     *  If the local variable at index is of type double or long, it occupies both
     *  index and index + 1.
     *
     *  A table is needed to fully specify the local variable whose type is annotated, because
     *  a single local variable may be represented with different local variable indices over
     *  multiple live ranges. The start_pc, length, and index items in each table entry
     *  specify the same information as a LocalVariableTable attribute.
     *
     *  The localvar_target item records that a local variable's type is annotated, but
     *  does not record the type itself. The type may be found by inspecting the appropriate
     *  LocalVariableTable attribute.
     */
    index: u16,
}

impl LocalVarTargetTable {
    pub fn new(cursor: &mut Cursor<&[u8]>) -> Result<LocalVarTargetTable, Box<dyn Error>> {
        Ok(LocalVarTargetTable {
            start_pc: cursor.read_u16::<BE>()?,
            length: cursor.read_u16::<BE>()?,
            index: cursor.read_u16::<BE>()?,
        })
    }
}

#[derive(Clone, Debug)]
/// [TargetInfo](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A1106%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C231%2Cnull%5D)
/**
 * *target_info*\
 *  The items of the target_info union (except for the first) specify precisely which
 *  type in a declaration or expression is annotated. The first item specifies not which
 *  type, but rather which declaration of a type parameter is annotated.
 */
enum TargetInfo {
    /**
     * *type_parameter_target*\
     *  The type_parameter_target item indicates that an annotation appears on the
     *  declaration of the i'th type parameter of a generic class, generic interface, generic
     *  method, or generic constructor.
     */
    TypeParameterTarget {
        /**
         * *type_parameter_index*\
         *  The value of the type_parameter_index item specifies which type parameter
         *  declaration is annotated. A type_parameter_index value of 0 specifies the first
         *  type parameter declaration.
         */
        type_parameter_index: u8,
    },
    /**
     * *supertype_target*\
     *  The supertype_target item indicates that an annotation appears on a type in
     *  the extends or implements clause of a class or interface declaration.
     */
    SupertypeTarget {
        /**
         * *supertype_index*\
         *  A supertype_index value of 65535 specifies that the annotation appears on the
         *  superclass in an extends clause of a class declaration.
         *  Any other supertype_index value is an index into the interfaces array of
         *  the enclosing ClassFile structure, and specifies that the annotation appears on
         *  that superinterface in either the implements clause of a class declaration or the
         *  extends clause of an interface declaration.
         */
        supertype_index: u16,
    },
    /**
     * *type_parameter_bound_target*\
     *  The type_parameter_bound_target item indicates that an annotation appears
     *  on the i'th bound of the j'th type parameter declaration of a generic class,
     *  interface, method, or constructor.
     */
    TypeParameterBoundTarget {
        /**
         * *type_parameter_index*\
         *  The value of the of type_parameter_index item specifies which type parameter
         *  declaration has an annotated bound. A type_parameter_index value of 0
         *  specifies the first type parameter declaration.
         */
        type_parameter_index: u8,
        /**
         * *bound_index*\
         *  The value of the bound_index item specifies which bound of the type parameter
         *  declaration indicated by type_parameter_index is annotated. A bound_index
         *  value of 0 specifies the first bound of a type parameter declaration.
         *
         *  The type_parameter_bound_target item records that a bound is annotated, but does
         *  not record the type which constitutes the bound. The type may be found by inspecting
         *  the class signature or method signature stored in the appropriate Signature attribute.
         */
        bound_index: u8,
    },
    /**
     * *empty_target*\
     *  The empty_target item indicates that an annotation appears on either the type
     *  in a field declaration, the type in a record component declaration, the return type
     *  of a method, the type of a newly constructed object, or the receiver type of a
     *  method or constructor.\
     *  Only one type appears in each of these locations, so there is no per-type information to
     *  represent in the target_info union.
     */
    EmptyTarget,
    /**
     * *formal_parameter_target*\
     *  The formal_parameter_target item indicates that an annotation appears on
     *  the type in a formal parameter declaration of a method, constructor, or lambda
     *  expression.
     */
    FormalParameterTarget {
        /**
         * *formal_parameter_index*\
         *  The value of the formal_parameter_index item specifies which formal
         *  parameter declaration has an annotated type. A formal_parameter_index value
         *  of i may, but is not required to, correspond to the i'th parameter descriptor in the
         *  method descriptor (§4.3.3).
         *
         *  The formal_parameter_target item records that a formal parameter's type is
         *  annotated, but does not record the type itself. The type may be found by inspecting the
         *  method descriptor, although a formal_parameter_index value of 0 does not always
         *  indicate the first parameter descriptor in the method descriptor; see the note in §4.7.18
         *  for a similar situation involving the parameter_annotations table.
         */
        formal_parameter_index: u8,
    },
    /**
     * *throws_target*\
     *  The throws_target item indicates that an annotation appears on the i'th type in
     *  the throws clause of a method or constructor declaration.
     */
    ThrowsTarget {
        /**
         * *throws_type_index*\
         * The value of the throws_type_index item is an index into the
         *  exception_index_table array of the Exceptions attribute of the method_info
         *  structure enclosing the RuntimeVisibleTypeAnnotations attribute.
         */
        throws_type_index: u16,
    },
    /**
     * *local_var_target*\
     *  The localvar_target item indicates that an annotation appears on the type in
     *  a local variable declaration, including a variable declared as a resource in a try-
     *  with-resources statement.
     */
    LocalvarTarget {
        /**
         * *table_length*\
         *  The value of the table_length item gives the number of entries in the table
         *  array.
         */
        table_length: u16,
        /**
         * *table*\
         *  Each entry indicates a range of code array offsets within which a local
         *  variable has a value. It also indicates the index into the local variable array of
         *  the current frame at which that local variable can be found.
         */
        table: Vec<LocalVarTargetTable>,
    },
    /**
     * *catch_target*\
     *  The catch_target item indicates that an annotation appears on the i'th type in
     *  an exception parameter declaration.
     */
    CatchTarget {
        /**
         * *exception_table_index*\
         *  The value of the exception_table_index item is an index into
         *  the exception_table array of the Code attribute enclosing the
         *  RuntimeVisibleTypeAnnotations attribute.
         *
         *  The possibility of more than one type in an exception parameter declaration arises from
         *  the multi-catch clause of the try statement, where the type of the exception parameter
         *  is a union of types (JLS §14.20). A compiler usually creates one exception_table
         *  entry for each type in the union, which allows the catch_target item to distinguish
         *  them. This preserves the correspondence between a type and its annotations.
         */
        exception_table_index: u16,
    },
    /**
     * *offset_target*\
     *  The offset_target item indicates that an annotation appears on either the type
     *  in an instanceof expression or a new expression, or the type before the :: in a
     *  method reference expression.
     */
    OffsetTarget {
        /**
         * *offset*\
         *  The value of the offset item specifies the code array offset of either the bytecode
         *  instruction corresponding to the instanceof expression, the new bytecode
         *  instruction corresponding to the new expression, or the bytecode instruction
         *  corresponding to the method reference expression.
         */
        offset: u16,
    },
    /**
     * *type_argument_target*\
     *  The type_argument_target item indicates that an annotation appears either on
     *  the i'th type in a cast expression, or on the i'th type argument in the explicit type
     *  argument list for any of the following: a new expression, an explicit constructor
     *  invocation statement, a method invocation expression, or a method reference
     *  expression.
     */
    TypeArgumentTarget {
        /**
         * *offset*\
         *  The value of the offset item specifies the code array offset of either the
         *  bytecode instruction corresponding to the cast expression, the new bytecode
         *  instruction corresponding to the new expression, the bytecode instruction
         *  corresponding to the explicit constructor invocation statement, the bytecode
         *  instruction corresponding to the method invocation expression, or the bytecode
         *  instruction corresponding to the method reference expression.
         */
        offset: u16,
        /**
         * *type_argument_index*\
         *  For a cast expression, the value of the type_argument_index item specifies
         *  which type in the cast operator is annotated. A type_argument_index value of
         *  0 specifies the first (or only) type in the cast operator.
         *
         *  The possibility of more than one type in a cast expression arises from a cast to an
         *  intersection type.
         *
         *  For an explicit type argument list, the value of the type_argument_index item
         *  specifies which type argument is annotated. A type_argument_index value of
         *  0 specifies the first type argument.
         */
        type_argument_index: u8,
    },
}
impl TargetInfo {
    fn get_target(tag: u8, cursor: &mut Cursor<&[u8]>) -> Result<TargetInfo, Box<dyn Error>> {
        match tag {
            0x00 | 0x01 => Ok(TargetInfo::TypeParameterTarget {
                type_parameter_index: cursor.read_u8()?,
            }),
            0x10 => Ok(TargetInfo::SupertypeTarget {
                supertype_index: cursor.read_u16::<BE>()?,
            }),
            0x11 | 0x12 => Ok(TargetInfo::TypeParameterBoundTarget {
                type_parameter_index: cursor.read_u8()?,
                bound_index: cursor.read_u8()?,
            }),
            0x13..=0x15 => Ok(TargetInfo::EmptyTarget),
            0x16 => Ok(TargetInfo::FormalParameterTarget {
                formal_parameter_index: cursor.read_u8()?,
            }),
            0x17 => Ok(TargetInfo::ThrowsTarget {
                throws_type_index: cursor.read_u16::<BE>()?,
            }),
            0x40 | 0x41 => Ok({
                let table_length = cursor.read_u16::<BE>()?;
                let mut table = Vec::with_capacity(table_length as usize);
                for _ in 0..table_length {
                    table.push(LocalVarTargetTable::new(cursor)?);
                }
                assert!(table.len() == table_length as usize);
                TargetInfo::LocalvarTarget {
                    table_length,
                    table,
                }
            }),
            0x42 => Ok(TargetInfo::CatchTarget {
                exception_table_index: cursor.read_u16::<BE>()?,
            }),
            0x43..=0x46 => Ok(TargetInfo::OffsetTarget {
                offset: cursor.read_u16::<BE>()?,
            }),
            0x47..=0x4B => Ok(TargetInfo::TypeArgumentTarget {
                offset: cursor.read_u16::<BE>()?,
                type_argument_index: cursor.read_u8()?,
            }),
            _ => Err(Box::new(LoadingError::new(
                LoadingCause::InvalidTargetInfoValue(tag),
                "Received invalid target info type for RuntimeVisibleTypeAnnotation",
            ))),
        }
    }
}

#[derive(Clone, Debug)]
struct PathDescriptor {
    /// [type_path_kind](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A1140%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C512%2Cnull%5D)
    type_path_kind: u8,
    /**
     * *type_argument_index*\
     *  If the value of the type_path_kind item is 0, 1, or 2, then the value of the
     *  type_argument_index item is 0.
     *
     *  If the value of the type_path_kind item is 3, then the value of
     *  the type_argument_index item specifies which type argument of a
     *  parameterized type is annotated, where 0 indicates the first type argument
     *  of a parameterized type.
     */
    type_argument_index: u8,
}

impl PathDescriptor {
    fn new(cursor: &mut Cursor<&[u8]>) -> Result<PathDescriptor, Box<dyn Error>> {
        let type_path_kind = cursor.read_u8()?;
        if type_path_kind > 3 {
            return Err(Box::new(LoadingError::new(
                LoadingCause::InvalidTypePathKind(type_path_kind),
                "Received type_path_kind > 3",
            )));
        }
        let type_argument_index = if (0..3).contains(&type_path_kind) {
            0
        } else {
            cursor.read_u8()?
        };
        Ok(PathDescriptor {
            type_path_kind,
            type_argument_index,
        })
    }
}

#[derive(Clone, Debug)]
/// [TypePath](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A1124%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C436%2Cnull%5D)
/**
 * *type_path*\
 *  Wherever a type is used in a declaration or expression, the type_path structure
 *  identifies which part of the type is annotated. An annotation may appear on the
 *  type itself, but if the type is a reference type, then there are additional locations
 *  where an annotation may appear
 */
struct TypePath {
    /**
     * *path_length*\
     *  The value of the path_length item gives the number of entries in the path array:\
     *  • If the value of path_length is 0, and the type being annotated is a nested type,
     *  then the annotation applies to the outermost part of the type for which a type
     *  annotation is admissible.\
     *  • If the value of path_length is 0, and the type being annotated is not a nested
     *  type, then the annotation appears directly on the type itself.\
     *  • If the value of path_length is non-zero, then each entry in the path array
     *  represents an iterative, left-to-right step towards the precise location of the
     *  annotation in an array type, nested type, or parameterized type. (In an array
     *  type, the iteration visits the array type itself, then its component type, then the
     *  component type of that component type, and so on, until the element type is
     *  reached.)
     */
    path_length: u8,
    path: Vec<PathDescriptor>,
}
impl TypePath {
    fn new(cursor: &mut Cursor<&[u8]>) -> Result<TypePath, Box<dyn Error>> {
        let path_length = cursor.read_u8()?;
        let mut path = Vec::with_capacity(path_length as usize);
        for _ in 0..path_length {
            path.push(PathDescriptor::new(cursor)?);
        }
        assert!(path.len() == path_length as usize);
        Ok(TypePath { path_length, path })
    }
}

#[derive(Clone, Debug)]
/**
 * *TypeAnnotation*\
 *  Each entry in the annotations table represents a single run-time visible
 *  annotation on a type used in a declaration or expression.
 */
struct TypeAnnotation {
    target_type: u8,
    target_info: TargetInfo,
    target_path: TypePath,
    type_index: u16,
    num_element_value_pairs: u16,
    element_value_pairs: Vec<ElementPairs>,
}

impl TypeAnnotation {
    pub fn new(cursor: &mut Cursor<&[u8]>) -> Result<TypeAnnotation, Box<dyn Error>> {
        let target_type = cursor.read_u8()?;
        match target_type {
            0x00 | 0x01 | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x40..=0x4B => {}
            _ => {
                return Err(Box::new(LoadingError::new(
                    LoadingCause::InvalidTargetTypeValue(target_type),
                    "TargetTypeValue for RuntimeVisibleTypeAnnotations was invalid",
                )))
            }
        }
        let target_info = TargetInfo::get_target(target_type, cursor)?;
        let target_path = TypePath::new(cursor)?;
        let type_index = cursor.read_u16::<BE>()?;
        let num_element_value_pairs = cursor.read_u16::<BE>()?;
        let mut element_value_pairs = Vec::with_capacity(num_element_value_pairs as usize);
        for _ in 0..num_element_value_pairs {
            element_value_pairs.push(ElementPairs::new(cursor.read_u16::<BE>()?, cursor)?);
        }
        assert!(element_value_pairs.len() == num_element_value_pairs as usize);

        Ok(TypeAnnotation {
            target_type,
            target_info,
            target_path,
            type_index,
            num_element_value_pairs,
            element_value_pairs,
        })
    }
}

#[derive(Clone, Debug)]
/// [RuntimeVisibleTypeAnnotations](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A1292%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C136%2Cnull%5D)
pub struct RuntimeVisibleTypeAnnotations {
    attribute_name_index: u16,
    attribute_length: u32,
    /**
     * *num_annotations*\
     *  The value of the num_annotations item gives the number of run-time visible
     *  type annotations represented by the structure.
     */
    num_annotations: u16,
    /**
     * *type_annotations*\
     *  Each entry in the type_annotations table represents a single run-time visible
     *  annotation on a type used in a declaration or expression.
     */
    type_annotations: Vec<TypeAnnotation>,
}

impl RuntimeVisibleTypeAnnotations {
    pub fn new(
        attribute_name_index: u16,
        attribute_length: u32,
        num_annotations: u16,
        cursor: &mut Cursor<&[u8]>,
    ) -> Result<RuntimeVisibleTypeAnnotations, Box<dyn Error>> {
        let mut type_annotations = Vec::with_capacity(num_annotations as usize);
        for _ in 0..num_annotations {
            type_annotations.push(TypeAnnotation::new(cursor)?);
        }
        assert!(type_annotations.len() == num_annotations as usize);
        Ok(RuntimeVisibleTypeAnnotations {
            attribute_name_index,
            attribute_length,
            num_annotations,
            type_annotations,
        })
    }
}

#[derive(Clone, Debug)]
/// [RuntimeInvisibleTypeAnnotations](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A1312%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C590%2Cnull%5D)
pub struct RuntimeInvisibleTypeAnnotations {
    attribute_name_index: u16,
    attribute_length: u32,
    /**
     * *num_annotations*\
     *  The value of the num_annotations item gives the number of run-time visible
     *  type annotations represented by the structure.
     */
    num_annotations: u16,
    /**
     * *type_annotations*\
     *  Each entry in the type_annotations table represents a single run-time visible
     *  annotation on a type used in a declaration or expression.
     */
    type_annotations: Vec<TypeAnnotation>,
}

impl RuntimeInvisibleTypeAnnotations {
    pub fn new(
        attribute_name_index: u16,
        attribute_length: u32,
        num_annotations: u16,
        cursor: &mut Cursor<&[u8]>,
    ) -> Result<RuntimeInvisibleTypeAnnotations, Box<dyn Error>> {
        let mut type_annotations = Vec::with_capacity(num_annotations as usize);
        for _ in 0..num_annotations {
            type_annotations.push(TypeAnnotation::new(cursor)?);
        }
        assert!(type_annotations.len() == num_annotations as usize);
        Ok(RuntimeInvisibleTypeAnnotations {
            attribute_name_index,
            attribute_length,
            num_annotations,
            type_annotations,
        })
    }
}

#[derive(Clone, Debug)]
/// [AnnotationDefault](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A1161%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C537%2Cnull%5D)
pub struct AnnotationDefault {
    attribute_name_index: u16,
    attribute_length: u32,
    /**
     * *default_value*\
     *  The default_value item represents the default value of the annotation
     *  interface element represented by the method_info structure enclosing this
     *  AnnotationDefault attribute.
     */
    default_value: Element,
}

impl AnnotationDefault {
    pub fn new(
        attribute_name_index: u16,
        attribute_length: u32,
        cursor: &mut Cursor<&[u8]>,
    ) -> Result<AnnotationDefault, Box<dyn Error>> {
        Ok(AnnotationDefault {
            attribute_name_index,
            attribute_length,
            default_value: Element::get_element(cursor.read_u8()?, cursor)?,
        })
    }
}

#[derive(Clone, Debug)]
struct Methods {
    /**
     * *bootstrap_method_ref*\
     *  The value of the bootstrap_method_ref item must be a valid index into
     *  the constant_pool table. The constant_pool entry at that index must be
     *  a CONSTANT_MethodHandle_info structure (§4.4.8).\
     *  The method handle will be resolved during resolution of a dynamically-
     *  computed constant or call site (§5.4.3.6), and then invoked as if by invocation
     *  of invokeWithArguments in java.lang.invoke.MethodHandle. The method
     *  handle must be able to accept the array of arguments described in §5.4.3.6, or
     *  resolution will fail.
     */
    bootstrap_method_ref: u16,
    /**
     * *num_bootstrap_arguments*\
     *  The value of the num_bootstrap_arguments item gives the number of
     *  items in the bootstrap_arguments array.
     */
    num_bootstrap_arguments: u16,
    /**
     * *bootstrap_arguments*\
     *  Each entry in the bootstrap_arguments array must be a valid index into
     *  the constant_pool table. The constant_pool entry at that index must be
     *  loadable (§4.4).
     */
    bootstrap_arguments: Vec<u16>,
}

impl Methods {
    fn new(
        bootstrap_method_ref: u16,
        num_bootstrap_arguments: u16,
        cursor: &mut Cursor<&[u8]>,
    ) -> Result<Methods, Box<dyn Error>> {
        let mut arguments = Vec::with_capacity(num_bootstrap_arguments.into());
        for _ in 0..num_bootstrap_arguments {
            arguments.push(cursor.read_u16::<BE>()?);
        }
        Ok(Methods {
            bootstrap_method_ref,
            num_bootstrap_arguments,
            bootstrap_arguments: arguments,
        })
    }
}

#[derive(Clone, Debug)]
/// [BootstrapMethods](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A1179%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C590%2Cnull%5D)
pub struct BootstrapMethods {
    attribute_name_index: u16,
    attribute_length: u32,
    /**
     * *num_bootstrap_methods*\
     *  The value of the num_bootstrap_methods item determines the number of
     *  bootstrap method specifiers in the bootstrap_methods array.
     */
    num_bootstrap_methods: u16,
    /**
     * *bootstrap_methods*\
     *  Each entry in the bootstrap_methods table contains an index to a
     *  CONSTANT_MethodHandle_info structure which specifies a bootstrap method,
     *  and a sequence (perhaps empty) of indexes to static arguments for the bootstrap
     *  method.
     */
    bootstrap_methods: Vec<Methods>,
}

impl BootstrapMethods {
    pub fn new(
        attribute_name_index: u16,
        attribute_length: u32,
        num_bootstrap_methods: u16,
        cursor: &mut Cursor<&[u8]>,
    ) -> Result<BootstrapMethods, Box<dyn Error>> {
        let mut bootstrap_methods = Vec::with_capacity(num_bootstrap_methods.into());
        for _ in 0..num_bootstrap_methods {
            bootstrap_methods.push(Methods::new(
                cursor.read_u16::<BE>()?,
                cursor.read_u16::<BE>()?,
                cursor,
            )?);
        }
        Ok(BootstrapMethods {
            attribute_name_index,
            attribute_length,
            num_bootstrap_methods,
            bootstrap_methods,
        })
    }
}

#[derive(Clone, Debug)]
struct Parameters {
    /**
     * *name_index*\
     *  The value of the name_index item must either be zero or a valid index into
     *  the constant_pool table.
     *
     *  If the value of the name_index item is zero, then this parameters element
     *  indicates a formal parameter with no name.
     *
     *  If the value of the name_index item is nonzero, the constant_pool entry
     *  at that index must be a CONSTANT_Utf8_info structure representing a valid
     *  unqualified name denoting a formal parameter (§4.2.2).
     */
    name_index: u16,
    access_flags: Vec<ParameterAccessFlags>,
}

impl Parameters {
    pub fn new(cursor: &mut Cursor<&[u8]>) -> Result<Parameters, Box<dyn Error>> {
        Ok(Parameters {
            name_index: cursor.read_u16::<BE>()?,
            access_flags: ParameterAccessFlags::from_u16(cursor.read_u16::<BE>()?),
        })
    }
}

#[derive(Clone, Debug)]
/// [MethodParameters](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A2433%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C343%2Cnull%5D)
pub struct MethodParameters {
    attribute_name_index: u16,
    attribute_length: u32,
    /**
     * *parameters_count*\
     *  The value of the parameters_count item indicates the number of
     *  parameter descriptors in the method descriptor (§4.3.3) referenced by the
     *  descriptor_index of the attribute's enclosing method_info structure.
     *
     *  This is not a constraint which a Java Virtual Machine implementation must enforce during
     *  format checking (§4.8). The task of matching parameter descriptors in a method descriptor
     *  against the items in the parameters array below is done by the reflection libraries of the
     *  Java SE Platform.
     */
    parameters_count: u8,
    /**
     * *parameters*\
     *  The i'th entry in the parameters array corresponds to the i'th parameter descriptor in
     *  the enclosing method's descriptor. (The parameters_count item is one byte because a
     *  method descriptor is limited to 255 parameters.) Effectively, this means the parameters
     *  array stores information for all the parameters of the method. One could imagine other
     *  schemes, where entries in the parameters array specify their corresponding parameter
     *  descriptors, but it would unduly complicate the MethodParameters attribute.
     *
     *  The i'th entry in the parameters array may or may not correspond to the i'th type in
     *  the enclosing method's Signature attribute (if present), or to the i'th annotation in the
     *  enclosing method's parameter annotations.
     */
    parameters: Vec<Parameters>,
}

impl MethodParameters {
    pub fn new(
        attribute_name_index: u16,
        attribute_length: u32,
        parameters_count: u8,
        cursor: &mut Cursor<&[u8]>,
    ) -> Result<MethodParameters, Box<dyn Error>> {
        let mut parameters = Vec::with_capacity(parameters_count as usize);
        for _ in 0..parameters_count {
            parameters.push(Parameters::new(cursor)?);
        }
        assert!(parameters.len() == parameters_count as usize);
        Ok(MethodParameters {
            attribute_name_index,
            attribute_length,
            parameters_count,
            parameters,
        })
    }
}

#[derive(Clone, Debug)]
struct ModuleRequires {
    /**
     * *requires_index*\
     *  The value of the requires_index item must be a valid index into the
     *  constant_pool table. The constant_pool entry at that index must be a
     *  CONSTANT_Module_info structure denoting a module on which the current
     *  module depends.
     *
     *  At most one entry in the requires table may specify a module of a given
     *  name with its requires_index item.
     */
    requires_index: u16,
    /**
     * *requires_flags*\
     *  If the current module is not java.base, and the class file version number
     *  is 54.0 or above, then neither ACC_TRANSITIVE nor ACC_STATIC_PHASE
     *  may be set in requires_flags.
     */
    requires_flags: Vec<ModuleFlags::RequiresAccessFlags>,
    /**
     * *requires_version_index*\
     *  The value of the requires_version_index item must be either zero or a
     *  valid index into the constant_pool table. If the value of the item is zero,
     *  then no version information about the dependence is present. If the value
     *  of the item is nonzero, then the constant_pool entry at that index must be
     *  a CONSTANT_Utf8_info structure representing the version of the module
     *  specified by requires_index.
     */
    requires_version_index: u16,
}

impl ModuleRequires {
    pub fn new(cursor: &mut Cursor<&[u8]>) -> Result<ModuleRequires, Box<dyn Error>> {
        Ok(ModuleRequires {
            requires_index: cursor.read_u16::<BE>()?,
            requires_flags: ModuleFlags::RequiresAccessFlags::from_u16(cursor.read_u16::<BE>()?),
            requires_version_index: cursor.read_u16::<BE>()?,
        })
    }
}

#[derive(Clone, Debug)]
struct ModuleExports {
    /**
     * *exports_index*\
     *  The value of the exports_index item must be a valid index into the
     *  constant_pool table. The constant_pool entry at that index must be
     *  a CONSTANT_Package_info structure (§4.4.12) representing a package
     *  exported by the current module.
     *
     *  At most one entry in the exports table may specify a package of a given
     *  name with its exports_index item.
     */
    exports_index: u16,
    exports_flags: Vec<ModuleFlags::ExportsAccessFlags>,
    /**
     * *exports_to_count*\
     *  The value of the exports_to_count indicates the number of entries in the
     *  exports_to_index table.
     *
     *  If exports_to_count is zero, then this package is exported by the current
     *  module in an unqualified fashion; code in any other module may access
     *  the types and members in the package.
     *
     *  If exports_to_count is nonzero, then this package is exported by the
     *  current module in a qualified fashion; only code in the modules listed in
     *  the exports_to_index table may access the types and members in the
     *  package.
     */
    exports_to_count: u16,
    /**
     * *exports_to_index*\
     *  The value of each entry in the exports_to_index table must be a valid
     *  index into the constant_pool table. The constant_pool entry at that
     *  index must be a CONSTANT_Module_info structure denoting a module
     *  whose code can access the types and members in this exported package.
     *
     *  For each entry in the exports table, at most one entry in its
     *  exports_to_index table may specify a module of a given name.
     */
    exports_to_index: Vec<u16>,
}

impl ModuleExports {
    pub fn new(cursor: &mut Cursor<&[u8]>) -> Result<ModuleExports, Box<dyn Error>> {
        let exports_index = cursor.read_u16::<BE>()?;
        let exports_flags = ModuleFlags::ExportsAccessFlags::from_u16(cursor.read_u16::<BE>()?);
        let exports_to_count = cursor.read_u16::<BE>()?;
        let mut exports_to_index: Vec<u16> = Vec::with_capacity(exports_to_count as usize);
        for _ in 0..exports_to_count {
            exports_to_index.push(cursor.read_u16::<BE>()?);
        }
        assert!(exports_to_index.len() == exports_to_count as usize);
        Ok(ModuleExports {
            exports_index,
            exports_flags,
            exports_to_count,
            exports_to_index,
        })
    }
}

#[derive(Clone, Debug)]
struct ModuleOpens {
    /**
     * *opens_index*\
     *  The value of the opens_index item must be a valid index into the
     *  constant_pool table. The constant_pool entry at that index must be a
     *  CONSTANT_Package_info structure representing a package opened by the
     *  current module.
     *
     *  At most one entry in the opens table may specify a package of a given
     *  name with its opens_index item.
     */
    opens_index: u16,
    /**
     * *opens_flags*
     */
    opens_flags: Vec<ModuleFlags::OpensAccessFlags>,
    /**
     * *opens_to_count*\
     *  The value of the opens_to_count indicates the number of entries in the
     *  opens_to_index table.
     *
     *  If opens_to_count is zero, then this package is opened by the current
     *  module in an unqualified fashion; code in any other module may
     *  reflectively access the types and members in the package.
     *
     *  If opens_to_count is nonzero, then this package is opened by the current
     *  module in a qualified fashion; only code in the modules listed in the
     *  exports_to_index table may reflectively access the types and members
     *  in the package.
     */
    opens_to_count: u16,
    /**
     * *opens_to_index*\
     *  The value of each entry in the opens_to_index table must be a valid index
     *  into the constant_pool table. The constant_pool entry at that index must
     *  be a CONSTANT_Module_info structure denoting a module whose code can
     *  access the types and members in this opened package.
     *
     *  For each entry in the opens table, at most one entry in its opens_to_index
     *  table may specify a module of a given name.
     */
    opens_to_index: Vec<u16>,
}

impl ModuleOpens {
    pub fn new(cursor: &mut Cursor<&[u8]>) -> Result<ModuleOpens, Box<dyn Error>> {
        let opens_index = cursor.read_u16::<BE>()?;
        let opens_flags = ModuleFlags::OpensAccessFlags::from_u16(cursor.read_u16::<BE>()?);
        let opens_to_count = cursor.read_u16::<BE>()?;
        let mut opens_to_index: Vec<u16> = Vec::with_capacity(opens_to_count as usize);
        for _ in 0..opens_to_count {
            opens_to_index.push(cursor.read_u16::<BE>()?);
        }
        assert!(opens_to_index.len() == opens_to_count as usize);
        Ok(ModuleOpens {
            opens_index,
            opens_flags,
            opens_to_count,
            opens_to_index,
        })
    }
}

#[derive(Clone, Debug)]
struct ModuleProvides {
    /**
     * *provides_index*\
     *  The value of the provides_index item must be a valid index into the
     *  constant_pool table. The constant_pool entry at that index must be a
     *  CONSTANT_Class_info structure representing a service interface for which
     *  the current module provides a service implementation.
     *
     *  At most one entry in the provides table may specify a service interface of
     *  a given name with its provides_index item.
     */
    // FIXME: Anything that is an index into the constant pool should maybe just be a constant pool object
    provides_index: u16,
    /**
     * *provides_with_count*\
     *  The value of the provides_with_count indicates the number of entries in
     *  the provides_with_index table.
     *  provides_with_count must be nonzero.
     */
    provides_with_count: u16,
    /**
     * *provides_with_index*\
     *  The value of each entry in the provides_with_index table must be a valid
     *  index into the constant_pool table. The constant_pool entry at that
     *  index must be a CONSTANT_Class_info structure representing a service
     *  implementation for the service interface specified by provides_index.
     *
     *  For each entry in the provides table, at most one entry in its
     *  provides_with_index table may specify a service implementation of a
     *  given name.
     */
    provides_with_index: Vec<u16>,
}

impl ModuleProvides {
    pub fn new(cursor: &mut Cursor<&[u8]>) -> Result<ModuleProvides, Box<dyn Error>> {
        let provides_index = cursor.read_u16::<BE>()?;
        let provides_with_count = cursor.read_u16::<BE>()?;
        let mut provides_with_index: Vec<u16> = Vec::with_capacity(provides_with_count as usize);
        for _ in 0..provides_with_count {
            provides_with_index.push(cursor.read_u16::<BE>()?);
        }
        assert!(provides_with_index.len() == provides_with_count as usize);
        Ok(ModuleProvides {
            provides_index,
            provides_with_count,
            provides_with_index,
        })
    }
}

#[derive(Clone, Debug)]
/// [Module](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A1184%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C377%2Cnull%5D)
pub struct Module {
    attribute_name_index: u16,
    attribute_length: u32,
    /**
     * *module_name_index*\
     *  The value of the module_name_index item must be a valid index into the
     *  constant_pool table. The constant_pool entry at that index must be a
     *  CONSTANT_Module_info structure (§4.4.11) denoting the current module.
     */
    module_name_index: u16,
    module_flags: Vec<ModuleFlags::ModuleAccessFlags>,
    /**
     * *module_version_index*\
     *  The value of the module_version_index item must be either zero or a valid
     *  index into the constant_pool table. If the value of the item is zero, then
     *  no version information about the current module is present. If the value of
     *  the item is nonzero, then the constant_pool entry at that index must be a
     *  CONSTANT_Utf8_info structure representing the version of the current module.
     */
    module_version_index: u16,

    /**
     * *requires_count*\
     *  The value of the requires_count item indicates the number of entries in the
     *  requires table.
     *  If the current module is java.base, then requires_count must be zero.
     *  If the current module is not java.base, then requires_count must be at least
     *  one.
     */
    requires_count: u16,
    /**
     * *requires*\
     *  Each entry in the requires table specifies a dependence of the current module.
     *
     *  Unless the current module is java.base, exactly one entry in the requires
     *  table must have both a requires_index item which indicates java.base and
     *  a requires_flags item which has the ACC_SYNTHETIC flag not set.
     */
    requires: Vec<ModuleRequires>,

    /**
     * *exports_count*\
     * The value of the exports_count item indicates the number of entries in the
     *  exports table.
     */
    exports_count: u16,
    /**
     * *exports*\
     *  Each entry in the exports table specifies a package exported by the current
     *  module, such that public and protected types in the package, and their
     *  public and protected members, may be accessed from outside the current
     *  module, possibly from a limited set of "friend" modules.
     */
    exports: Vec<ModuleExports>,

    /**
     * *opens_count*\
     *  The value of the opens_count item indicates the number of entries in the opens
     *  table.
     */
    opens_count: u16,
    /**
     * *opens*\
     *  Each entry in the opens table specifies a package opened by the current module,
     *  such that all types in the package, and all their members, may be accessed from
     *  outside the current module via the reflection libraries of the Java SE Platform,
     *  possibly from a limited set of "friend" modules.
     */
    opens: Vec<ModuleOpens>,

    /**
     * *uses_count*\
     *  The value of the uses_count item indicates the number of entries in the
     *  uses_index table.
     */
    uses_count: u16,
    /**
     * *uses_index*\
     *  The value of each entry in the uses_index table must be a valid index into
     *  the constant_pool table. The constant_pool entry at that index must be
     *  a CONSTANT_Class_info structure (§4.4.1) representing a service interface
     *  which the current module may discover via java.util.ServiceLoader.
     *
     *  At most one entry in the uses_index table may specify a service interface of
     *  a given name.
     */
    uses_index: Vec<u16>,

    /**
     * *provides_count*\
     *  The value of the provides_count item indicates the number of entries in the
     *  provides table.
     */
    provides_count: u16,
    /**
     * *provides*\
     *  Each entry in the provides table represents a service implementation for a
     *  given service interface.
     */
    provides: Vec<ModuleProvides>,
}

impl Module {
    pub fn new(
        attribute_name_index: u16,
        attribute_length: u32,
        cursor: &mut Cursor<&[u8]>,
    ) -> Result<Module, Box<dyn Error>> {
        let module_name_index = cursor.read_u16::<BE>()?;
        let module_flags = ModuleFlags::ModuleAccessFlags::from_u16(cursor.read_u16::<BE>()?);
        let module_version_index = cursor.read_u16::<BE>()?;

        let requires_count = cursor.read_u16::<BE>()?;
        let mut requires: Vec<ModuleRequires> = Vec::with_capacity(requires_count as usize);
        for _ in 0..requires_count {
            requires.push(ModuleRequires::new(cursor)?);
        }
        assert!(requires.len() == requires_count as usize);

        let exports_count = cursor.read_u16::<BE>()?;
        let mut exports: Vec<ModuleExports> = Vec::with_capacity(exports_count as usize);
        for _ in 0..exports_count {
            exports.push(ModuleExports::new(cursor)?);
        }
        assert!(exports.len() == exports_count as usize);

        let opens_count = cursor.read_u16::<BE>()?;
        let mut opens: Vec<ModuleOpens> = Vec::with_capacity(opens_count as usize);
        for _ in 0..opens_count {
            opens.push(ModuleOpens::new(cursor)?);
        }
        assert!(opens.len() == opens_count as usize);

        let uses_count = cursor.read_u16::<BE>()?;
        let mut uses_index: Vec<u16> = Vec::with_capacity(uses_count as usize);
        for _ in 0..uses_count {
            uses_index.push(cursor.read_u16::<BE>()?)
        }
        assert!(uses_index.len() == uses_count as usize);

        let provides_count = cursor.read_u16::<BE>()?;
        let mut provides: Vec<ModuleProvides> = Vec::with_capacity(provides_count as usize);
        for _ in 0..provides_count {
            provides.push(ModuleProvides::new(cursor)?);
        }
        assert!(provides.len() == provides_count as usize);

        Ok(Module {
            attribute_name_index,
            attribute_length,
            module_name_index,
            module_flags,
            module_version_index,
            requires_count,
            requires,
            exports_count,
            exports,
            opens_count,
            opens,
            uses_count,
            uses_index,
            provides_count,
            provides,
        })
    }
}

#[derive(Clone, Debug)]
/// [ModulePackages](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A1230%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C479%2Cnull%5D)
pub struct ModulePackages {
    attribute_name_index: u16,
    attribute_length: u32,
    /**
     * *package_count*\
     *  The value of the package_count item indicates the number of entries in the
     *  package_index table.
     */
    package_count: u16,
    /**
     * *package_index*\
     *  The value of each entry in the package_index table must be a valid index
     *  into the constant_pool table. The constant_pool entry at that index must be
     *  a CONSTANT_Package_info structure (§4.4.12) representing a package in the
     *  current module.
     *
     *  At most one entry in the package_index table may specify a package of a
     *  given name.
     */
    package_index: Vec<u16>,
}

impl ModulePackages {
    pub fn new(
        attribute_name_index: u16,
        attribute_length: u32,
        package_count: u16,
        cursor: &mut Cursor<&[u8]>,
    ) -> Result<ModulePackages, Box<dyn Error>> {
        let mut package_index: Vec<u16> = Vec::with_capacity(package_count as usize);
        for _ in 0..package_count {
            package_index.push(cursor.read_u16::<BE>()?);
        }
        assert!(package_index.len() == package_count as usize);
        Ok(ModulePackages {
            attribute_name_index,
            attribute_length,
            package_count,
            package_index,
        })
    }
}

#[derive(Clone, Debug)]
/// [ModuleMainClass](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A1237%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C445%2Cnull%5D)
pub struct ModuleMainClass {
    attribute_name_index: u16,
    attribute_length: u32,
    /**
     * *main_class_index*\
     *  The value of the main_class_index item must be a valid index into the
     *  constant_pool table. The constant_pool entry at that index must be a
     *  CONSTANT_Class_info structure (§4.4.1) representing the main class of the
     *  current module.
     */
    main_class_index: u16,
}

impl ModuleMainClass {
    pub fn new(
        attribute_name_index: u16,
        attribute_length: u32,
        main_class_index: u16,
    ) -> Result<ModuleMainClass, Box<dyn Error>> {
        Ok(ModuleMainClass {
            attribute_name_index,
            attribute_length,
            main_class_index,
        })
    }
}

#[derive(Clone, Debug)]
/// [NestHost](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A2472%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C564%2Cnull%5D)
pub struct NestHost {
    attribute_name_index: u16,
    attribute_length: u32,
    /**
     * *host_class_index*\
     *  The value of the host_class_index item must be a valid index into the
     *  constant_pool table. The constant_pool entry at that index must be a
     *  CONSTANT_Class_info structure (§4.4.1) representing a class or interface
     *  which is the nest host for the current class or interface.
     *
     *  If the nest host cannot be loaded, or is not in the same run-time package as the current class
     *  or interface, or does not authorize nest membership for the current class or interface, then
     *  an error may occur during access control (§5.4.4).
     */
    host_class_index: u16,
}

impl NestHost {
    pub fn new(
        attribute_name_index: u16,
        attribute_length: u32,
        host_class_index: u16,
    ) -> Result<NestHost, Box<dyn Error>> {
        Ok(NestHost {
            attribute_name_index,
            attribute_length,
            host_class_index,
        })
    }
}

#[derive(Clone, Debug)]
/// []()
pub struct NestMembers {
    attribute_name_index: u16,
    attribute_length: u32,
    /**
     * *number_of_classes*\
     *  The value of the number_of_classes item indicates the number of entries in
     *  the classes array.
     */
    number_of_classes: u16,
    /**
     * *classes*\
     *  Each value in the classes array must be a valid index into the constant_pool
     *  table. The constant_pool entry at that index must be a CONSTANT_Class_info
     *  structure (§4.4.1) representing a class or interface which is a member of the
     *  nest hosted by the current class or interface.
     *
     *  The classes array is consulted by access control (§5.4.4). It should consist of references
     *  to other classes and interfaces that are in the same run-time package and have NestHost
     *  attributes which reference the current class or interface. Array items that do not meet these
     *  criteria are ignored by access control.
     */
    classes: Vec<u16>,
}

impl NestMembers {
    pub fn new(
        attribute_name_index: u16,
        attribute_length: u32,
        number_of_classes: u16,
        cursor: &mut Cursor<&[u8]>,
    ) -> Result<NestMembers, Box<dyn Error>> {
        let mut classes: Vec<u16> = Vec::with_capacity(number_of_classes as usize);
        for _ in 0..number_of_classes {
            classes.push(cursor.read_u16::<BE>()?);
        }
        assert!(classes.len() == number_of_classes as usize);
        Ok(NestMembers {
            attribute_name_index,
            attribute_length,
            number_of_classes,
            classes,
        })
    }
}

#[derive(Clone, Debug)]
struct RecordComponentInfo {
    /**
     * *name_index*\
     *  The value of the name_index item must be a valid index into the
     *  constant_pool table. The constant_pool entry at that index must be a
     *  CONSTANT_Utf8_info structure (§4.4.7) representing a valid unqualified
     *  name denoting the record component (§4.2.2).
     */
    name_index: u16,
    /**
     * *descriptor_index*\
     *  The value of the descriptor_index item must be a valid index into the
     *  constant_pool table. The constant_pool entry at that index must be
     *  a CONSTANT_Utf8_info structure (§4.4.7) representing a field descriptor
     *  which encodes the type of the record component (§4.3.2).
     */
    descriptor_index: u16,
    /**
     * *attributes_count*\
     *  The value of the attributes_count item indicates the number of
     *  additional attributes of this record component.
     */
    attributes_count: u16,
    /**
     * *attributes*\
     *  Each value of the attributes table must be an attribute_info structure
     *  (§4.7).
     *  A record component can have any number of optional attributes associated
     *  with it.
     *  The attributes defined by this specification as appearing in the attributes
     *  table of a record_component_info structure are listed in Table 4.7-C.
     *  The rules concerning attributes defined to appear in the attributes table
     *  of a record_component_info structure are given in §4.7.
     *  The rules concerning non-predefined attributes in the attributes table of
     *  a record_component_info structure are given in §4.7.1.
     */
    attributes: Vec<crate::class_file::AttributeInfo>,
}

impl RecordComponentInfo {
    pub fn new(
        constant_pool: &[crate::class_file::ConstantPool],
        version: Option<u16>,
        cursor: &mut Cursor<&[u8]>,
    ) -> Result<RecordComponentInfo, Box<dyn Error>> {
        let name_index = cursor.read_u16::<BE>()?;
        let descriptor_index = cursor.read_u16::<BE>()?;

        let attributes_count = cursor.read_u16::<BE>()?;
        let mut attributes: Vec<crate::class_file::AttributeInfo> =
            Vec::with_capacity(attributes_count as usize);
        read_attributes(constant_pool, &mut attributes, cursor, version)?;
        assert!(attributes.len() == attributes_count as usize);
        Ok(RecordComponentInfo {
            name_index,
            descriptor_index,
            attributes_count,
            attributes,
        })
    }
}

#[derive(Clone, Debug)]
/// [Record](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A1243%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C590%2Cnull%5D)
pub struct Record {
    attribute_name_index: u16,
    attribute_length: u32,
    /**
     * *components_count*\
     *  The value of the components_count item indicates the number of entries in
     *  the components table.
     */
    components_count: u16,
    /**
     * *components*\
     *  Each entry in the components table specifies a record component of the
     *  current class, in the order the record components were declared.
     */
    components: Vec<RecordComponentInfo>,
}

impl Record {
    pub fn new(
        attribute_name_index: u16,
        attribute_length: u32,
        components_count: u16,
        constant_pool: &[crate::class_file::ConstantPool],
        version: Option<u16>,
        cursor: &mut Cursor<&[u8]>,
    ) -> Result<Record, Box<dyn Error>> {
        let mut components: Vec<RecordComponentInfo> =
            Vec::with_capacity(components_count as usize);
        for _ in 0..components_count {
            components.push(RecordComponentInfo::new(constant_pool, version, cursor)?);
        }
        assert!(components.len() == components_count as usize);

        Ok(Record {
            attribute_name_index,
            attribute_length,
            components_count,
            components,
        })
    }
}

#[derive(Clone, Debug)]
/// [PermittedSubclasses](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A2280%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C229%2Cnull%5D)
pub struct PermittedSubclasses {
    attribute_name_index: u16,
    attribute_length: u32,
    /**
     * *number_of_classes*\
     *  The value of the number_of_classes item indicates the number of entries in
     *  the classes array.
     */
    number_of_classes: u16,
    /**
     * *classes*\
     *  Each value in the classes array must be a valid index into the constant_pool
     *  table. The constant_pool entry at that index must be a CONSTANT_Class_info
     *  structure (§4.4.1) representing a class or interface which is authorized to
     *  directly extend or implement the current class or interface.
     *
     *  The classes array is consulted when a class or interface is created that attempts to directly
     *  extend or implement the current class or interface (§5.3.5). Array items that represent
     *  classes or interfaces which do not attempt to directly extend or implement the current class
     *  or interface are ignored.
     */
    classes: Vec<u16>,
}

impl PermittedSubclasses {
    pub fn new(
        attribute_name_index: u16,
        attribute_length: u32,
        number_of_classes: u16,
        cursor: &mut Cursor<&[u8]>,
    ) -> Result<PermittedSubclasses, Box<dyn Error>> {
        let mut classes: Vec<u16> = Vec::with_capacity(number_of_classes as usize);
        for _ in 0..number_of_classes {
            classes.push(cursor.read_u16::<BE>()?);
        }
        assert!(classes.len() == number_of_classes as usize);
        Ok(PermittedSubclasses {
            attribute_name_index,
            attribute_length,
            number_of_classes,
            classes,
        })
    }
}

#[derive(Clone, Debug)]
pub struct Unknown {
    attribute_name_index: u16,
    attribute_length: u32,
}

pub(crate) fn read_attributes(
    constant_pool: &[crate::class_file::ConstantPool],
    attributes: &mut Vec<crate::class_file::AttributeInfo>,
    cursor: &mut Cursor<&[u8]>,
    version: Option<u16>,
) -> Result<(), Box<dyn Error>> {
    let size = attributes.capacity();
    for _ in 0..size {
        let name_index = cursor.read_u16::<BE>()?;
        let name = &constant_pool[name_index as usize];
        let length = cursor.read_u32::<BE>()?;
        if let ConstantPool::Utf8(n) = name {
            // println!("{} begins at {:#04X?}", n.get_string(), cursor.position() - 6);
            let attribute = match n.get_string().as_str() {
                "ConstantValue" => AttributeInfo::ConstantValue(ConstantValue::new(
                    name_index,
                    length,
                    cursor.read_u16::<BE>()?,
                )),
                "Code" => AttributeInfo::Code(Code::new(
                    name_index,
                    length,
                    constant_pool,
                    cursor,
                    version.unwrap(),
                )?),
                "StackMapTable" => {
                    AttributeInfo::StackMapTable(StackMapTable::new(name_index, length, cursor)?)
                }
                "Exceptions" => AttributeInfo::Exceptions(Exceptions::new(
                    name_index,
                    length,
                    constant_pool,
                    cursor,
                )?),
                "InnerClasses" => AttributeInfo::InnerClasses(InnerClasses::new(
                    name_index,
                    length,
                    constant_pool,
                    cursor,
                )?),
                "EnclosingMethod" => AttributeInfo::EnclosingMethod(EnclosingMethod::new(
                    name_index,
                    length,
                    cursor.read_u16::<BE>()?,
                    cursor.read_u16::<BE>()?,
                )?),
                "Synthetic" => AttributeInfo::Synthetic(Synthetic::new(name_index, length)?),
                "Signature" => AttributeInfo::Signature(Signature::new(
                    name_index,
                    length,
                    cursor.read_u16::<BE>()?,
                )?),
                "SourceFile" => AttributeInfo::SourceFile(SourceFile::new(
                    name_index,
                    length,
                    cursor.read_u16::<BE>()?,
                )?),
                "SourceDebugExtension" => AttributeInfo::SourceDebugExtension(
                    SourceDebugExtension::new(name_index, length, cursor)?,
                ),
                "LineNumberTable" => AttributeInfo::LineNumberTable(LineNumberTable::new(
                    name_index,
                    length,
                    cursor.read_u16::<BE>()?,
                    cursor,
                )?),
                "LocalVariableTable" => AttributeInfo::LocalVariableTable(LocalVariableTable::new(
                    name_index,
                    length,
                    cursor.read_u16::<BE>()?,
                    cursor,
                )?),
                "LocalVariableTypeTable" => {
                    AttributeInfo::LocalVariableTypeTable(LocalVariableTypeTable::new(
                        name_index,
                        length,
                        cursor.read_u16::<BE>()?,
                        cursor,
                    )?)
                }
                "Deprecated" => AttributeInfo::Deprecated(Deprecated::new(name_index, length)?),
                "RuntimeVisibleAnnotations" => {
                    AttributeInfo::RuntimeVisibleAnnotations(RuntimeVisibleAnnotations::new(
                        name_index,
                        length,
                        cursor.read_u16::<BE>()?,
                        cursor,
                    )?)
                }
                "RuntimeInvisibleAnnotations" => {
                    AttributeInfo::RuntimeInvisibleAnnotations(RuntimeInvisibleAnnotations::new(
                        name_index,
                        length,
                        cursor.read_u16::<BE>()?,
                        cursor,
                    )?)
                }
                "RuntimeVisibleParameterAnnotations" => {
                    AttributeInfo::RuntimeVisibleParameterAnnotations(
                        RuntimeVisibleParameterAnnotations::new(
                            name_index,
                            length,
                            cursor.read_u8()?,
                            cursor,
                        )?,
                    )
                }
                "RuntimeInvisibleParameterAnnotations" => {
                    AttributeInfo::RuntimeInvisibleParameterAnnotations(
                        RuntimeInvisibleParameterAnnotations::new(
                            name_index,
                            length,
                            cursor.read_u8()?,
                            cursor,
                        )?,
                    )
                }
                "RuntimeVisibleTypeAnnotations" => AttributeInfo::RuntimeVisibleTypeAnnotations(
                    RuntimeVisibleTypeAnnotations::new(
                        name_index,
                        length,
                        cursor.read_u16::<BE>()?,
                        cursor,
                    )?,
                ),
                "RuntimeInvisibleTypeAnnotations" => {
                    AttributeInfo::RuntimeInvisibleTypeAnnotations(
                        RuntimeInvisibleTypeAnnotations::new(
                            name_index,
                            length,
                            cursor.read_u16::<BE>()?,
                            cursor,
                        )?,
                    )
                }
                "AnnotationDefault" => AttributeInfo::AnnotationDefault(AnnotationDefault::new(
                    name_index, length, cursor,
                )?),
                "BootstrapMethods" => AttributeInfo::BootstrapMethods(BootstrapMethods::new(
                    name_index,
                    length,
                    cursor.read_u16::<BE>()?,
                    cursor,
                )?),
                "MethodParameters" => AttributeInfo::MethodParameters(MethodParameters::new(
                    name_index,
                    length,
                    cursor.read_u8()?,
                    cursor,
                )?),
                "Module" => AttributeInfo::Module(Module::new(name_index, length, cursor)?),
                "ModulePackages" => AttributeInfo::ModulePackages(ModulePackages::new(
                    name_index,
                    length,
                    cursor.read_u16::<BE>()?,
                    cursor,
                )?),
                "ModuleMainClass" => AttributeInfo::ModuleMainClass(ModuleMainClass::new(
                    name_index,
                    length,
                    cursor.read_u16::<BE>()?,
                )?),
                "NestHost" => AttributeInfo::NestHost(NestHost::new(
                    name_index,
                    length,
                    cursor.read_u16::<BE>()?,
                )?),
                "NestMembers" => AttributeInfo::NestMembers(NestMembers::new(
                    name_index,
                    length,
                    cursor.read_u16::<BE>()?,
                    cursor,
                )?),
                "Record" => AttributeInfo::Record(Record::new(
                    name_index,
                    length,
                    cursor.read_u16::<BE>()?,
                    constant_pool,
                    version,
                    cursor,
                )?),
                "PermittedSubclasses" => AttributeInfo::PermittedSubclasses(
                    PermittedSubclasses::new(name_index, length, cursor.read_u16::<BE>()?, cursor)?,
                ),
                _ => {
                    cursor.set_position(cursor.position() + length as u64);
                    AttributeInfo::Unknown(n.get_string())
                }
            };
            attributes.push(attribute);
        } else {
            return Err(Box::new(LoadingError::new(
                LoadingCause::InvalidAttributeNameIndex(name.clone()),
                &format!("Cursor Position: {:#04X?}", cursor.position() - 2),
            )));
        }
    }

    Ok(())
}
