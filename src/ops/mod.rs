pub mod mnemonics;

use std::{
    error::Error,
    io::Cursor,
    ops::Index,
    path::Path,
    sync::{Arc, Mutex},
};

use crate::{
    runtime_pool::SymbolicRef,
    vm::{FrameValues, VM},
};
use crate::{
    runtime_pool::{self, RuntimeConstant},
    stack_frame::StackFrame,
};
use byteorder::ReadBytesExt;
use jloader::{
    attributes::AttributeInfo,
    class_file::ClassLoc,
    constants::{self, PoolConstants},
    descriptors::FieldDescriptor,
};
use mnemonics::Mnemonic;

#[derive(Debug)]
pub enum OperandType {
    PoolIndex(u8),
    VarIndex(u8),
    Offset(u8),
    Immediate(u8),
}

#[derive(Debug)]
pub struct Instruction {
    mnemonic: Mnemonic,
    const_operands: Vec<OperandType>,
}

impl Instruction {
    pub fn from_frame(
        frame: &mut StackFrame,
    ) -> Result<Instruction, Box<dyn std::error::Error>> {
        let mut pc_opt = frame.pc.as_mut();
        let Some(mut pc) = pc_opt else {
            panic!("Program Counter was None");
        };
        let mnemonic = Mnemonic::from(frame.code[*pc as usize]);
        let result = match mnemonic {
            Mnemonic::Aaload => Instruction {
                mnemonic: Mnemonic::Aaload,
                const_operands: vec![],
            },
            Mnemonic::Aastore => Instruction {
                mnemonic: Mnemonic::Aastore,
                const_operands: vec![],
            },
            Mnemonic::AconstNull => Instruction {
                mnemonic: Mnemonic::AconstNull,
                const_operands: vec![],
            },
            Mnemonic::Aload => Instruction {
                mnemonic: Mnemonic::Aload,
                const_operands: vec![OperandType::VarIndex(get_operand(frame))],
            },
            Mnemonic::Aload0 => Instruction {
                mnemonic: Mnemonic::Aload0,
                const_operands: vec![],
            },
            Mnemonic::Aload1 => Instruction {
                mnemonic: Mnemonic::Aload1,
                const_operands: vec![],
            },
            Mnemonic::Aload2 => Instruction {
                mnemonic: Mnemonic::Aload2,
                const_operands: vec![],
            },
            Mnemonic::Aload3 => Instruction {
                mnemonic: Mnemonic::Aload3,
                const_operands: vec![],
            },
            Mnemonic::Anewarray => Instruction {
                mnemonic: Mnemonic::Anewarray,
                const_operands: vec![
                    OperandType::PoolIndex(get_operand(frame)),
                    OperandType::PoolIndex(get_operand(frame)),
                ],
            },
            Mnemonic::Areturn => Instruction {
                mnemonic: Mnemonic::Areturn,
                const_operands: vec![],
            },
            Mnemonic::Arraylength => Instruction {
                mnemonic: Mnemonic::Arraylength,
                const_operands: vec![],
            },
            Mnemonic::Astore => Instruction {
                mnemonic: Mnemonic::Astore,
                const_operands: vec![OperandType::VarIndex(get_operand(frame))],
            },
            Mnemonic::Astore0 => Instruction {
                mnemonic: Mnemonic::Astore0,
                const_operands: vec![],
            },
            Mnemonic::Astore1 => Instruction {
                mnemonic: Mnemonic::Astore1,
                const_operands: vec![],
            },
            Mnemonic::Astore2 => Instruction {
                mnemonic: Mnemonic::Astore2,
                const_operands: vec![],
            },
            Mnemonic::Astore3 => Instruction {
                mnemonic: Mnemonic::Astore3,
                const_operands: vec![],
            },
            Mnemonic::Athrow => Instruction {
                mnemonic: Mnemonic::Athrow,
                const_operands: vec![],
            },
            Mnemonic::Baload => Instruction {
                mnemonic: Mnemonic::Baload,
                const_operands: vec![],
            },
            Mnemonic::Bastore => Instruction {
                mnemonic: Mnemonic::Bastore,
                const_operands: vec![],
            },
            Mnemonic::Bipush => Instruction {
                mnemonic: Mnemonic::Bipush,
                const_operands: vec![OperandType::Immediate(get_operand(
                    frame,
                ))],
            },
            Mnemonic::Caload => Instruction {
                mnemonic: Mnemonic::Caload,
                const_operands: vec![],
            },
            Mnemonic::Castore => Instruction {
                mnemonic: Mnemonic::Castore,
                const_operands: vec![],
            },
            Mnemonic::Checkcast => Instruction {
                mnemonic: Mnemonic::Checkcast,
                const_operands: vec![
                    OperandType::PoolIndex(get_operand(frame)),
                    OperandType::PoolIndex(get_operand(frame)),
                ],
            },
            Mnemonic::D2f => Instruction {
                mnemonic: Mnemonic::D2f,
                const_operands: vec![],
            },
            Mnemonic::D2i => Instruction {
                mnemonic: Mnemonic::D2i,
                const_operands: vec![],
            },
            Mnemonic::D2l => Instruction {
                mnemonic: Mnemonic::D2l,
                const_operands: vec![],
            },
            Mnemonic::Dadd => Instruction {
                mnemonic: Mnemonic::Dadd,
                const_operands: vec![],
            },
            Mnemonic::Daload => Instruction {
                mnemonic: Mnemonic::Daload,
                const_operands: vec![],
            },
            Mnemonic::Dastore => Instruction {
                mnemonic: Mnemonic::Dastore,
                const_operands: vec![],
            },
            Mnemonic::Dcmpg => Instruction {
                mnemonic: Mnemonic::Dcmpg,
                const_operands: vec![],
            },
            Mnemonic::Dcmpl => Instruction {
                mnemonic: Mnemonic::Dcmpl,
                const_operands: vec![],
            },
            Mnemonic::Dconst0 => Instruction {
                mnemonic: Mnemonic::Dconst0,
                const_operands: vec![],
            },
            Mnemonic::Dconst1 => Instruction {
                mnemonic: Mnemonic::Dconst1,
                const_operands: vec![],
            },
            Mnemonic::Ddiv => Instruction {
                mnemonic: Mnemonic::Ddiv,
                const_operands: vec![],
            },
            Mnemonic::Dload => Instruction {
                mnemonic: Mnemonic::Dload,
                const_operands: vec![OperandType::Immediate(get_operand(
                    frame,
                ))],
            },
            Mnemonic::Dload0 => Instruction {
                mnemonic: Mnemonic::Dload0,
                const_operands: vec![],
            },
            Mnemonic::Dload1 => Instruction {
                mnemonic: Mnemonic::Dload1,
                const_operands: vec![],
            },
            Mnemonic::Dload2 => Instruction {
                mnemonic: Mnemonic::Dload2,
                const_operands: vec![],
            },
            Mnemonic::Dload3 => Instruction {
                mnemonic: Mnemonic::Dload3,
                const_operands: vec![],
            },
            Mnemonic::Dmul => Instruction {
                mnemonic: Mnemonic::Dmul,
                const_operands: vec![],
            },
            Mnemonic::Dneg => Instruction {
                mnemonic: Mnemonic::Dneg,
                const_operands: vec![],
            },
            Mnemonic::Drem => Instruction {
                mnemonic: Mnemonic::Drem,
                const_operands: vec![],
            },
            Mnemonic::Dreturn => Instruction {
                mnemonic: Mnemonic::Dreturn,
                const_operands: vec![],
            },
            Mnemonic::Dstore => Instruction {
                mnemonic: Mnemonic::Dstore,
                const_operands: vec![OperandType::VarIndex(get_operand(frame))],
            },
            Mnemonic::Dstore0 => Instruction {
                mnemonic: Mnemonic::Dstore0,
                const_operands: vec![],
            },
            Mnemonic::Dstore1 => Instruction {
                mnemonic: Mnemonic::Dstore1,
                const_operands: vec![],
            },
            Mnemonic::Dstore2 => Instruction {
                mnemonic: Mnemonic::Dstore2,
                const_operands: vec![],
            },
            Mnemonic::Dstore3 => Instruction {
                mnemonic: Mnemonic::Dstore3,
                const_operands: vec![],
            },
            Mnemonic::Dsub => Instruction {
                mnemonic: Mnemonic::Dsub,
                const_operands: vec![],
            },
            Mnemonic::Dup => Instruction {
                mnemonic: Mnemonic::Dup,
                const_operands: vec![],
            },
            Mnemonic::DupX1 => Instruction {
                mnemonic: Mnemonic::DupX1,
                const_operands: vec![],
            },
            Mnemonic::DupX2 => Instruction {
                mnemonic: Mnemonic::DupX2,
                const_operands: vec![],
            },
            Mnemonic::Dup2 => Instruction {
                mnemonic: Mnemonic::Dup2,
                const_operands: vec![],
            },
            Mnemonic::Dup2X1 => Instruction {
                mnemonic: Mnemonic::Dup2X1,
                const_operands: vec![],
            },
            Mnemonic::Dup2X2 => Instruction {
                mnemonic: Mnemonic::Dup2X2,
                const_operands: vec![],
            },
            Mnemonic::F2d => Instruction {
                mnemonic: Mnemonic::F2d,
                const_operands: vec![],
            },
            Mnemonic::F2i => Instruction {
                mnemonic: Mnemonic::F2i,
                const_operands: vec![],
            },
            Mnemonic::F2l => Instruction {
                mnemonic: Mnemonic::F2l,
                const_operands: vec![],
            },
            Mnemonic::Fadd => Instruction {
                mnemonic: Mnemonic::Fadd,
                const_operands: vec![],
            },
            Mnemonic::Faload => Instruction {
                mnemonic: Mnemonic::Faload,
                const_operands: vec![],
            },
            Mnemonic::Fastore => Instruction {
                mnemonic: Mnemonic::Fastore,
                const_operands: vec![],
            },
            Mnemonic::Fcmpg => Instruction {
                mnemonic: Mnemonic::Fcmpg,
                const_operands: vec![],
            },
            Mnemonic::Fcmpl => Instruction {
                mnemonic: Mnemonic::Fcmpl,
                const_operands: vec![],
            },
            Mnemonic::Fconst0 => Instruction {
                mnemonic: Mnemonic::Fconst0,
                const_operands: vec![],
            },
            Mnemonic::Fconst1 => Instruction {
                mnemonic: Mnemonic::Fconst1,
                const_operands: vec![],
            },
            Mnemonic::Fconst2 => Instruction {
                mnemonic: Mnemonic::Fconst2,
                const_operands: vec![],
            },
            Mnemonic::Fdiv => Instruction {
                mnemonic: Mnemonic::Fdiv,
                const_operands: vec![],
            },
            Mnemonic::Fload => Instruction {
                mnemonic: Mnemonic::Fload,
                const_operands: vec![OperandType::VarIndex(get_operand(frame))],
            },
            Mnemonic::Fload0 => Instruction {
                mnemonic: Mnemonic::Fload0,
                const_operands: vec![],
            },
            Mnemonic::Fload1 => Instruction {
                mnemonic: Mnemonic::Fload1,
                const_operands: vec![],
            },
            Mnemonic::Fload2 => Instruction {
                mnemonic: Mnemonic::Fload2,
                const_operands: vec![],
            },
            Mnemonic::Fload3 => Instruction {
                mnemonic: Mnemonic::Fload3,
                const_operands: vec![],
            },
            Mnemonic::Fmul => Instruction {
                mnemonic: Mnemonic::Fmul,
                const_operands: vec![],
            },
            Mnemonic::Fneg => Instruction {
                mnemonic: Mnemonic::Fneg,
                const_operands: vec![],
            },
            Mnemonic::Frem => Instruction {
                mnemonic: Mnemonic::Frem,
                const_operands: vec![],
            },
            Mnemonic::Freturn => Instruction {
                mnemonic: Mnemonic::Freturn,
                const_operands: vec![],
            },
            Mnemonic::Fstore => Instruction {
                mnemonic: Mnemonic::Fstore,
                const_operands: vec![OperandType::VarIndex(get_operand(frame))],
            },
            Mnemonic::Fstore0 => Instruction {
                mnemonic: Mnemonic::Fstore0,
                const_operands: vec![],
            },
            Mnemonic::Fstore1 => Instruction {
                mnemonic: Mnemonic::Fstore1,
                const_operands: vec![],
            },
            Mnemonic::Fstore2 => Instruction {
                mnemonic: Mnemonic::Fstore2,
                const_operands: vec![],
            },
            Mnemonic::Fstore3 => Instruction {
                mnemonic: Mnemonic::Fstore3,
                const_operands: vec![],
            },
            Mnemonic::Fsub => Instruction {
                mnemonic: Mnemonic::Fsub,
                const_operands: vec![],
            },
            Mnemonic::Getfield => Instruction {
                mnemonic: Mnemonic::Getfield,
                const_operands: vec![
                    OperandType::PoolIndex(get_operand(frame)),
                    OperandType::PoolIndex(get_operand(frame)),
                ],
            },
            Mnemonic::Getstatic => Instruction {
                mnemonic: Mnemonic::Getstatic,
                const_operands: vec![
                    OperandType::PoolIndex(get_operand(frame)),
                    OperandType::PoolIndex(get_operand(frame)),
                ],
            },
            Mnemonic::Goto => Instruction {
                mnemonic: Mnemonic::Goto,
                const_operands: vec![
                    OperandType::Offset(get_operand(frame)),
                    OperandType::Offset(get_operand(frame)),
                ],
            },
            Mnemonic::GotoW => Instruction {
                mnemonic: Mnemonic::GotoW,
                const_operands: vec![
                    OperandType::Offset(get_operand(frame)),
                    OperandType::Offset(get_operand(frame)),
                    OperandType::Offset(get_operand(frame)),
                    OperandType::Offset(get_operand(frame)),
                ],
            },
            Mnemonic::I2b => Instruction {
                mnemonic: Mnemonic::I2b,
                const_operands: vec![],
            },
            Mnemonic::I2c => Instruction {
                mnemonic: Mnemonic::I2c,
                const_operands: vec![],
            },
            Mnemonic::I2d => Instruction {
                mnemonic: Mnemonic::I2d,
                const_operands: vec![],
            },
            Mnemonic::I2f => Instruction {
                mnemonic: Mnemonic::I2f,
                const_operands: vec![],
            },
            Mnemonic::I2l => Instruction {
                mnemonic: Mnemonic::I2l,
                const_operands: vec![],
            },
            Mnemonic::I2s => Instruction {
                mnemonic: Mnemonic::I2s,
                const_operands: vec![],
            },
            Mnemonic::Iadd => Instruction {
                mnemonic: Mnemonic::Iadd,
                const_operands: vec![],
            },
            Mnemonic::Iaload => Instruction {
                mnemonic: Mnemonic::Iaload,
                const_operands: vec![],
            },
            Mnemonic::Iand => Instruction {
                mnemonic: Mnemonic::Iand,
                const_operands: vec![],
            },
            Mnemonic::Iastore => Instruction {
                mnemonic: Mnemonic::Iastore,
                const_operands: vec![],
            },
            Mnemonic::IconstM1 => Instruction {
                mnemonic: Mnemonic::IconstM1,
                const_operands: vec![],
            },
            Mnemonic::Iconst0 => Instruction {
                mnemonic: Mnemonic::Iconst0,
                const_operands: vec![],
            },
            Mnemonic::Iconst1 => Instruction {
                mnemonic: Mnemonic::Iconst1,
                const_operands: vec![],
            },
            Mnemonic::Iconst2 => Instruction {
                mnemonic: Mnemonic::Iconst2,
                const_operands: vec![],
            },
            Mnemonic::Iconst3 => Instruction {
                mnemonic: Mnemonic::Iconst3,
                const_operands: vec![],
            },
            Mnemonic::Iconst4 => Instruction {
                mnemonic: Mnemonic::Iconst4,
                const_operands: vec![],
            },
            Mnemonic::Iconst5 => Instruction {
                mnemonic: Mnemonic::Iconst5,
                const_operands: vec![],
            },
            Mnemonic::Idiv => Instruction {
                mnemonic: Mnemonic::Idiv,
                const_operands: vec![],
            },
            Mnemonic::IfAcmpeq => Instruction {
                mnemonic: Mnemonic::IfAcmpeq,
                const_operands: vec![
                    OperandType::Offset(get_operand(frame)),
                    OperandType::Offset(get_operand(frame)),
                ],
            },
            Mnemonic::IfAcmpne => Instruction {
                mnemonic: Mnemonic::IfAcmpne,
                const_operands: vec![
                    OperandType::Offset(get_operand(frame)),
                    OperandType::Offset(get_operand(frame)),
                ],
            },
            Mnemonic::IfIcmpeq => Instruction {
                mnemonic: Mnemonic::IfIcmpeq,
                const_operands: vec![
                    OperandType::Offset(get_operand(frame)),
                    OperandType::Offset(get_operand(frame)),
                ],
            },
            Mnemonic::IfIcmpne => Instruction {
                mnemonic: Mnemonic::IfIcmpne,
                const_operands: vec![
                    OperandType::Offset(get_operand(frame)),
                    OperandType::Offset(get_operand(frame)),
                ],
            },
            Mnemonic::IfIcmplt => Instruction {
                mnemonic: Mnemonic::IfIcmplt,
                const_operands: vec![
                    OperandType::Offset(get_operand(frame)),
                    OperandType::Offset(get_operand(frame)),
                ],
            },
            Mnemonic::IfIcmpge => Instruction {
                mnemonic: Mnemonic::IfIcmpge,
                const_operands: vec![
                    OperandType::Offset(get_operand(frame)),
                    OperandType::Offset(get_operand(frame)),
                ],
            },
            Mnemonic::IfIcmpgt => Instruction {
                mnemonic: Mnemonic::IfIcmpgt,
                const_operands: vec![
                    OperandType::Offset(get_operand(frame)),
                    OperandType::Offset(get_operand(frame)),
                ],
            },
            Mnemonic::IfIcmple => Instruction {
                mnemonic: Mnemonic::IfIcmple,
                const_operands: vec![
                    OperandType::Offset(get_operand(frame)),
                    OperandType::Offset(get_operand(frame)),
                ],
            },
            Mnemonic::Ifeq => Instruction {
                mnemonic: Mnemonic::Ifeq,
                const_operands: vec![
                    OperandType::Offset(get_operand(frame)),
                    OperandType::Offset(get_operand(frame)),
                ],
            },
            Mnemonic::Ifne => Instruction {
                mnemonic: Mnemonic::Ifne,
                const_operands: vec![
                    OperandType::Offset(get_operand(frame)),
                    OperandType::Offset(get_operand(frame)),
                ],
            },
            Mnemonic::Iflt => Instruction {
                mnemonic: Mnemonic::Iflt,
                const_operands: vec![
                    OperandType::Offset(get_operand(frame)),
                    OperandType::Offset(get_operand(frame)),
                ],
            },
            Mnemonic::Ifge => Instruction {
                mnemonic: Mnemonic::Ifge,
                const_operands: vec![
                    OperandType::Offset(get_operand(frame)),
                    OperandType::Offset(get_operand(frame)),
                ],
            },
            Mnemonic::Ifgt => Instruction {
                mnemonic: Mnemonic::Ifgt,
                const_operands: vec![
                    OperandType::Offset(get_operand(frame)),
                    OperandType::Offset(get_operand(frame)),
                ],
            },
            Mnemonic::Ifle => Instruction {
                mnemonic: Mnemonic::Ifle,
                const_operands: vec![
                    OperandType::Offset(get_operand(frame)),
                    OperandType::Offset(get_operand(frame)),
                ],
            },
            Mnemonic::Ifnonnull => Instruction {
                mnemonic: Mnemonic::Ifnonnull,
                const_operands: vec![
                    OperandType::Offset(get_operand(frame)),
                    OperandType::Offset(get_operand(frame)),
                ],
            },
            Mnemonic::Ifnull => Instruction {
                mnemonic: Mnemonic::Ifnull,
                const_operands: vec![
                    OperandType::Offset(get_operand(frame)),
                    OperandType::Offset(get_operand(frame)),
                ],
            },
            Mnemonic::Iinc => Instruction {
                mnemonic: Mnemonic::Iinc,
                const_operands: vec![
                    OperandType::VarIndex(get_operand(frame)),
                    OperandType::Immediate(get_operand(frame)),
                ],
            },
            Mnemonic::Iload => Instruction {
                mnemonic: Mnemonic::Iload,
                const_operands: vec![OperandType::VarIndex(get_operand(frame))],
            },
            Mnemonic::Iload0 => Instruction {
                mnemonic: Mnemonic::Iload0,
                const_operands: vec![],
            },
            Mnemonic::Iload1 => Instruction {
                mnemonic: Mnemonic::Iload1,
                const_operands: vec![],
            },
            Mnemonic::Iload2 => Instruction {
                mnemonic: Mnemonic::Iload2,
                const_operands: vec![],
            },
            Mnemonic::Iload3 => Instruction {
                mnemonic: Mnemonic::Iload3,
                const_operands: vec![],
            },
            Mnemonic::Imul => Instruction {
                mnemonic: Mnemonic::Imul,
                const_operands: vec![],
            },
            Mnemonic::Ineg => Instruction {
                mnemonic: Mnemonic::Ineg,
                const_operands: vec![],
            },
            Mnemonic::Instanceof => Instruction {
                mnemonic: Mnemonic::Instanceof,
                const_operands: vec![
                    OperandType::PoolIndex(get_operand(frame)),
                    OperandType::PoolIndex(get_operand(frame)),
                ],
            },
            Mnemonic::Invokedynamic => Instruction {
                mnemonic: Mnemonic::Invokedynamic,
                const_operands: vec![
                    OperandType::PoolIndex(get_operand(frame)),
                    OperandType::PoolIndex(get_operand(frame)),
                    OperandType::Immediate(get_operand(frame)),
                    OperandType::Immediate(get_operand(frame)),
                ],
            },
            Mnemonic::Invokeinterface => Instruction {
                mnemonic: Mnemonic::Invokeinterface,
                const_operands: vec![
                    OperandType::PoolIndex(get_operand(frame)),
                    OperandType::PoolIndex(get_operand(frame)),
                    OperandType::Immediate(get_operand(frame)),
                    OperandType::Immediate(get_operand(frame)),
                ],
            },
            Mnemonic::Invokespecial => Instruction {
                mnemonic: Mnemonic::Invokespecial,
                const_operands: vec![
                    OperandType::PoolIndex(get_operand(frame)),
                    OperandType::PoolIndex(get_operand(frame)),
                ],
            },
            Mnemonic::Invokestatic => Instruction {
                mnemonic: Mnemonic::Invokestatic,
                const_operands: vec![
                    OperandType::PoolIndex(get_operand(frame)),
                    OperandType::PoolIndex(get_operand(frame)),
                ],
            },
            Mnemonic::Invokevirtual => Instruction {
                mnemonic: Mnemonic::Invokevirtual,
                const_operands: vec![
                    OperandType::PoolIndex(get_operand(frame)),
                    OperandType::PoolIndex(get_operand(frame)),
                ],
            },
            Mnemonic::Ior => Instruction {
                mnemonic: Mnemonic::Ior,
                const_operands: vec![],
            },
            Mnemonic::Irem => Instruction {
                mnemonic: Mnemonic::Irem,
                const_operands: vec![],
            },
            Mnemonic::Ireturn => Instruction {
                mnemonic: Mnemonic::Ireturn,
                const_operands: vec![],
            },
            Mnemonic::Ishl => Instruction {
                mnemonic: Mnemonic::Ishl,
                const_operands: vec![],
            },
            Mnemonic::Ishr => Instruction {
                mnemonic: Mnemonic::Ishr,
                const_operands: vec![],
            },
            Mnemonic::Istore => Instruction {
                mnemonic: Mnemonic::Istore,
                const_operands: vec![OperandType::VarIndex(get_operand(frame))],
            },
            Mnemonic::Istore0 => Instruction {
                mnemonic: Mnemonic::Istore0,
                const_operands: vec![],
            },
            Mnemonic::Istore1 => Instruction {
                mnemonic: Mnemonic::Istore1,
                const_operands: vec![],
            },
            Mnemonic::Istore2 => Instruction {
                mnemonic: Mnemonic::Istore2,
                const_operands: vec![],
            },
            Mnemonic::Istore3 => Instruction {
                mnemonic: Mnemonic::Istore3,
                const_operands: vec![],
            },
            Mnemonic::Isub => Instruction {
                mnemonic: Mnemonic::Isub,
                const_operands: vec![],
            },
            Mnemonic::Iushr => Instruction {
                mnemonic: Mnemonic::Iushr,
                const_operands: vec![],
            },
            Mnemonic::Ixor => Instruction {
                mnemonic: Mnemonic::Ixor,
                const_operands: vec![],
            },
            Mnemonic::Jsr => Instruction {
                mnemonic: Mnemonic::Jsr,
                const_operands: vec![
                    OperandType::Offset(get_operand(frame)),
                    OperandType::Offset(get_operand(frame)),
                ],
            },
            Mnemonic::JsrW => Instruction {
                mnemonic: Mnemonic::JsrW,
                const_operands: vec![
                    OperandType::Offset(get_operand(frame)),
                    OperandType::Offset(get_operand(frame)),
                    OperandType::Offset(get_operand(frame)),
                    OperandType::Offset(get_operand(frame)),
                ],
            },
            Mnemonic::L2d => Instruction {
                mnemonic: Mnemonic::L2d,
                const_operands: vec![],
            },
            Mnemonic::L2f => Instruction {
                mnemonic: Mnemonic::L2f,
                const_operands: vec![],
            },
            Mnemonic::L2i => Instruction {
                mnemonic: Mnemonic::L2i,
                const_operands: vec![],
            },
            Mnemonic::Ladd => Instruction {
                mnemonic: Mnemonic::Ladd,
                const_operands: vec![],
            },
            Mnemonic::Laload => Instruction {
                mnemonic: Mnemonic::Laload,
                const_operands: vec![],
            },
            Mnemonic::Land => Instruction {
                mnemonic: Mnemonic::Land,
                const_operands: vec![],
            },
            Mnemonic::Lastore => Instruction {
                mnemonic: Mnemonic::Lastore,
                const_operands: vec![],
            },
            Mnemonic::Lcmp => Instruction {
                mnemonic: Mnemonic::Lcmp,
                const_operands: vec![],
            },
            Mnemonic::Lconst0 => Instruction {
                mnemonic: Mnemonic::Lconst0,
                const_operands: vec![],
            },
            Mnemonic::Lconst1 => Instruction {
                mnemonic: Mnemonic::Lconst1,
                const_operands: vec![],
            },
            Mnemonic::Ldc => Instruction {
                mnemonic: Mnemonic::Ldc,
                const_operands: vec![OperandType::PoolIndex(get_operand(
                    frame,
                ))],
            },
            Mnemonic::LdcW => Instruction {
                mnemonic: Mnemonic::LdcW,
                const_operands: vec![
                    OperandType::PoolIndex(get_operand(frame)),
                    OperandType::PoolIndex(get_operand(frame)),
                ],
            },
            Mnemonic::Ldc2W => Instruction {
                mnemonic: Mnemonic::Ldc2W,
                const_operands: vec![
                    OperandType::PoolIndex(get_operand(frame)),
                    OperandType::PoolIndex(get_operand(frame)),
                ],
            },
            Mnemonic::Ldiv => Instruction {
                mnemonic: Mnemonic::Ldiv,
                const_operands: vec![],
            },
            Mnemonic::Lload => Instruction {
                mnemonic: Mnemonic::Lload,
                const_operands: vec![OperandType::VarIndex(get_operand(frame))],
            },
            Mnemonic::Lload0 => Instruction {
                mnemonic: Mnemonic::Lload0,
                const_operands: vec![],
            },
            Mnemonic::Lload1 => Instruction {
                mnemonic: Mnemonic::Lload1,
                const_operands: vec![],
            },
            Mnemonic::Lload2 => Instruction {
                mnemonic: Mnemonic::Lload2,
                const_operands: vec![],
            },
            Mnemonic::Lload3 => Instruction {
                mnemonic: Mnemonic::Lload3,
                const_operands: vec![],
            },
            Mnemonic::Lmul => Instruction {
                mnemonic: Mnemonic::Lmul,
                const_operands: vec![],
            },
            Mnemonic::Lneg => Instruction {
                mnemonic: Mnemonic::Lneg,
                const_operands: vec![],
            },
            Mnemonic::Lookupswitch => Instruction {
                mnemonic: Mnemonic::Lookupswitch,
                const_operands: vec![],
            },
            Mnemonic::Lor => Instruction {
                mnemonic: Mnemonic::Lor,
                const_operands: vec![],
            },
            Mnemonic::Lrem => Instruction {
                mnemonic: Mnemonic::Lrem,
                const_operands: vec![],
            },
            Mnemonic::Lreturn => Instruction {
                mnemonic: Mnemonic::Lreturn,
                const_operands: vec![],
            },
            Mnemonic::Lshl => Instruction {
                mnemonic: Mnemonic::Lshl,
                const_operands: vec![],
            },
            Mnemonic::Lshr => Instruction {
                mnemonic: Mnemonic::Lshr,
                const_operands: vec![],
            },
            Mnemonic::Lstore => Instruction {
                mnemonic: Mnemonic::Lstore,
                const_operands: vec![
                    OperandType::VarIndex(get_operand(frame)),
                    OperandType::VarIndex(get_operand(frame)),
                ],
            },
            Mnemonic::Lstore0 => Instruction {
                mnemonic: Mnemonic::Lstore0,
                const_operands: vec![],
            },
            Mnemonic::Lstore1 => Instruction {
                mnemonic: Mnemonic::Lstore1,
                const_operands: vec![],
            },
            Mnemonic::Lstore2 => Instruction {
                mnemonic: Mnemonic::Lstore2,
                const_operands: vec![],
            },
            Mnemonic::Lstore3 => Instruction {
                mnemonic: Mnemonic::Lstore3,
                const_operands: vec![],
            },
            Mnemonic::Lsub => Instruction {
                mnemonic: Mnemonic::Lsub,
                const_operands: vec![],
            },
            Mnemonic::Lushr => Instruction {
                mnemonic: Mnemonic::Lushr,
                const_operands: vec![],
            },
            Mnemonic::Lxor => Instruction {
                mnemonic: Mnemonic::Lxor,
                const_operands: vec![],
            },
            Mnemonic::Monitorenter => Instruction {
                mnemonic: Mnemonic::Monitorenter,
                const_operands: vec![],
            },
            Mnemonic::Monitorexit => Instruction {
                mnemonic: Mnemonic::Monitorexit,
                const_operands: vec![],
            },
            Mnemonic::Multianewarray => Instruction {
                mnemonic: Mnemonic::Multianewarray,
                // The dimensions is how many values to pull off the operand stack for countN
                const_operands: vec![
                    OperandType::PoolIndex(get_operand(frame)),
                    OperandType::PoolIndex(get_operand(frame)),
                    OperandType::Immediate(get_operand(frame)),
                ],
            },
            Mnemonic::New => Instruction {
                mnemonic: Mnemonic::New,
                const_operands: vec![
                    OperandType::PoolIndex(get_operand(frame)),
                    OperandType::PoolIndex(get_operand(frame)),
                ],
            },
            Mnemonic::Newarray => Instruction {
                mnemonic: Mnemonic::Newarray,
                const_operands: vec![OperandType::Immediate(get_operand(
                    frame,
                ))],
            },
            Mnemonic::Nop => Instruction {
                mnemonic: Mnemonic::Nop,
                const_operands: vec![],
            },
            Mnemonic::Pop => Instruction {
                mnemonic: Mnemonic::Pop,
                const_operands: vec![],
            },
            Mnemonic::Pop2 => Instruction {
                mnemonic: Mnemonic::Pop2,
                const_operands: vec![],
            },
            Mnemonic::Putfield => Instruction {
                mnemonic: Mnemonic::Putfield,
                const_operands: vec![
                    OperandType::PoolIndex(get_operand(frame)),
                    OperandType::PoolIndex(get_operand(frame)),
                ],
            },
            Mnemonic::Putstatic => Instruction {
                mnemonic: Mnemonic::Putstatic,
                const_operands: vec![
                    OperandType::PoolIndex(get_operand(frame)),
                    OperandType::PoolIndex(get_operand(frame)),
                ],
            },
            Mnemonic::Ret => Instruction {
                mnemonic: Mnemonic::Ret,
                const_operands: vec![OperandType::VarIndex(get_operand(frame))],
            },
            Mnemonic::Return => Instruction {
                mnemonic: Mnemonic::Return,
                const_operands: vec![],
            },
            Mnemonic::Saload => Instruction {
                mnemonic: Mnemonic::Saload,
                const_operands: vec![],
            },
            Mnemonic::Satore => Instruction {
                mnemonic: Mnemonic::Satore,
                const_operands: vec![],
            },
            Mnemonic::Sipush => Instruction {
                mnemonic: Mnemonic::Sipush,
                const_operands: vec![
                    OperandType::Immediate(get_operand(frame)),
                    OperandType::Immediate(get_operand(frame)),
                ],
            },
            Mnemonic::Swap => Instruction {
                mnemonic: Mnemonic::Swap,
                const_operands: vec![],
            },
            Mnemonic::Tableswitch => Instruction {
                mnemonic: Mnemonic::Tableswitch,
                // FIXME: Variable Length https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A4328%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C590%2Cnull%5D
                const_operands: vec![],
            },
            Mnemonic::WideOp => Instruction {
                mnemonic: Mnemonic::WideOp,
                const_operands: vec![
                    OperandType::Immediate(get_operand(frame)),
                    OperandType::VarIndex(get_operand(frame)),
                    OperandType::VarIndex(get_operand(frame)),
                ],
            },
            Mnemonic::WideIinc => Instruction {
                mnemonic: Mnemonic::WideIinc,
                const_operands: vec![
                    OperandType::Immediate(get_operand(frame)),
                    OperandType::VarIndex(get_operand(frame)),
                    OperandType::VarIndex(get_operand(frame)),
                    OperandType::Immediate(get_operand(frame)),
                    OperandType::Immediate(get_operand(frame)),
                ],
            },
            Mnemonic::Unknown(opcode) => {
                eprintln!(
                    "UNKNOWN INSTRUCTION {opcode} AT {}",
                    frame.pc.unwrap()
                );
                Instruction {
                    mnemonic: Mnemonic::Unknown(opcode),
                    const_operands: vec![],
                }
            }
        };
        let mut pc_opt = frame.pc.as_mut();
        let Some(mut pc) = pc_opt else {
            panic!("Program Counter was None");
        };
        println!("Program Counter is: {pc}");
        *pc += 1;
        println!("Program Counter is: {pc}");
        Ok(result)
    }
    pub fn from_mnemonic_cursor(
        mnemonic: &Mnemonic,
        cursor: &mut Cursor<&[u8]>,
    ) -> Result<Instruction, Box<dyn std::error::Error>> {
        Ok(match mnemonic {
            Mnemonic::Aaload => Instruction {
                mnemonic: Mnemonic::Aaload,
                const_operands: vec![],
            },
            Mnemonic::Aastore => Instruction {
                mnemonic: Mnemonic::Aastore,
                const_operands: vec![],
            },
            Mnemonic::AconstNull => Instruction {
                mnemonic: Mnemonic::AconstNull,
                const_operands: vec![],
            },
            Mnemonic::Aload => Instruction {
                mnemonic: Mnemonic::Aload,
                const_operands: vec![OperandType::VarIndex(cursor.read_u8()?)],
            },
            Mnemonic::Aload0 => Instruction {
                mnemonic: Mnemonic::Aload0,
                const_operands: vec![],
            },
            Mnemonic::Aload1 => Instruction {
                mnemonic: Mnemonic::Aload1,
                const_operands: vec![],
            },
            Mnemonic::Aload2 => Instruction {
                mnemonic: Mnemonic::Aload2,
                const_operands: vec![],
            },
            Mnemonic::Aload3 => Instruction {
                mnemonic: Mnemonic::Aload3,
                const_operands: vec![],
            },
            Mnemonic::Anewarray => Instruction {
                mnemonic: Mnemonic::Anewarray,
                const_operands: vec![
                    OperandType::PoolIndex(cursor.read_u8()?),
                    OperandType::PoolIndex(cursor.read_u8()?),
                ],
            },
            Mnemonic::Areturn => Instruction {
                mnemonic: Mnemonic::Areturn,
                const_operands: vec![],
            },
            Mnemonic::Arraylength => Instruction {
                mnemonic: Mnemonic::Arraylength,
                const_operands: vec![],
            },
            Mnemonic::Astore => Instruction {
                mnemonic: Mnemonic::Astore,
                const_operands: vec![OperandType::VarIndex(cursor.read_u8()?)],
            },
            Mnemonic::Astore0 => Instruction {
                mnemonic: Mnemonic::Astore0,
                const_operands: vec![],
            },
            Mnemonic::Astore1 => Instruction {
                mnemonic: Mnemonic::Astore1,
                const_operands: vec![],
            },
            Mnemonic::Astore2 => Instruction {
                mnemonic: Mnemonic::Astore2,
                const_operands: vec![],
            },
            Mnemonic::Astore3 => Instruction {
                mnemonic: Mnemonic::Astore3,
                const_operands: vec![],
            },
            Mnemonic::Athrow => Instruction {
                mnemonic: Mnemonic::Athrow,
                const_operands: vec![],
            },
            Mnemonic::Baload => Instruction {
                mnemonic: Mnemonic::Baload,
                const_operands: vec![],
            },
            Mnemonic::Bastore => Instruction {
                mnemonic: Mnemonic::Bastore,
                const_operands: vec![],
            },
            Mnemonic::Bipush => Instruction {
                mnemonic: Mnemonic::Bipush,
                const_operands: vec![OperandType::Immediate(cursor.read_u8()?)],
            },
            Mnemonic::Caload => Instruction {
                mnemonic: Mnemonic::Caload,
                const_operands: vec![],
            },
            Mnemonic::Castore => Instruction {
                mnemonic: Mnemonic::Castore,
                const_operands: vec![],
            },
            Mnemonic::Checkcast => Instruction {
                mnemonic: Mnemonic::Checkcast,
                const_operands: vec![
                    OperandType::PoolIndex(cursor.read_u8()?),
                    OperandType::PoolIndex(cursor.read_u8()?),
                ],
            },
            Mnemonic::D2f => Instruction {
                mnemonic: Mnemonic::D2f,
                const_operands: vec![],
            },
            Mnemonic::D2i => Instruction {
                mnemonic: Mnemonic::D2i,
                const_operands: vec![],
            },
            Mnemonic::D2l => Instruction {
                mnemonic: Mnemonic::D2l,
                const_operands: vec![],
            },
            Mnemonic::Dadd => Instruction {
                mnemonic: Mnemonic::Dadd,
                const_operands: vec![],
            },
            Mnemonic::Daload => Instruction {
                mnemonic: Mnemonic::Daload,
                const_operands: vec![],
            },
            Mnemonic::Dastore => Instruction {
                mnemonic: Mnemonic::Dastore,
                const_operands: vec![],
            },
            Mnemonic::Dcmpg => Instruction {
                mnemonic: Mnemonic::Dcmpg,
                const_operands: vec![],
            },
            Mnemonic::Dcmpl => Instruction {
                mnemonic: Mnemonic::Dcmpl,
                const_operands: vec![],
            },
            Mnemonic::Dconst0 => Instruction {
                mnemonic: Mnemonic::Dconst0,
                const_operands: vec![],
            },
            Mnemonic::Dconst1 => Instruction {
                mnemonic: Mnemonic::Dconst1,
                const_operands: vec![],
            },
            Mnemonic::Ddiv => Instruction {
                mnemonic: Mnemonic::Ddiv,
                const_operands: vec![],
            },
            Mnemonic::Dload => Instruction {
                mnemonic: Mnemonic::Dload,
                const_operands: vec![OperandType::Immediate(cursor.read_u8()?)],
            },
            Mnemonic::Dload0 => Instruction {
                mnemonic: Mnemonic::Dload0,
                const_operands: vec![],
            },
            Mnemonic::Dload1 => Instruction {
                mnemonic: Mnemonic::Dload1,
                const_operands: vec![],
            },
            Mnemonic::Dload2 => Instruction {
                mnemonic: Mnemonic::Dload2,
                const_operands: vec![],
            },
            Mnemonic::Dload3 => Instruction {
                mnemonic: Mnemonic::Dload3,
                const_operands: vec![],
            },
            Mnemonic::Dmul => Instruction {
                mnemonic: Mnemonic::Dmul,
                const_operands: vec![],
            },
            Mnemonic::Dneg => Instruction {
                mnemonic: Mnemonic::Dneg,
                const_operands: vec![],
            },
            Mnemonic::Drem => Instruction {
                mnemonic: Mnemonic::Drem,
                const_operands: vec![],
            },
            Mnemonic::Dreturn => Instruction {
                mnemonic: Mnemonic::Dreturn,
                const_operands: vec![],
            },
            Mnemonic::Dstore => Instruction {
                mnemonic: Mnemonic::Dstore,
                const_operands: vec![OperandType::VarIndex(cursor.read_u8()?)],
            },
            Mnemonic::Dstore0 => Instruction {
                mnemonic: Mnemonic::Dstore0,
                const_operands: vec![],
            },
            Mnemonic::Dstore1 => Instruction {
                mnemonic: Mnemonic::Dstore1,
                const_operands: vec![],
            },
            Mnemonic::Dstore2 => Instruction {
                mnemonic: Mnemonic::Dstore2,
                const_operands: vec![],
            },
            Mnemonic::Dstore3 => Instruction {
                mnemonic: Mnemonic::Dstore3,
                const_operands: vec![],
            },
            Mnemonic::Dsub => Instruction {
                mnemonic: Mnemonic::Dsub,
                const_operands: vec![],
            },
            Mnemonic::Dup => Instruction {
                mnemonic: Mnemonic::Dup,
                const_operands: vec![],
            },
            Mnemonic::DupX1 => Instruction {
                mnemonic: Mnemonic::DupX1,
                const_operands: vec![],
            },
            Mnemonic::DupX2 => Instruction {
                mnemonic: Mnemonic::DupX2,
                const_operands: vec![],
            },
            Mnemonic::Dup2 => Instruction {
                mnemonic: Mnemonic::Dup2,
                const_operands: vec![],
            },
            Mnemonic::Dup2X1 => Instruction {
                mnemonic: Mnemonic::Dup2X1,
                const_operands: vec![],
            },
            Mnemonic::Dup2X2 => Instruction {
                mnemonic: Mnemonic::Dup2X2,
                const_operands: vec![],
            },
            Mnemonic::F2d => Instruction {
                mnemonic: Mnemonic::F2d,
                const_operands: vec![],
            },
            Mnemonic::F2i => Instruction {
                mnemonic: Mnemonic::F2i,
                const_operands: vec![],
            },
            Mnemonic::F2l => Instruction {
                mnemonic: Mnemonic::F2l,
                const_operands: vec![],
            },
            Mnemonic::Fadd => Instruction {
                mnemonic: Mnemonic::Fadd,
                const_operands: vec![],
            },
            Mnemonic::Faload => Instruction {
                mnemonic: Mnemonic::Faload,
                const_operands: vec![],
            },
            Mnemonic::Fastore => Instruction {
                mnemonic: Mnemonic::Fastore,
                const_operands: vec![],
            },
            Mnemonic::Fcmpg => Instruction {
                mnemonic: Mnemonic::Fcmpg,
                const_operands: vec![],
            },
            Mnemonic::Fcmpl => Instruction {
                mnemonic: Mnemonic::Fcmpl,
                const_operands: vec![],
            },
            Mnemonic::Fconst0 => Instruction {
                mnemonic: Mnemonic::Fconst0,
                const_operands: vec![],
            },
            Mnemonic::Fconst1 => Instruction {
                mnemonic: Mnemonic::Fconst1,
                const_operands: vec![],
            },
            Mnemonic::Fconst2 => Instruction {
                mnemonic: Mnemonic::Fconst2,
                const_operands: vec![],
            },
            Mnemonic::Fdiv => Instruction {
                mnemonic: Mnemonic::Fdiv,
                const_operands: vec![],
            },
            Mnemonic::Fload => Instruction {
                mnemonic: Mnemonic::Fload,
                const_operands: vec![OperandType::VarIndex(cursor.read_u8()?)],
            },
            Mnemonic::Fload0 => Instruction {
                mnemonic: Mnemonic::Fload0,
                const_operands: vec![],
            },
            Mnemonic::Fload1 => Instruction {
                mnemonic: Mnemonic::Fload1,
                const_operands: vec![],
            },
            Mnemonic::Fload2 => Instruction {
                mnemonic: Mnemonic::Fload2,
                const_operands: vec![],
            },
            Mnemonic::Fload3 => Instruction {
                mnemonic: Mnemonic::Fload3,
                const_operands: vec![],
            },
            Mnemonic::Fmul => Instruction {
                mnemonic: Mnemonic::Fmul,
                const_operands: vec![],
            },
            Mnemonic::Fneg => Instruction {
                mnemonic: Mnemonic::Fneg,
                const_operands: vec![],
            },
            Mnemonic::Frem => Instruction {
                mnemonic: Mnemonic::Frem,
                const_operands: vec![],
            },
            Mnemonic::Freturn => Instruction {
                mnemonic: Mnemonic::Freturn,
                const_operands: vec![],
            },
            Mnemonic::Fstore => Instruction {
                mnemonic: Mnemonic::Fstore,
                const_operands: vec![OperandType::VarIndex(cursor.read_u8()?)],
            },
            Mnemonic::Fstore0 => Instruction {
                mnemonic: Mnemonic::Fstore0,
                const_operands: vec![],
            },
            Mnemonic::Fstore1 => Instruction {
                mnemonic: Mnemonic::Fstore1,
                const_operands: vec![],
            },
            Mnemonic::Fstore2 => Instruction {
                mnemonic: Mnemonic::Fstore2,
                const_operands: vec![],
            },
            Mnemonic::Fstore3 => Instruction {
                mnemonic: Mnemonic::Fstore3,
                const_operands: vec![],
            },
            Mnemonic::Fsub => Instruction {
                mnemonic: Mnemonic::Fsub,
                const_operands: vec![],
            },
            Mnemonic::Getfield => Instruction {
                mnemonic: Mnemonic::Getfield,
                const_operands: vec![
                    OperandType::PoolIndex(cursor.read_u8()?),
                    OperandType::PoolIndex(cursor.read_u8()?),
                ],
            },
            Mnemonic::Getstatic => Instruction {
                mnemonic: Mnemonic::Getstatic,
                const_operands: vec![
                    OperandType::PoolIndex(cursor.read_u8()?),
                    OperandType::PoolIndex(cursor.read_u8()?),
                ],
            },
            Mnemonic::Goto => Instruction {
                mnemonic: Mnemonic::Goto,
                const_operands: vec![
                    OperandType::Offset(cursor.read_u8()?),
                    OperandType::Offset(cursor.read_u8()?),
                ],
            },
            Mnemonic::GotoW => Instruction {
                mnemonic: Mnemonic::GotoW,
                const_operands: vec![
                    OperandType::Offset(cursor.read_u8()?),
                    OperandType::Offset(cursor.read_u8()?),
                    OperandType::Offset(cursor.read_u8()?),
                    OperandType::Offset(cursor.read_u8()?),
                ],
            },
            Mnemonic::I2b => Instruction {
                mnemonic: Mnemonic::I2b,
                const_operands: vec![],
            },
            Mnemonic::I2c => Instruction {
                mnemonic: Mnemonic::I2c,
                const_operands: vec![],
            },
            Mnemonic::I2d => Instruction {
                mnemonic: Mnemonic::I2d,
                const_operands: vec![],
            },
            Mnemonic::I2f => Instruction {
                mnemonic: Mnemonic::I2f,
                const_operands: vec![],
            },
            Mnemonic::I2l => Instruction {
                mnemonic: Mnemonic::I2l,
                const_operands: vec![],
            },
            Mnemonic::I2s => Instruction {
                mnemonic: Mnemonic::I2s,
                const_operands: vec![],
            },
            Mnemonic::Iadd => Instruction {
                mnemonic: Mnemonic::Iadd,
                const_operands: vec![],
            },
            Mnemonic::Iaload => Instruction {
                mnemonic: Mnemonic::Iaload,
                const_operands: vec![],
            },
            Mnemonic::Iand => Instruction {
                mnemonic: Mnemonic::Iand,
                const_operands: vec![],
            },
            Mnemonic::Iastore => Instruction {
                mnemonic: Mnemonic::Iastore,
                const_operands: vec![],
            },
            Mnemonic::IconstM1 => Instruction {
                mnemonic: Mnemonic::IconstM1,
                const_operands: vec![],
            },
            Mnemonic::Iconst0 => Instruction {
                mnemonic: Mnemonic::Iconst0,
                const_operands: vec![],
            },
            Mnemonic::Iconst1 => Instruction {
                mnemonic: Mnemonic::Iconst1,
                const_operands: vec![],
            },
            Mnemonic::Iconst2 => Instruction {
                mnemonic: Mnemonic::Iconst2,
                const_operands: vec![],
            },
            Mnemonic::Iconst3 => Instruction {
                mnemonic: Mnemonic::Iconst3,
                const_operands: vec![],
            },
            Mnemonic::Iconst4 => Instruction {
                mnemonic: Mnemonic::Iconst4,
                const_operands: vec![],
            },
            Mnemonic::Iconst5 => Instruction {
                mnemonic: Mnemonic::Iconst5,
                const_operands: vec![],
            },
            Mnemonic::Idiv => Instruction {
                mnemonic: Mnemonic::Idiv,
                const_operands: vec![],
            },
            Mnemonic::IfAcmpeq => Instruction {
                mnemonic: Mnemonic::IfAcmpeq,
                const_operands: vec![
                    OperandType::Offset(cursor.read_u8()?),
                    OperandType::Offset(cursor.read_u8()?),
                ],
            },
            Mnemonic::IfAcmpne => Instruction {
                mnemonic: Mnemonic::IfAcmpne,
                const_operands: vec![
                    OperandType::Offset(cursor.read_u8()?),
                    OperandType::Offset(cursor.read_u8()?),
                ],
            },
            Mnemonic::IfIcmpeq => Instruction {
                mnemonic: Mnemonic::IfIcmpeq,
                const_operands: vec![
                    OperandType::Offset(cursor.read_u8()?),
                    OperandType::Offset(cursor.read_u8()?),
                ],
            },
            Mnemonic::IfIcmpne => Instruction {
                mnemonic: Mnemonic::IfIcmpne,
                const_operands: vec![
                    OperandType::Offset(cursor.read_u8()?),
                    OperandType::Offset(cursor.read_u8()?),
                ],
            },
            Mnemonic::IfIcmplt => Instruction {
                mnemonic: Mnemonic::IfIcmplt,
                const_operands: vec![
                    OperandType::Offset(cursor.read_u8()?),
                    OperandType::Offset(cursor.read_u8()?),
                ],
            },
            Mnemonic::IfIcmpge => Instruction {
                mnemonic: Mnemonic::IfIcmpge,
                const_operands: vec![
                    OperandType::Offset(cursor.read_u8()?),
                    OperandType::Offset(cursor.read_u8()?),
                ],
            },
            Mnemonic::IfIcmpgt => Instruction {
                mnemonic: Mnemonic::IfIcmpgt,
                const_operands: vec![
                    OperandType::Offset(cursor.read_u8()?),
                    OperandType::Offset(cursor.read_u8()?),
                ],
            },
            Mnemonic::IfIcmple => Instruction {
                mnemonic: Mnemonic::IfIcmple,
                const_operands: vec![
                    OperandType::Offset(cursor.read_u8()?),
                    OperandType::Offset(cursor.read_u8()?),
                ],
            },
            Mnemonic::Ifeq => Instruction {
                mnemonic: Mnemonic::Ifeq,
                const_operands: vec![
                    OperandType::Offset(cursor.read_u8()?),
                    OperandType::Offset(cursor.read_u8()?),
                ],
            },
            Mnemonic::Ifne => Instruction {
                mnemonic: Mnemonic::Ifne,
                const_operands: vec![
                    OperandType::Offset(cursor.read_u8()?),
                    OperandType::Offset(cursor.read_u8()?),
                ],
            },
            Mnemonic::Iflt => Instruction {
                mnemonic: Mnemonic::Iflt,
                const_operands: vec![
                    OperandType::Offset(cursor.read_u8()?),
                    OperandType::Offset(cursor.read_u8()?),
                ],
            },
            Mnemonic::Ifge => Instruction {
                mnemonic: Mnemonic::Ifge,
                const_operands: vec![
                    OperandType::Offset(cursor.read_u8()?),
                    OperandType::Offset(cursor.read_u8()?),
                ],
            },
            Mnemonic::Ifgt => Instruction {
                mnemonic: Mnemonic::Ifgt,
                const_operands: vec![
                    OperandType::Offset(cursor.read_u8()?),
                    OperandType::Offset(cursor.read_u8()?),
                ],
            },
            Mnemonic::Ifle => Instruction {
                mnemonic: Mnemonic::Ifle,
                const_operands: vec![
                    OperandType::Offset(cursor.read_u8()?),
                    OperandType::Offset(cursor.read_u8()?),
                ],
            },
            Mnemonic::Ifnonnull => Instruction {
                mnemonic: Mnemonic::Ifnonnull,
                const_operands: vec![
                    OperandType::Offset(cursor.read_u8()?),
                    OperandType::Offset(cursor.read_u8()?),
                ],
            },
            Mnemonic::Ifnull => Instruction {
                mnemonic: Mnemonic::Ifnull,
                const_operands: vec![
                    OperandType::Offset(cursor.read_u8()?),
                    OperandType::Offset(cursor.read_u8()?),
                ],
            },
            Mnemonic::Iinc => Instruction {
                mnemonic: Mnemonic::Iinc,
                const_operands: vec![
                    OperandType::VarIndex(cursor.read_u8()?),
                    OperandType::Immediate(cursor.read_u8()?),
                ],
            },
            Mnemonic::Iload => Instruction {
                mnemonic: Mnemonic::Iload,
                const_operands: vec![OperandType::VarIndex(cursor.read_u8()?)],
            },
            Mnemonic::Iload0 => Instruction {
                mnemonic: Mnemonic::Iload0,
                const_operands: vec![],
            },
            Mnemonic::Iload1 => Instruction {
                mnemonic: Mnemonic::Iload1,
                const_operands: vec![],
            },
            Mnemonic::Iload2 => Instruction {
                mnemonic: Mnemonic::Iload2,
                const_operands: vec![],
            },
            Mnemonic::Iload3 => Instruction {
                mnemonic: Mnemonic::Iload3,
                const_operands: vec![],
            },
            Mnemonic::Imul => Instruction {
                mnemonic: Mnemonic::Imul,
                const_operands: vec![],
            },
            Mnemonic::Ineg => Instruction {
                mnemonic: Mnemonic::Ineg,
                const_operands: vec![],
            },
            Mnemonic::Instanceof => Instruction {
                mnemonic: Mnemonic::Instanceof,
                const_operands: vec![
                    OperandType::PoolIndex(cursor.read_u8()?),
                    OperandType::PoolIndex(cursor.read_u8()?),
                ],
            },
            Mnemonic::Invokedynamic => Instruction {
                mnemonic: Mnemonic::Invokedynamic,
                const_operands: vec![
                    OperandType::PoolIndex(cursor.read_u8()?),
                    OperandType::PoolIndex(cursor.read_u8()?),
                    OperandType::Immediate(cursor.read_u8()?),
                    OperandType::Immediate(cursor.read_u8()?),
                ],
            },
            Mnemonic::Invokeinterface => Instruction {
                mnemonic: Mnemonic::Invokeinterface,
                const_operands: vec![
                    OperandType::PoolIndex(cursor.read_u8()?),
                    OperandType::PoolIndex(cursor.read_u8()?),
                    OperandType::Immediate(cursor.read_u8()?),
                    OperandType::Immediate(cursor.read_u8()?),
                ],
            },
            Mnemonic::Invokespecial => Instruction {
                mnemonic: Mnemonic::Invokespecial,
                const_operands: vec![
                    OperandType::PoolIndex(cursor.read_u8()?),
                    OperandType::PoolIndex(cursor.read_u8()?),
                ],
            },
            Mnemonic::Invokestatic => Instruction {
                mnemonic: Mnemonic::Invokestatic,
                const_operands: vec![
                    OperandType::PoolIndex(cursor.read_u8()?),
                    OperandType::PoolIndex(cursor.read_u8()?),
                ],
            },
            Mnemonic::Invokevirtual => Instruction {
                mnemonic: Mnemonic::Invokevirtual,
                const_operands: vec![
                    OperandType::PoolIndex(cursor.read_u8()?),
                    OperandType::PoolIndex(cursor.read_u8()?),
                ],
            },
            Mnemonic::Ior => Instruction {
                mnemonic: Mnemonic::Ior,
                const_operands: vec![],
            },
            Mnemonic::Irem => Instruction {
                mnemonic: Mnemonic::Irem,
                const_operands: vec![],
            },
            Mnemonic::Ireturn => Instruction {
                mnemonic: Mnemonic::Ireturn,
                const_operands: vec![],
            },
            Mnemonic::Ishl => Instruction {
                mnemonic: Mnemonic::Ishl,
                const_operands: vec![],
            },
            Mnemonic::Ishr => Instruction {
                mnemonic: Mnemonic::Ishr,
                const_operands: vec![],
            },
            Mnemonic::Istore => Instruction {
                mnemonic: Mnemonic::Istore,
                const_operands: vec![OperandType::VarIndex(cursor.read_u8()?)],
            },
            Mnemonic::Istore0 => Instruction {
                mnemonic: Mnemonic::Istore0,
                const_operands: vec![],
            },
            Mnemonic::Istore1 => Instruction {
                mnemonic: Mnemonic::Istore1,
                const_operands: vec![],
            },
            Mnemonic::Istore2 => Instruction {
                mnemonic: Mnemonic::Istore2,
                const_operands: vec![],
            },
            Mnemonic::Istore3 => Instruction {
                mnemonic: Mnemonic::Istore3,
                const_operands: vec![],
            },
            Mnemonic::Isub => Instruction {
                mnemonic: Mnemonic::Isub,
                const_operands: vec![],
            },
            Mnemonic::Iushr => Instruction {
                mnemonic: Mnemonic::Iushr,
                const_operands: vec![],
            },
            Mnemonic::Ixor => Instruction {
                mnemonic: Mnemonic::Ixor,
                const_operands: vec![],
            },
            Mnemonic::Jsr => Instruction {
                mnemonic: Mnemonic::Jsr,
                const_operands: vec![
                    OperandType::Offset(cursor.read_u8()?),
                    OperandType::Offset(cursor.read_u8()?),
                ],
            },
            Mnemonic::JsrW => Instruction {
                mnemonic: Mnemonic::JsrW,
                const_operands: vec![
                    OperandType::Offset(cursor.read_u8()?),
                    OperandType::Offset(cursor.read_u8()?),
                    OperandType::Offset(cursor.read_u8()?),
                    OperandType::Offset(cursor.read_u8()?),
                ],
            },
            Mnemonic::L2d => Instruction {
                mnemonic: Mnemonic::L2d,
                const_operands: vec![],
            },
            Mnemonic::L2f => Instruction {
                mnemonic: Mnemonic::L2f,
                const_operands: vec![],
            },
            Mnemonic::L2i => Instruction {
                mnemonic: Mnemonic::L2i,
                const_operands: vec![],
            },
            Mnemonic::Ladd => Instruction {
                mnemonic: Mnemonic::Ladd,
                const_operands: vec![],
            },
            Mnemonic::Laload => Instruction {
                mnemonic: Mnemonic::Laload,
                const_operands: vec![],
            },
            Mnemonic::Land => Instruction {
                mnemonic: Mnemonic::Land,
                const_operands: vec![],
            },
            Mnemonic::Lastore => Instruction {
                mnemonic: Mnemonic::Lastore,
                const_operands: vec![],
            },
            Mnemonic::Lcmp => Instruction {
                mnemonic: Mnemonic::Lcmp,
                const_operands: vec![],
            },
            Mnemonic::Lconst0 => Instruction {
                mnemonic: Mnemonic::Lconst0,
                const_operands: vec![],
            },
            Mnemonic::Lconst1 => Instruction {
                mnemonic: Mnemonic::Lconst1,
                const_operands: vec![],
            },
            Mnemonic::Ldc => Instruction {
                mnemonic: Mnemonic::Ldc,
                const_operands: vec![OperandType::PoolIndex(cursor.read_u8()?)],
            },
            Mnemonic::LdcW => Instruction {
                mnemonic: Mnemonic::LdcW,
                const_operands: vec![
                    OperandType::PoolIndex(cursor.read_u8()?),
                    OperandType::PoolIndex(cursor.read_u8()?),
                ],
            },
            Mnemonic::Ldc2W => Instruction {
                mnemonic: Mnemonic::Ldc2W,
                const_operands: vec![
                    OperandType::PoolIndex(cursor.read_u8()?),
                    OperandType::PoolIndex(cursor.read_u8()?),
                ],
            },
            Mnemonic::Ldiv => Instruction {
                mnemonic: Mnemonic::Ldiv,
                const_operands: vec![],
            },
            Mnemonic::Lload => Instruction {
                mnemonic: Mnemonic::Lload,
                const_operands: vec![OperandType::VarIndex(cursor.read_u8()?)],
            },
            Mnemonic::Lload0 => Instruction {
                mnemonic: Mnemonic::Lload0,
                const_operands: vec![],
            },
            Mnemonic::Lload1 => Instruction {
                mnemonic: Mnemonic::Lload1,
                const_operands: vec![],
            },
            Mnemonic::Lload2 => Instruction {
                mnemonic: Mnemonic::Lload2,
                const_operands: vec![],
            },
            Mnemonic::Lload3 => Instruction {
                mnemonic: Mnemonic::Lload3,
                const_operands: vec![],
            },
            Mnemonic::Lmul => Instruction {
                mnemonic: Mnemonic::Lmul,
                const_operands: vec![],
            },
            Mnemonic::Lneg => Instruction {
                mnemonic: Mnemonic::Lneg,
                const_operands: vec![],
            },
            Mnemonic::Lookupswitch => Instruction {
                mnemonic: Mnemonic::Lookupswitch,
                const_operands: vec![],
            },
            Mnemonic::Lor => Instruction {
                mnemonic: Mnemonic::Lor,
                const_operands: vec![],
            },
            Mnemonic::Lrem => Instruction {
                mnemonic: Mnemonic::Lrem,
                const_operands: vec![],
            },
            Mnemonic::Lreturn => Instruction {
                mnemonic: Mnemonic::Lreturn,
                const_operands: vec![],
            },
            Mnemonic::Lshl => Instruction {
                mnemonic: Mnemonic::Lshl,
                const_operands: vec![],
            },
            Mnemonic::Lshr => Instruction {
                mnemonic: Mnemonic::Lshr,
                const_operands: vec![],
            },
            Mnemonic::Lstore => Instruction {
                mnemonic: Mnemonic::Lstore,
                const_operands: vec![
                    OperandType::VarIndex(cursor.read_u8()?),
                    OperandType::VarIndex(cursor.read_u8()?),
                ],
            },
            Mnemonic::Lstore0 => Instruction {
                mnemonic: Mnemonic::Lstore0,
                const_operands: vec![],
            },
            Mnemonic::Lstore1 => Instruction {
                mnemonic: Mnemonic::Lstore1,
                const_operands: vec![],
            },
            Mnemonic::Lstore2 => Instruction {
                mnemonic: Mnemonic::Lstore2,
                const_operands: vec![],
            },
            Mnemonic::Lstore3 => Instruction {
                mnemonic: Mnemonic::Lstore3,
                const_operands: vec![],
            },
            Mnemonic::Lsub => Instruction {
                mnemonic: Mnemonic::Lsub,
                const_operands: vec![],
            },
            Mnemonic::Lushr => Instruction {
                mnemonic: Mnemonic::Lushr,
                const_operands: vec![],
            },
            Mnemonic::Lxor => Instruction {
                mnemonic: Mnemonic::Lxor,
                const_operands: vec![],
            },
            Mnemonic::Monitorenter => Instruction {
                mnemonic: Mnemonic::Monitorenter,
                const_operands: vec![],
            },
            Mnemonic::Monitorexit => Instruction {
                mnemonic: Mnemonic::Monitorexit,
                const_operands: vec![],
            },
            Mnemonic::Multianewarray => Instruction {
                mnemonic: Mnemonic::Multianewarray,
                // The dimensions is how many values to pull off the operand stack for countN
                const_operands: vec![
                    OperandType::PoolIndex(cursor.read_u8()?),
                    OperandType::PoolIndex(cursor.read_u8()?),
                    OperandType::Immediate(cursor.read_u8()?),
                ],
            },
            Mnemonic::New => Instruction {
                mnemonic: Mnemonic::New,
                const_operands: vec![
                    OperandType::PoolIndex(cursor.read_u8()?),
                    OperandType::PoolIndex(cursor.read_u8()?),
                ],
            },
            Mnemonic::Newarray => Instruction {
                mnemonic: Mnemonic::Newarray,
                const_operands: vec![OperandType::Immediate(cursor.read_u8()?)],
            },
            Mnemonic::Nop => Instruction {
                mnemonic: Mnemonic::Nop,
                const_operands: vec![],
            },
            Mnemonic::Pop => Instruction {
                mnemonic: Mnemonic::Pop,
                const_operands: vec![],
            },
            Mnemonic::Pop2 => Instruction {
                mnemonic: Mnemonic::Pop2,
                const_operands: vec![],
            },
            Mnemonic::Putfield => Instruction {
                mnemonic: Mnemonic::Putfield,
                const_operands: vec![
                    OperandType::PoolIndex(cursor.read_u8()?),
                    OperandType::PoolIndex(cursor.read_u8()?),
                ],
            },
            Mnemonic::Putstatic => Instruction {
                mnemonic: Mnemonic::Putstatic,
                const_operands: vec![
                    OperandType::PoolIndex(cursor.read_u8()?),
                    OperandType::PoolIndex(cursor.read_u8()?),
                ],
            },
            Mnemonic::Ret => Instruction {
                mnemonic: Mnemonic::Ret,
                const_operands: vec![OperandType::VarIndex(cursor.read_u8()?)],
            },
            Mnemonic::Return => Instruction {
                mnemonic: Mnemonic::Return,
                const_operands: vec![],
            },
            Mnemonic::Saload => Instruction {
                mnemonic: Mnemonic::Saload,
                const_operands: vec![],
            },
            Mnemonic::Satore => Instruction {
                mnemonic: Mnemonic::Satore,
                const_operands: vec![],
            },
            Mnemonic::Sipush => Instruction {
                mnemonic: Mnemonic::Sipush,
                const_operands: vec![
                    OperandType::Immediate(cursor.read_u8()?),
                    OperandType::Immediate(cursor.read_u8()?),
                ],
            },
            Mnemonic::Swap => Instruction {
                mnemonic: Mnemonic::Swap,
                const_operands: vec![],
            },
            Mnemonic::Tableswitch => Instruction {
                mnemonic: Mnemonic::Tableswitch,
                // FIXME: Variable Length https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A4328%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C590%2Cnull%5D
                const_operands: vec![],
            },
            Mnemonic::WideOp => Instruction {
                mnemonic: Mnemonic::WideOp,
                const_operands: vec![
                    OperandType::Immediate(cursor.read_u8()?),
                    OperandType::VarIndex(cursor.read_u8()?),
                    OperandType::VarIndex(cursor.read_u8()?),
                ],
            },
            Mnemonic::WideIinc => Instruction {
                mnemonic: Mnemonic::WideIinc,
                const_operands: vec![
                    OperandType::Immediate(cursor.read_u8()?),
                    OperandType::VarIndex(cursor.read_u8()?),
                    OperandType::VarIndex(cursor.read_u8()?),
                    OperandType::Immediate(cursor.read_u8()?),
                    OperandType::Immediate(cursor.read_u8()?),
                ],
            },
            Mnemonic::Unknown(opcode) => {
                eprintln!(
                    "UNKNOWN INSTRUCTION {opcode} AT {}",
                    cursor.position()
                );
                Instruction {
                    mnemonic: Mnemonic::Unknown(*opcode),
                    const_operands: vec![],
                }
            }
        })
    }

    pub fn get_const_operands(&self) -> &Vec<OperandType> {
        &self.const_operands
    }
    pub fn get_mnemonic(&self) -> &Mnemonic { &self.mnemonic }
}

fn get_operand(frame: &mut StackFrame) -> u8 {
    let Some(pc) = frame.pc.as_mut() else {
        panic!("Program Counter was None")
    };
    *pc += 1;
    frame.code[*pc as usize]
}

pub fn aaload(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn aastore(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn aconst_null(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    frame
        .stack
        .push(FrameValues::Reference(runtime_pool::SymbolicRef::Null));
}
pub fn aload(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    let OperandType::VarIndex(index) = inst.get_const_operands()[0] else {
        panic!("Operand type for aload was not a var index");
    };
    let local = frame.locals[index as usize].clone();
    if let FrameValues::Reference(_) = local {
        frame.stack.push(local);
    } else {
        panic!("Local value at [{index}] was not a reference");
    }
}
pub fn aload_0(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    let local = frame.locals[0].clone();
    if let FrameValues::Reference(_) = local {
        frame.stack.push(local);
    } else {
        panic!("Local value at [0] was not a reference");
    }
}
pub fn aload_1(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    let local = frame.locals[1].clone();
    if let FrameValues::Reference(_) = local {
        frame.stack.push(local);
    } else {
        panic!("Local value at [1] was not a reference");
    }
}
pub fn aload_2(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    let local = frame.locals[2].clone();
    if let FrameValues::Reference(_) = local {
        frame.stack.push(local);
    } else {
        panic!("Local value at [2] was not a reference");
    }
}
pub fn aload_3(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    let local = frame.locals[3].clone();
    if let FrameValues::Reference(_) = local {
        frame.stack.push(local);
    } else {
        panic!("Local value at [3] was not a reference");
    }
}
pub fn anewarray(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn areturn(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn arraylength(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn astore(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn astore_0(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn astore_1(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn astore_2(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn astore_3(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn athrow(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn baload(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn bastore(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn bipush(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    let operands = inst.get_const_operands();
    let OperandType::Immediate(byte) = operands[0] else {
        panic!("Operand [0] for bipush was not an immediate");
    };
    frame.stack.push(FrameValues::Int(byte as i32));
}
pub fn caload(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn castore(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn checkcast(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn d2f(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn d2i(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn d2l(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn dadd(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    let Some(FrameValues::Double(first)) = frame.stack.pop() else {
        panic!("Top of Frame stack was empty or was not a Double");
    };
    let Some(FrameValues::Double(second)) = frame.stack.pop() else {
        panic!("Top of Frame stack was empty was not a Double");
    };
    let result = first + second;
    if result.is_nan() {
        frame.stack.push(FrameValues::NaN);
    } else {
        frame.stack.push(FrameValues::Double(result));
    }
}
pub fn daload(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn dastore(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn dcmpg(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn dcmpl(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn dconst_0(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn dconst_1(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn ddiv(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn dload(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    let operands = inst.get_const_operands();
    let OperandType::VarIndex(index) = operands[0] else {
        panic!("Operand [0] for dload was not a var index");
    };
    let Some(FrameValues::Double(local)) = frame.locals.get(index as usize)
    else {
        panic!("Frame local[{index}] does not exist or was not a Double");
    };
    frame.stack.push(FrameValues::Double(*local));
}
pub fn dload_0(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    let Some(FrameValues::Double(local)) = frame.locals.first() else {
        panic!("Frame local[0] does not exist or was not a Double");
    };
    frame.stack.push(FrameValues::Double(*local));
}
pub fn dload_1(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    let Some(FrameValues::Double(local)) = frame.locals.get(1) else {
        panic!("Frame local[1] does not exist or was not a Double");
    };
    frame.stack.push(FrameValues::Double(*local));
}
pub fn dload_2(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    let Some(FrameValues::Double(local)) = frame.locals.get(2) else {
        panic!("Frame local[2] does not exist or was not a Double");
    };
    frame.stack.push(FrameValues::Double(*local));
}
pub fn dload_3(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    let Some(FrameValues::Double(local)) = frame.locals.get(3) else {
        panic!("Frame local[3] does not exist or was not a Double");
    };
    frame.stack.push(FrameValues::Double(*local));
}
pub fn dmul(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn dneg(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn drem(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn dreturn(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn dstore(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    let operands = inst.get_const_operands();
    let OperandType::VarIndex(index) = operands[0] else {
        panic!("Operand [0] for dstore was not a var index");
    };
    let Some(mut local) = frame.locals.get_mut(index as usize) else {
        panic!("Frame local[{index}] does not exist or was not a Double");
    };
    let Some(FrameValues::Double(top)) = frame.stack.pop() else {
        panic!("Frame stack was empty or not a Double!");
    };
    *local = FrameValues::Double(top);
}
pub fn dstore_0(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    match frame.stack.pop() {
        Some(FrameValues::Double(double)) => {
            if let Some(mut local) = frame.locals.get_mut(0) {
                *local = FrameValues::Double(double);
            } else {
                frame.locals.insert(0, FrameValues::Double(double));
            }
        }
        _ => panic!("Frame stack was empty or top was not a double!"),
    }
}
pub fn dstore_1(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    match frame.stack.pop() {
        Some(FrameValues::Double(double)) => {
            if let Some(mut local) = frame.locals.get_mut(1) {
                *local = FrameValues::Double(double);
            } else {
                frame.locals.insert(1, FrameValues::Double(double));
            }
        }
        _ => panic!("Frame stack was empty or top was not a double!"),
    }
}
pub fn dstore_2(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    match frame.stack.pop() {
        Some(FrameValues::Double(double)) => {
            if let Some(mut local) = frame.locals.get_mut(2) {
                *local = FrameValues::Double(double);
            } else {
                frame.locals.insert(2, FrameValues::Double(double));
            }
        }
        _ => panic!("Frame stack was empty or top was not a double!"),
    }
}
pub fn dstore_3(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    match frame.stack.pop() {
        Some(FrameValues::Double(double)) => {
            if let Some(mut local) = frame.locals.get_mut(3) {
                *local = FrameValues::Double(double);
            } else {
                frame.locals.insert(3, FrameValues::Double(double));
            }
        }
        _ => panic!("Frame stack was empty or top was not a double!"),
    }
}
pub fn dsub(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn dup(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn dup_x1(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn dup_x2(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn dup2(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn dup2_x1(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn dup2_x2(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn f2d(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    let Some(FrameValues::Float(float)) = frame.stack.pop() else {
        panic!("Top of stack was not a Float or stack is empty");
    };
    frame.stack.push(FrameValues::Double(float as f64));
}
pub fn f2i(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn f2l(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn fadd(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn faload(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn fastore(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn fcmpg(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn fcmpl(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn fconst_0(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn fconst_1(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn fconst_2(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn fdiv(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn fload(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    let operands = inst.get_const_operands();
    let OperandType::VarIndex(index) = operands[0] else {
        panic!("Operand [0] for fload was not a var index");
    };
    let Some(FrameValues::Float(local)) = frame.locals.get(index as usize)
    else {
        panic!("Frame local[{index}] does not exist or was not a Float");
    };
    frame.stack.push(FrameValues::Float(*local));
}
pub fn fload_0(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    let Some(FrameValues::Float(local)) = frame.locals.first() else {
        panic!("Frame local[0] does not exist or was not a Float");
    };
    frame.stack.push(FrameValues::Float(*local));
}
pub fn fload_1(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    let Some(FrameValues::Float(local)) = frame.locals.get(1) else {
        panic!("Frame local[1] does not exist or was not a Float");
    };
    frame.stack.push(FrameValues::Float(*local));
}
pub fn fload_2(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    let Some(FrameValues::Float(local)) = frame.locals.get(2) else {
        panic!("Frame local[2] does not exist or was not a Float");
    };
    frame.stack.push(FrameValues::Float(*local));
}
pub fn fload_3(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    let Some(FrameValues::Float(local)) = frame.locals.get(3) else {
        panic!("Frame local[3] does not exist or was not a Float");
    };
    frame.stack.push(FrameValues::Float(*local));
}
pub fn fmul(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn fneg(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn frem(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn freturn(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn fstore(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    let operands = inst.get_const_operands();
    let OperandType::VarIndex(index) = operands[0] else {
        panic!("Operand [0] for dstore was not a var index");
    };
    let Some(mut local) = frame.locals.get_mut(index as usize) else {
        panic!("Frame local[{index}] does not exist or was not a Float");
    };
    let Some(FrameValues::Float(top)) = frame.stack.pop() else {
        panic!("Frame stack was empty or not a Float!");
    };
    *local = FrameValues::Float(top);
}
pub fn fstore_0(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    match frame.stack.pop() {
        Some(FrameValues::Float(float)) => {
            if let Some(mut local) = frame.locals.get_mut(0) {
                *local = FrameValues::Float(float);
            } else {
                frame.locals.insert(0, FrameValues::Float(float));
            }
        }
        _ => panic!("Frame stack was empty or top was not a float!"),
    }
}
pub fn fstore_1(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    match frame.stack.pop() {
        Some(FrameValues::Float(float)) => {
            if let Some(mut local) = frame.locals.get_mut(1) {
                *local = FrameValues::Float(float);
            } else {
                frame.locals.insert(1, FrameValues::Float(float));
            }
        }
        _ => panic!("Frame stack was empty or top was not a float!"),
    }
}
pub fn fstore_2(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    match frame.stack.pop() {
        Some(FrameValues::Float(float)) => {
            if let Some(mut local) = frame.locals.get_mut(2) {
                *local = FrameValues::Float(float);
            } else {
                frame.locals.insert(2, FrameValues::Float(float));
            }
        }
        _ => panic!("Frame stack was empty or top was not a float!"),
    }
}
pub fn fstore_3(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    match frame.stack.pop() {
        Some(FrameValues::Float(float)) => {
            if let Some(mut local) = frame.locals.get_mut(3) {
                *local = FrameValues::Float(float);
            } else {
                frame.locals.insert(3, FrameValues::Float(float));
            }
        }
        _ => panic!("Frame stack was empty or top was not a float!"),
    }
}
pub fn fsub(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn getfield(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn goto(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn goto_w(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn i2b(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn i2c(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn i2d(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn i2f(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn i2l(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn i2s(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn iadd(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    let Some(FrameValues::Int(a)) = frame.stack.pop() else {
        panic!("Value on top of stack was not int");
    };
    let Some(FrameValues::Int(b)) = frame.stack.pop() else {
        panic!("Value on top of stack was not int");
    };
    let (res, _) = a.overflowing_add(b);
    frame.stack.push(FrameValues::Int(res));
}
pub fn iaload(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn iand(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn iastore(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn iconst_m1(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    frame.stack.push(FrameValues::Int(-1));
}
pub fn iconst_0(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    frame.stack.push(FrameValues::Int(0));
}
pub fn iconst_1(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    frame.stack.push(FrameValues::Int(1));
}
pub fn iconst_2(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    frame.stack.push(FrameValues::Int(2));
}
pub fn iconst_3(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    frame.stack.push(FrameValues::Int(3));
}
pub fn iconst_4(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    frame.stack.push(FrameValues::Int(4));
}
pub fn iconst_5(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    frame.stack.push(FrameValues::Int(5));
}
pub fn idiv(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn if_acmpeq(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn if_acmpne(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn if_icmpeq(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn if_icmpne(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn if_icmplt(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn if_icmpge(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn if_icmpgt(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn if_icmple(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn ifeq(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn ifne(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn iflt(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn ifge(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn ifgt(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn ifle(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn ifnonnull(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn ifnull(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn iinc(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn iload(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    let operands = inst.get_const_operands();
    let OperandType::VarIndex(index) = operands[0] else {
        panic!("Operand [0] for iload was not a var index");
    };
    let Some(FrameValues::Int(local)) = frame.locals.get(index as usize) else {
        panic!("Frame local[{index}] does not exist or was not an Int");
    };
    frame.stack.push(FrameValues::Int(*local));
}
pub fn iload_0(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    let Some(FrameValues::Int(local)) = frame.locals.first() else {
        panic!("Frame local[0] does not exist");
    };
    frame.stack.push(FrameValues::Int(*local));
}
pub fn iload_1(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    let Some(FrameValues::Int(local)) = frame.locals.get(1) else {
        panic!("Frame local[1] does not exist");
    };
    frame.stack.push(FrameValues::Int(*local));
}
pub fn iload_2(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    let Some(FrameValues::Int(local)) = frame.locals.get(2) else {
        panic!("Frame local[2] does not exist");
    };
    frame.stack.push(FrameValues::Int(*local));
}
pub fn iload_3(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    let Some(FrameValues::Int(local)) = frame.locals.get(3) else {
        panic!("Frame local[3] does not exist");
    };
    frame.stack.push(FrameValues::Int(*local));
}
pub fn invokestatic(frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn imul(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    let Some(FrameValues::Int(lhs)) = frame.stack.pop() else {
        panic!("Stack was empty");
    };
    let Some(FrameValues::Int(rhs)) = frame.stack.pop() else {
        panic!("Stack was empty");
    };
    frame.stack.push(FrameValues::Int(lhs * rhs));
}
pub fn ineg(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn instanceof(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn invokedynamic(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn invokeinterface(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn invokespecial(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn invokevirtual(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn ior(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn irem(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn ireturn(
    vm: &mut VM,
    frame: &mut StackFrame,
    inst: Instruction,
    callee: &mut StackFrame,
) {
    let Some(value) = frame.stack.pop() else {
        panic!("Stack was empty!");
    };
    callee.stack.push(value);
    let mut thread = &mut vm.threads[frame.thread_id];
    // This should definite get the callee's id
    thread.active_frame -= 1;
}
pub fn ishl(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn ishr(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn istore(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    let operands = inst.get_const_operands();
    let OperandType::VarIndex(index) = operands[0] else {
        panic!("Operand [0] for istore was not a var index");
    };
    let Some(mut local) = frame.locals.get_mut(index as usize) else {
        panic!("Frame local[{index}] does not exist or was not an Int");
    };
    let Some(FrameValues::Int(top)) = frame.stack.pop() else {
        panic!("Frame stack was empty or not an int!");
    };
    *local = FrameValues::Int(top);
}
pub fn istore_0(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    let Some(FrameValues::Int(top)) = frame.stack.pop() else {
        panic!("Frame stack was empty or not an int!");
    };
    if let Some(mut local) = frame.locals.get_mut(0) {
        *local = FrameValues::Int(top);
    } else {
        frame.locals.insert(0, FrameValues::Int(top));
    }
}
pub fn istore_1(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    let Some(FrameValues::Int(top)) = frame.stack.pop() else {
        panic!("Frame stack was empty or not an int!");
    };
    if let Some(mut local) = frame.locals.get_mut(1) {
        *local = FrameValues::Int(top);
    } else {
        frame.locals.insert(1, FrameValues::Int(top));
    }
}
pub fn istore_2(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    let Some(FrameValues::Int(top)) = frame.stack.pop() else {
        panic!("Frame stack was empty or not an int!");
    };
    if let Some(mut local) = frame.locals.get_mut(2) {
        *local = FrameValues::Int(top);
    } else {
        frame.locals.insert(2, FrameValues::Int(top));
    }
}
pub fn istore_3(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    let Some(FrameValues::Int(top)) = frame.stack.pop() else {
        panic!("Frame stack was empty or not an int!");
    };
    if let Some(mut local) = frame.locals.get_mut(3) {
        *local = FrameValues::Int(top);
    } else {
        frame.locals.insert(3, FrameValues::Int(top));
    }
}
pub fn isub(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn iushr(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn ixor(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn jsr(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn jsr_w(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn l2d(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn l2f(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn l2i(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn ladd(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn laload(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn land(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn lastore(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn lcmp(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn lconst_0(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn lconst_1(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn ldc(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    let OperandType::PoolIndex(index) = inst.get_const_operands()[0] else {
        panic!(
            "No PoolIndex found, instead found {:?}",
            inst.get_const_operands()[0]
        );
    };
    let Some(constant) = frame.pool.get(index as usize) else {
        panic!("Failed to get constant at {index} from frame pool");
    };
    dbg!(&constant);

    match constant {
        runtime_pool::RuntimeConstant::SymbolicRef(sr) => match sr {
            runtime_pool::SymbolicRef::Class(_) => todo!(),
            runtime_pool::SymbolicRef::Interface(_) => todo!(),
            runtime_pool::SymbolicRef::Array(_) => todo!(),
            runtime_pool::SymbolicRef::Field(_, _, _) => todo!(),
            runtime_pool::SymbolicRef::ClassMethod(_, _, _) => todo!(),
            runtime_pool::SymbolicRef::InterfaceMethod(_, _, _) => todo!(),
            runtime_pool::SymbolicRef::MethodHandle(_) => todo!(),
            runtime_pool::SymbolicRef::MethodType(_) => todo!(),
            runtime_pool::SymbolicRef::DynamicConst(_, _, _, _) => todo!(),
            runtime_pool::SymbolicRef::DynamicCall(_, _, _, _) => todo!(),
            runtime_pool::SymbolicRef::Null => todo!(),
        },
        runtime_pool::RuntimeConstant::StaticConstant(sc) => {
            match sc {
                runtime_pool::StaticConstant::String(s) => todo!(),
                runtime_pool::StaticConstant::Integer(i) => todo!(),
                runtime_pool::StaticConstant::Float(f) => {
                    match f {
                        runtime_pool::Decimal::Double(_) => {
                            // FIXME: Probably has some java error
                            unreachable!("Should not find a double where a float is expected")
                        }
                        runtime_pool::Decimal::Float(float) => {
                            frame.stack.push(FrameValues::Float(*float))
                        }
                    }
                }
                runtime_pool::StaticConstant::Long(l) => todo!(),
                runtime_pool::StaticConstant::Double(d) => match d {
                    runtime_pool::Decimal::Float(_) => {
                        unreachable!("Should not find a float where a double is expected")
                    }
                    runtime_pool::Decimal::Double(double) => {
                        frame.stack.push(FrameValues::Double(*double))
                    }
                },
            }
        }
        RuntimeConstant::Spacer => {}
    }
}
pub fn ldc_w(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn ldc2_w(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    let OperandType::PoolIndex(upper) = inst.get_const_operands()[0] else {
        panic!(
            "No PoolIndex found, instead found {:?}",
            inst.get_const_operands()[0]
        );
    };
    let OperandType::PoolIndex(lower) = inst.get_const_operands()[1] else {
        panic!(
            "No PoolIndex found, instead found {:?}",
            inst.get_const_operands()[0]
        );
    };
    let index = (upper as usize) << 8 | lower as usize;
    let Some(constant) = frame.pool.get(index) else {
        panic!("Failed to get constant at {index} from frame pool");
    };
    dbg!(&constant);

    match constant {
        runtime_pool::RuntimeConstant::StaticConstant(sc) => match sc {
            runtime_pool::StaticConstant::Long(l) => frame.stack.push(FrameValues::Long(*l)),
            runtime_pool::StaticConstant::Double(d) => match d {
                runtime_pool::Decimal::Float(_) => {
                    unreachable!("Should not find a float where a double is expected")
                }
                runtime_pool::Decimal::Double(double) => {
                    frame.stack.push(FrameValues::Double(*double))
                }
            },
            _other => panic!("Expected a Double, Long or dyn-computed Constant, got {_other:?}"),
        },
        _other => panic!("Expected a Double, Long or dyn-computed Constant, got {_other:?}"),
    }
}
pub fn ldiv(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn lload(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn lload_0(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn lload_1(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn lload_2(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn lload_3(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn lmul(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn lneg(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn lookupswitch(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn lor(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn lrem(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn lreturn(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn lshl(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn lshr(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn lstore(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn lstore_0(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn lstore_1(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn lstore_2(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn lstore_3(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn lsub(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn lushr(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn lxor(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn monitorenter(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn monitorexit(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn multianewarray(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn new(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn newarray(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn nop(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn pop(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn pop2(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn putfield(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn putstatic(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn ret(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn r#return(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    /*
       The current method must have return type void. If the
       current method is a synchronized method, the monitor entered
       or reentered on invocation of the method is updated and
       possibly exited as if by execution of a monitorexit instruction
       (§monitorexit) in the current thread. If no exception is thrown,
       any values on the operand stack of the current frame (§2.6) are
       discarded.
       The interpreter then returns control to the invoker of the method,
       reinstating the frame of the invoker.
    */
    dbg!(&frame.stack);
    dbg!(&frame.locals);
    println!("Returned!");
}
pub fn saload(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn satore(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn sipush(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    let operands = inst.get_const_operands();
    let OperandType::Immediate(byte1) = operands[0] else {
        panic!("Operand [0] for sipush was not an immediate");
    };
    let OperandType::Immediate(byte2) = operands[1] else {
        panic!("Operand [1] for sipush was not an immediate");
    };
    let short: u16 = ((byte1 as u16) << 8) | byte2 as u16;
    let sign_extend: i32 = short as i32;
    frame.stack.push(FrameValues::Int(sign_extend));
}
pub fn swap(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
pub fn tableswitch(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) {
    todo!()
}
pub fn wide(vm: &mut VM, frame: &mut StackFrame, inst: Instruction) { todo!() }
