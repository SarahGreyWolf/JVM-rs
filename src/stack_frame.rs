use std::{
    error::Error,
    path::Path,
    sync::{Arc, Mutex},
};

use jloader::{class_file::ClassLoc, constants::PoolConstants};

use crate::{
    ops::{mnemonics::Mnemonic, Instruction},
    vm::FrameValues,
};

// https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A45%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C250%2Cnull%5D
#[derive(Debug)]
pub struct StackFrame {
    pub pc: Option<u64>,
    pub code: Vec<u8>,
    // https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A802%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C165%2Cnull%5D
    pub locals: Vec<FrameValues>,
    // https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A814%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C267%2Cnull%5D
    pub stack: Vec<FrameValues>,
    // https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A4314%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C325%2Cnull%5D
    pub pool: Vec<PoolConstants>,
}

impl StackFrame {
    // Takes an optional callee that is a mutable reference to the caller StackFrame
    pub fn run(
        &mut self,
        class_path: &Path,
        heap_ref: Arc<Mutex<Vec<u8>>>,
        method_area_ref: Arc<Mutex<Vec<ClassLoc>>>,
        callee: Option<&mut StackFrame>,
    ) -> Result<(), Box<dyn Error>> {
        loop {
            let instruction = Instruction::from_frame(self)?;
            println!("Executing Instruction {:?}", instruction.get_mnemonic());
            match instruction.get_mnemonic() {
                Mnemonic::Aaload => todo!(),
                Mnemonic::Aastore => todo!(),
                Mnemonic::AconstNull => todo!(),
                Mnemonic::Aload => todo!(),
                Mnemonic::Aload0 => todo!(),
                Mnemonic::Aload1 => todo!(),
                Mnemonic::Aload2 => todo!(),
                Mnemonic::Aload3 => todo!(),
                Mnemonic::Anewarray => todo!(),
                Mnemonic::Areturn => todo!(),
                Mnemonic::Arraylength => todo!(),
                Mnemonic::Astore => todo!(),
                Mnemonic::Astore0 => todo!(),
                Mnemonic::Astore1 => todo!(),
                Mnemonic::Astore2 => todo!(),
                Mnemonic::Astore3 => todo!(),
                Mnemonic::Athrow => todo!(),
                Mnemonic::Baload => todo!(),
                Mnemonic::Bastore => todo!(),
                Mnemonic::Bipush => crate::ops::bipush(self, instruction),
                Mnemonic::Caload => todo!(),
                Mnemonic::Castore => todo!(),
                Mnemonic::Checkcast => todo!(),
                Mnemonic::D2f => todo!(),
                Mnemonic::D2i => todo!(),
                Mnemonic::D2l => todo!(),
                Mnemonic::Dadd => todo!(),
                Mnemonic::Daload => todo!(),
                Mnemonic::Dastore => todo!(),
                Mnemonic::Dcmpg => todo!(),
                Mnemonic::Dcmpl => todo!(),
                Mnemonic::Dconst0 => todo!(),
                Mnemonic::Dconst1 => todo!(),
                Mnemonic::Ddiv => todo!(),
                Mnemonic::Dload => todo!(),
                Mnemonic::Dload0 => todo!(),
                Mnemonic::Dload1 => todo!(),
                Mnemonic::Dload2 => todo!(),
                Mnemonic::Dload3 => todo!(),
                Mnemonic::Dmul => todo!(),
                Mnemonic::Dneg => todo!(),
                Mnemonic::Drem => todo!(),
                Mnemonic::Dreturn => todo!(),
                Mnemonic::Dstore => todo!(),
                Mnemonic::Dstore0 => todo!(),
                Mnemonic::Dstore1 => todo!(),
                Mnemonic::Dstore2 => todo!(),
                Mnemonic::Dstore3 => todo!(),
                Mnemonic::Dsub => todo!(),
                Mnemonic::Dup => todo!(),
                Mnemonic::DupX1 => todo!(),
                Mnemonic::DupX2 => todo!(),
                Mnemonic::Dup2 => todo!(),
                Mnemonic::Dup2X1 => todo!(),
                Mnemonic::Dup2X2 => todo!(),
                Mnemonic::F2d => todo!(),
                Mnemonic::F2i => todo!(),
                Mnemonic::F2l => todo!(),
                Mnemonic::Fadd => todo!(),
                Mnemonic::Faload => todo!(),
                Mnemonic::Fastore => todo!(),
                Mnemonic::Fcmpg => todo!(),
                Mnemonic::Fcmpl => todo!(),
                Mnemonic::Fconst0 => todo!(),
                Mnemonic::Fconst1 => todo!(),
                Mnemonic::Fconst2 => todo!(),
                Mnemonic::Fdiv => todo!(),
                Mnemonic::Fload => todo!(),
                Mnemonic::Fload0 => todo!(),
                Mnemonic::Fload1 => todo!(),
                Mnemonic::Fload2 => todo!(),
                Mnemonic::Fload3 => todo!(),
                Mnemonic::Fmul => todo!(),
                Mnemonic::Fneg => todo!(),
                Mnemonic::Frem => todo!(),
                Mnemonic::Freturn => todo!(),
                Mnemonic::Fstore => todo!(),
                Mnemonic::Fstore0 => todo!(),
                Mnemonic::Fstore1 => todo!(),
                Mnemonic::Fstore2 => todo!(),
                Mnemonic::Fstore3 => todo!(),
                Mnemonic::Fsub => todo!(),
                Mnemonic::Getfield => todo!(),
                Mnemonic::Getstatic => todo!(),
                Mnemonic::Goto => todo!(),
                Mnemonic::GotoW => todo!(),
                Mnemonic::I2b => todo!(),
                Mnemonic::I2c => todo!(),
                Mnemonic::I2d => todo!(),
                Mnemonic::I2f => todo!(),
                Mnemonic::I2l => todo!(),
                Mnemonic::I2s => todo!(),
                Mnemonic::Iadd => crate::ops::iadd(self, instruction),
                Mnemonic::Iaload => todo!(),
                Mnemonic::Iand => todo!(),
                Mnemonic::Iastore => todo!(),
                Mnemonic::IconstM1 => crate::ops::iconst_m1(self, instruction),
                Mnemonic::Iconst0 => crate::ops::iconst_0(self, instruction),
                Mnemonic::Iconst1 => crate::ops::iconst_1(self, instruction),
                Mnemonic::Iconst2 => crate::ops::iconst_2(self, instruction),
                Mnemonic::Iconst3 => crate::ops::iconst_3(self, instruction),
                Mnemonic::Iconst4 => crate::ops::iconst_4(self, instruction),
                Mnemonic::Iconst5 => crate::ops::iconst_5(self, instruction),
                Mnemonic::Idiv => todo!(),
                Mnemonic::IfAcmpeq => todo!(),
                Mnemonic::IfAcmpne => todo!(),
                Mnemonic::IfIcmpeq => todo!(),
                Mnemonic::IfIcmpne => todo!(),
                Mnemonic::IfIcmplt => todo!(),
                Mnemonic::IfIcmpge => todo!(),
                Mnemonic::IfIcmpgt => todo!(),
                Mnemonic::IfIcmple => todo!(),
                Mnemonic::Ifeq => todo!(),
                Mnemonic::Ifne => todo!(),
                Mnemonic::Iflt => todo!(),
                Mnemonic::Ifge => todo!(),
                Mnemonic::Ifgt => todo!(),
                Mnemonic::Ifle => todo!(),
                Mnemonic::Ifnonnull => todo!(),
                Mnemonic::Ifnull => todo!(),
                Mnemonic::Iinc => todo!(),
                Mnemonic::Iload => todo!(),
                Mnemonic::Iload0 => todo!(),
                Mnemonic::Iload1 => crate::ops::iload_1(self, instruction),
                Mnemonic::Iload2 => crate::ops::iload_2(self, instruction),
                Mnemonic::Iload3 => todo!(),
                Mnemonic::Imul => todo!(),
                Mnemonic::Ineg => todo!(),
                Mnemonic::Instanceof => todo!(),
                Mnemonic::Invokedynamic => todo!(),
                Mnemonic::Invokeinterface => todo!(),
                Mnemonic::Invokespecial => todo!(),
                Mnemonic::Invokestatic => todo!(),
                Mnemonic::Invokevirtual => todo!(),
                Mnemonic::Ior => todo!(),
                Mnemonic::Irem => todo!(),
                Mnemonic::Ireturn => todo!(),
                Mnemonic::Ishl => todo!(),
                Mnemonic::Ishr => todo!(),
                Mnemonic::Istore => todo!(),
                Mnemonic::Istore0 => todo!(),
                Mnemonic::Istore1 => crate::ops::istore_1(self, instruction),
                Mnemonic::Istore2 => crate::ops::istore_2(self, instruction),
                Mnemonic::Istore3 => crate::ops::istore_3(self, instruction),
                Mnemonic::Isub => todo!(),
                Mnemonic::Iushr => todo!(),
                Mnemonic::Ixor => todo!(),
                Mnemonic::Jsr => todo!(),
                Mnemonic::JsrW => todo!(),
                Mnemonic::L2d => todo!(),
                Mnemonic::L2f => todo!(),
                Mnemonic::L2i => todo!(),
                Mnemonic::Ladd => todo!(),
                Mnemonic::Laload => todo!(),
                Mnemonic::Land => todo!(),
                Mnemonic::Lastore => todo!(),
                Mnemonic::Lcmp => todo!(),
                Mnemonic::Lconst0 => todo!(),
                Mnemonic::Lconst1 => todo!(),
                Mnemonic::Ldc => todo!(),
                Mnemonic::LdcW => todo!(),
                Mnemonic::Ldc2W => todo!(),
                Mnemonic::Ldiv => todo!(),
                Mnemonic::Lload => todo!(),
                Mnemonic::Lload0 => todo!(),
                Mnemonic::Lload1 => todo!(),
                Mnemonic::Lload2 => todo!(),
                Mnemonic::Lload3 => todo!(),
                Mnemonic::Lmul => todo!(),
                Mnemonic::Lneg => todo!(),
                Mnemonic::Lookupswitch => todo!(),
                Mnemonic::Lor => todo!(),
                Mnemonic::Lrem => todo!(),
                Mnemonic::Lreturn => todo!(),
                Mnemonic::Lshl => todo!(),
                Mnemonic::Lshr => todo!(),
                Mnemonic::Lstore => todo!(),
                Mnemonic::Lstore0 => todo!(),
                Mnemonic::Lstore1 => todo!(),
                Mnemonic::Lstore2 => todo!(),
                Mnemonic::Lstore3 => todo!(),
                Mnemonic::Lsub => todo!(),
                Mnemonic::Lushr => todo!(),
                Mnemonic::Lxor => todo!(),
                Mnemonic::Monitorenter => todo!(),
                Mnemonic::Monitorexit => todo!(),
                Mnemonic::Multianewarray => todo!(),
                Mnemonic::New => todo!(),
                Mnemonic::Newarray => todo!(),
                Mnemonic::Nop => todo!(),
                Mnemonic::Pop => todo!(),
                Mnemonic::Pop2 => todo!(),
                Mnemonic::Putfield => todo!(),
                Mnemonic::Putstatic => todo!(),
                Mnemonic::Ret => todo!(),
                // FIXME: This should return back to the previous StackFrame (if there is one)
                Mnemonic::Return => {
                    dbg!(&self.stack);
                    dbg!(&self.locals);
                    break;
                }
                Mnemonic::Saload => todo!(),
                Mnemonic::Satore => todo!(),
                Mnemonic::Sipush => crate::ops::sipush(self, instruction),
                Mnemonic::Swap => todo!(),
                Mnemonic::Tableswitch => todo!(),
                Mnemonic::WideOp => todo!(),
                Mnemonic::WideIinc => todo!(),
                Mnemonic::Unknown(_) => todo!(),
            }
        }
        Ok(())
    }
}