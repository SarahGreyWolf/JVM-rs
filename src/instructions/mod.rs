pub mod mnemonics;

use std::{io::Cursor, ops::Deref};

use byteorder::ReadBytesExt;
use jloader::constants::{self, ConstantPool};
use mnemonics::Mnemonic;

#[derive(Debug)]
pub enum OperandType {
    PoolIndex(u8),
    VarIndex(u8),
    Offset(u8),
    Immediate(u8),
}

// impl Deref for OperandType {
//     type Target = u8;

//     fn deref(&self) -> &Self::Target {
//         match self {
//             OperandType::PoolIndex(byte) => byte,
//             OperandType::VarIndex(byte) => byte,
//             OperandType::Offset(byte) => byte,
//             OperandType::Immediate(byte) => byte,
//         }
//     }
// }

#[derive(Debug)]
pub struct Instruction {
    mnemonic: Mnemonic,
    const_operands: Vec<OperandType>,
}

impl Instruction {
    pub fn from_mnemonic(
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
                const_operands: vec![OperandType::Immediate(cursor.read_u8()?)],
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
                eprintln!("UNKNOWN INSTRUCTION {opcode} AT {}", cursor.position());
                Instruction {
                    mnemonic: Mnemonic::Unknown(*opcode),
                    const_operands: vec![],
                }
            }
        })
    }

    pub fn get_const_operands(&self) -> &Vec<OperandType> { &self.const_operands }
}

fn aaload(inst: Instruction) { todo!() }
fn aastore(inst: Instruction) { todo!() }
fn aconst_null(inst: Instruction) { todo!() }
fn aload(inst: Instruction) { todo!() }
fn aload_0(inst: Instruction) { todo!() }
fn aload_1(inst: Instruction) { todo!() }
fn aload_2(inst: Instruction) { todo!() }
fn aload_3(inst: Instruction) { todo!() }
fn anewarray(inst: Instruction) { todo!() }
fn areturn(inst: Instruction) { todo!() }
fn arraylength(inst: Instruction) { todo!() }
fn astore(inst: Instruction) { todo!() }
fn astore_0(inst: Instruction) { todo!() }
fn astore_1(inst: Instruction) { todo!() }
fn astore_2(inst: Instruction) { todo!() }
fn astore_3(inst: Instruction) { todo!() }
fn athrow(inst: Instruction) { todo!() }
fn baload(inst: Instruction) { todo!() }
fn bastore(inst: Instruction) { todo!() }
fn bipush(inst: Instruction) { todo!() }
fn caload(inst: Instruction) { todo!() }
fn castore(inst: Instruction) { todo!() }
fn checkcast(inst: Instruction) { todo!() }
fn d2f(inst: Instruction) { todo!() }
fn d2i(inst: Instruction) { todo!() }
fn d2l(inst: Instruction) { todo!() }
fn dadd(inst: Instruction) { todo!() }
fn daload(inst: Instruction) { todo!() }
fn dastore(inst: Instruction) { todo!() }
fn dcmpg(inst: Instruction) { todo!() }
fn dcmpl(inst: Instruction) { todo!() }
fn dconst_0(inst: Instruction) { todo!() }
fn dconst_1(inst: Instruction) { todo!() }
fn ddiv(inst: Instruction) { todo!() }
fn dload(inst: Instruction) { todo!() }
fn dload_0(inst: Instruction) { todo!() }
fn dload_1(inst: Instruction) { todo!() }
fn dload_2(inst: Instruction) { todo!() }
fn dload_3(inst: Instruction) { todo!() }
fn dmul(inst: Instruction) { todo!() }
fn dneg(inst: Instruction) { todo!() }
fn drem(inst: Instruction) { todo!() }
fn dreturn(inst: Instruction) { todo!() }
fn dstore(inst: Instruction) { todo!() }
fn dstore_0(inst: Instruction) { todo!() }
fn dstore_1(inst: Instruction) { todo!() }
fn dstore_2(inst: Instruction) { todo!() }
fn dstore_3(inst: Instruction) { todo!() }
fn dsub(inst: Instruction) { todo!() }
fn dup(inst: Instruction) { todo!() }
fn dup_x1(inst: Instruction) { todo!() }
fn dup_x2(inst: Instruction) { todo!() }
fn dup2(inst: Instruction) { todo!() }
fn dup2_x1(inst: Instruction) { todo!() }
fn dup2_x2(inst: Instruction) { todo!() }
fn f2d(inst: Instruction) { todo!() }
fn f2i(inst: Instruction) { todo!() }
fn f2l(inst: Instruction) { todo!() }
fn fadd(inst: Instruction) { todo!() }
fn faload(inst: Instruction) { todo!() }
fn fastore(inst: Instruction) { todo!() }
fn fcmpg(inst: Instruction) { todo!() }
fn fcmpl(inst: Instruction) { todo!() }
fn fconst_0(inst: Instruction) { todo!() }
fn fconst_1(inst: Instruction) { todo!() }
fn fconst_2(inst: Instruction) { todo!() }
fn fdiv(inst: Instruction) { todo!() }
fn fload(inst: Instruction) { todo!() }
fn fload_0(inst: Instruction) { todo!() }
fn fload_1(inst: Instruction) { todo!() }
fn fload_2(inst: Instruction) { todo!() }
fn fload_3(inst: Instruction) { todo!() }
fn fmul(inst: Instruction) { todo!() }
fn fneg(inst: Instruction) { todo!() }
fn frem(inst: Instruction) { todo!() }
fn freturn(inst: Instruction) { todo!() }
fn fstore(inst: Instruction) { todo!() }
fn fstore_0(inst: Instruction) { todo!() }
fn fstore_1(inst: Instruction) { todo!() }
fn fstore_2(inst: Instruction) { todo!() }
fn fstore_3(inst: Instruction) { todo!() }
fn fsub(inst: Instruction) { todo!() }
fn getfield(inst: Instruction) { todo!() }
fn getstatic(inst: Instruction) { todo!() }
fn goto(inst: Instruction) { todo!() }
fn goto_w(inst: Instruction) { todo!() }
fn i2b(inst: Instruction) { todo!() }
fn i2c(inst: Instruction) { todo!() }
fn i2d(inst: Instruction) { todo!() }
fn i2f(inst: Instruction) { todo!() }
fn i2l(inst: Instruction) { todo!() }
fn i2s(inst: Instruction) { todo!() }
fn iadd(inst: Instruction) { todo!() }
fn iaload(inst: Instruction) { todo!() }
fn iand(inst: Instruction) { todo!() }
fn iastore(inst: Instruction) { todo!() }
fn iconst_m1(inst: Instruction) { todo!() }
fn iconst_0(inst: Instruction) { todo!() }
fn iconst_1(inst: Instruction) { todo!() }
fn iconst_2(inst: Instruction) { todo!() }
fn iconst_3(inst: Instruction) { todo!() }
fn iconst_4(inst: Instruction) { todo!() }
fn iconst_5(inst: Instruction) { todo!() }
fn idiv(inst: Instruction) { todo!() }
fn if_acmpeq(inst: Instruction) { todo!() }
fn if_acmpne(inst: Instruction) { todo!() }
fn if_icmpeq(inst: Instruction) { todo!() }
fn if_icmpne(inst: Instruction) { todo!() }
fn if_icmplt(inst: Instruction) { todo!() }
fn if_icmpge(inst: Instruction) { todo!() }
fn if_icmpgt(inst: Instruction) { todo!() }
fn if_icmple(inst: Instruction) { todo!() }
fn ifeq(inst: Instruction) { todo!() }
fn ifne(inst: Instruction) { todo!() }
fn iflt(inst: Instruction) { todo!() }
fn ifge(inst: Instruction) { todo!() }
fn ifgt(inst: Instruction) { todo!() }
fn ifle(inst: Instruction) { todo!() }
fn ifnonnull(inst: Instruction) { todo!() }
fn ifnull(inst: Instruction) { todo!() }
fn iinc(inst: Instruction) { todo!() }
fn iload(inst: Instruction) { todo!() }
fn iload_0(inst: Instruction) { todo!() }
fn iload_1(inst: Instruction) { todo!() }
fn iload_2(inst: Instruction) { todo!() }
fn iload_3(inst: Instruction) { todo!() }
fn imul(inst: Instruction) { todo!() }
fn ineg(inst: Instruction) { todo!() }
fn instanceof(inst: Instruction) { todo!() }
fn invokedynamic(inst: Instruction) { todo!() }
fn invokeinterface(inst: Instruction) { todo!() }
fn invokespecial(inst: Instruction) { todo!() }
fn invokestatic(inst: Instruction) { todo!() }
fn invokevirtual(inst: Instruction) { todo!() }
fn ior(inst: Instruction) { todo!() }
fn irem(inst: Instruction) { todo!() }
fn ireturn(inst: Instruction) { todo!() }
fn ishl(inst: Instruction) { todo!() }
fn ishr(inst: Instruction) { todo!() }
fn istore(inst: Instruction) { todo!() }
fn istore_0(inst: Instruction) { todo!() }
fn istore_1(inst: Instruction) { todo!() }
fn istore_2(inst: Instruction) { todo!() }
fn istore_3(inst: Instruction) { todo!() }
fn isub(inst: Instruction) { todo!() }
fn iushr(inst: Instruction) { todo!() }
fn ixor(inst: Instruction) { todo!() }
fn jsr(inst: Instruction) { todo!() }
fn jsr_w(inst: Instruction) { todo!() }
fn l2d(inst: Instruction) { todo!() }
fn l2f(inst: Instruction) { todo!() }
fn l2i(inst: Instruction) { todo!() }
fn ladd(inst: Instruction) { todo!() }
fn laload(inst: Instruction) { todo!() }
fn land(inst: Instruction) { todo!() }
fn lastore(inst: Instruction) { todo!() }
fn lcmp(inst: Instruction) { todo!() }
fn lconst_0(inst: Instruction) { todo!() }
fn lconst_1(inst: Instruction) { todo!() }
fn ldc(inst: Instruction) { todo!() }
fn ldc_w(inst: Instruction) { todo!() }
fn ldc2_w(inst: Instruction) { todo!() }
fn ldiv(inst: Instruction) { todo!() }
fn lload(inst: Instruction) { todo!() }
fn lload_0(inst: Instruction) { todo!() }
fn lload_1(inst: Instruction) { todo!() }
fn lload_2(inst: Instruction) { todo!() }
fn lload_3(inst: Instruction) { todo!() }
fn lmul(inst: Instruction) { todo!() }
fn lneg(inst: Instruction) { todo!() }
fn lookupswitch(inst: Instruction) { todo!() }
fn lor(inst: Instruction) { todo!() }
fn lrem(inst: Instruction) { todo!() }
fn lreturn(inst: Instruction) { todo!() }
fn lshl(inst: Instruction) { todo!() }
fn lshr(inst: Instruction) { todo!() }
fn lstore(inst: Instruction) { todo!() }
fn lstore_0(inst: Instruction) { todo!() }
fn lstore_1(inst: Instruction) { todo!() }
fn lstore_2(inst: Instruction) { todo!() }
fn lstore_3(inst: Instruction) { todo!() }
fn lsub(inst: Instruction) { todo!() }
fn lushr(inst: Instruction) { todo!() }
fn lxor(inst: Instruction) { todo!() }
fn monitorenter(inst: Instruction) { todo!() }
fn monitorexit(inst: Instruction) { todo!() }
fn multianewarray(inst: Instruction) { todo!() }
fn new(inst: Instruction) { todo!() }
fn newarray(inst: Instruction) { todo!() }
fn nop(inst: Instruction) { todo!() }
fn pop(inst: Instruction) { todo!() }
fn pop2(inst: Instruction) { todo!() }
fn putfield(inst: Instruction) { todo!() }
fn putstatic(inst: Instruction) { todo!() }
fn ret(inst: Instruction) { todo!() }
fn r#return(inst: Instruction) { todo!() }
fn saload(inst: Instruction) { todo!() }
fn satore(inst: Instruction) { todo!() }
fn sipush(inst: Instruction) { todo!() }
fn swap(inst: Instruction) { todo!() }
fn tableswitch(inst: Instruction) { todo!() }
fn wide(inst: Instruction) { todo!() }
