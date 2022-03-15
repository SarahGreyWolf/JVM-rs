

#[repr(u16)]
#[derive(Debug, Clone)]
/// [Class Access Flags](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#page=85)
pub enum ClassAccessFlags {
    None=0x0000,
    /// Declared public; may be accessed from outside its package.
    AccPublic=0x0001,
    /// Declared final; no subclasses allowed.
    AccFinal=0x0010,
    /// Treat superclass methods specially when invoked by\
    /// the invokespecial instruction.
    AccSuper=0x0020,
    /// Is an interface, not a class.
    AccInterface=0x0200,
    /// Declared abstract; must not be instantiated.
    AccAbstract=0x0400,
    /// Declared synthetic; not present in the source code.
    AccSynthetic=0x1000,
    /// Declared as an annotation interface.
    AccAnnotation=0x2000,
    /// Declared as an enum class.
    AccEnum=0x4000,
    /// Is a module, not a class or interface.
    AccModule=0x8000
}

impl ClassAccessFlags {
    pub fn from_u16(value: u16) -> Vec<Self> {
        let mut flags = vec![];
        if value & ClassAccessFlags::AccPublic as u16 == ClassAccessFlags::AccPublic as u16 {
            flags.push(ClassAccessFlags::AccPublic);
        }
        if value & ClassAccessFlags::AccFinal as u16 == ClassAccessFlags::AccFinal as u16 {
            flags.push(ClassAccessFlags::AccFinal);
        }
        if value & ClassAccessFlags::AccSuper as u16 == ClassAccessFlags::AccSuper as u16 {
            flags.push(ClassAccessFlags::AccSuper);
        }
        if value & ClassAccessFlags::AccInterface as u16 == ClassAccessFlags::AccInterface as u16 {
            flags.push(ClassAccessFlags::AccInterface);
        }
        if value & ClassAccessFlags::AccAbstract as u16 == ClassAccessFlags::AccAbstract as u16 {
            flags.push(ClassAccessFlags::AccAbstract);
        }
        if value & ClassAccessFlags::AccSynthetic as u16 == ClassAccessFlags::AccSynthetic as u16 {
            flags.push(ClassAccessFlags::AccSynthetic);
        }
        if value & ClassAccessFlags::AccAnnotation as u16 == ClassAccessFlags::AccAnnotation as u16 {
            flags.push(ClassAccessFlags::AccAnnotation);
        }
        if value & ClassAccessFlags::AccEnum as u16 == ClassAccessFlags::AccEnum as u16 {
            flags.push(ClassAccessFlags::AccEnum);
        }
        if value & ClassAccessFlags::AccModule as u16 == ClassAccessFlags::AccModule as u16 {
            flags.push(ClassAccessFlags::AccModule);
        }
        flags
    }
}

#[repr(u16)]
#[derive(Debug, Clone)]
/// [Method Access Flags](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#page=112)
pub enum MethodAccessFlags {
    None=0x0000,
    /// Declared public; may be accessed from outside its package.
    AccPublic=0x0001,
    /// Declared private; accessible only within the
    /// defining class and other classes belonging to the same
    /// nest (§5.4.4).
    AccPrivate=0x0002,
    /// Declared protected; may be accessed within
    /// subclasses.
    AccProtected=0x0004,
    /// Declared static.
    AccStatic=0x0008,
    /// Declared final; no subclasses allowed.
    AccFinal=0x0010,
    /// Declared synchronized; invocation is wrapped
    /// by a monitor use.
    AccSynchronized=0x0020,
    /// A bridge method, generated by the compiler.
    AccBridge=0x0040,
    /// Declared with variable number of arguments.
    AccVarArgs=0x0080,
    /// Declared native; implemented in a language other
    /// than the Java programming language.
    AccNative=0x0100,
    /// Declared abstract; must not be instantiated.
    AccAbstract=0x0400,
    /// In a class file whose major version number is at
    /// least 46 and at most 60: Declared strictfp.
    AccStrict=0x0800,
    /// Declared synthetic; not present in the source code.
    AccSynthetic=0x1000,
}

impl MethodAccessFlags {
    pub fn from_u16(value: u16) -> Vec<Self> {
        let mut flags = vec![];
        if value & MethodAccessFlags::AccPublic as u16 == MethodAccessFlags::AccPublic as u16 {
            flags.push(MethodAccessFlags::AccPublic);
        }
        if value & MethodAccessFlags::AccPrivate as u16 == MethodAccessFlags::AccPrivate as u16 {
            flags.push(MethodAccessFlags::AccPrivate);
        }
        if value & MethodAccessFlags::AccProtected as u16 == MethodAccessFlags::AccProtected as u16 {
            flags.push(MethodAccessFlags::AccProtected);
        }
        if value & MethodAccessFlags::AccStatic as u16 == MethodAccessFlags::AccStatic as u16 {
            flags.push(MethodAccessFlags::AccStatic);
        }
        if value & MethodAccessFlags::AccFinal as u16 == MethodAccessFlags::AccFinal as u16 {
            flags.push(MethodAccessFlags::AccFinal);
        }
        if value & MethodAccessFlags::AccSynchronized as u16 == MethodAccessFlags::AccSynchronized as u16 {
            flags.push(MethodAccessFlags::AccSynchronized);
        }
        if value & MethodAccessFlags::AccBridge as u16 == MethodAccessFlags::AccBridge as u16 {
            flags.push(MethodAccessFlags::AccBridge);
        }
        if value & MethodAccessFlags::AccVarArgs as u16 == MethodAccessFlags::AccVarArgs as u16 {
            flags.push(MethodAccessFlags::AccVarArgs);
        }
        if value & MethodAccessFlags::AccNative as u16 == MethodAccessFlags::AccNative as u16 {
            flags.push(MethodAccessFlags::AccNative);
        }
        if value & MethodAccessFlags::AccAbstract as u16 == MethodAccessFlags::AccAbstract as u16 {
            flags.push(MethodAccessFlags::AccAbstract);
        }
        if value & MethodAccessFlags::AccStrict as u16 == MethodAccessFlags::AccStrict as u16 {
            flags.push(MethodAccessFlags::AccStrict);
        }
        if value & MethodAccessFlags::AccSynthetic as u16 == MethodAccessFlags::AccSynthetic as u16 {
            flags.push(MethodAccessFlags::AccSynthetic);
        }
        flags
    }
}

#[repr(u16)]
#[derive(Debug, Clone)]
pub enum FieldAccessFlags {
    None=0x0000,
    /// Declared public; may be accessed from outside its package.
    AccPublic=0x0001,
    /// Declared private; accessible only within the
    /// defining class and other classes belonging to the same
    /// nest (§5.4.4).
    AccPrivate=0x0002,
    /// Declared protected; may be accessed within
    /// subclasses.
    AccProtected=0x0004,
    /// Declared static.
    AccStatic=0x0008,
    /// Declared final; no subclasses allowed.
    AccFinal=0x0010,
    /// Declared volatile; cannot be cached.
    AccVolatile=0x0040,
    /// Declared transient; not written or read by a
    /// persistent object manager.
    AccTransient=0x0080,
    /// Declared synthetic; not present in the source code.
    AccSynthetic=0x1000,
    /// Declared as an enum class.
    AccEnum=0x4000,
}

impl FieldAccessFlags {
    pub fn from_u16(value: u16) -> Vec<Self> {
        let mut flags = vec![];
        if value & FieldAccessFlags::AccPublic as u16 == FieldAccessFlags::AccPublic as u16 {
            flags.push(FieldAccessFlags::AccPublic);
        }
        if value & FieldAccessFlags::AccPrivate as u16 == FieldAccessFlags::AccPrivate as u16 {
            flags.push(FieldAccessFlags::AccPrivate);
        }
        if value & FieldAccessFlags::AccProtected as u16 == FieldAccessFlags::AccProtected as u16 {
            flags.push(FieldAccessFlags::AccProtected);
        }
        if value & FieldAccessFlags::AccStatic as u16 == FieldAccessFlags::AccStatic as u16 {
            flags.push(FieldAccessFlags::AccStatic);
        }
        if value & FieldAccessFlags::AccFinal as u16 == FieldAccessFlags::AccFinal as u16 {
            flags.push(FieldAccessFlags::AccFinal);
        }
        if value & FieldAccessFlags::AccVolatile as u16 == FieldAccessFlags::AccVolatile as u16 {
            flags.push(FieldAccessFlags::AccVolatile);
        }
        if value & FieldAccessFlags::AccTransient as u16 == FieldAccessFlags::AccTransient as u16 {
            flags.push(FieldAccessFlags::AccTransient);
        }
        if value & FieldAccessFlags::AccSynthetic as u16 == FieldAccessFlags::AccSynthetic as u16 {
            flags.push(FieldAccessFlags::AccSynthetic);
        }
        if value & FieldAccessFlags::AccEnum as u16 == FieldAccessFlags::AccEnum as u16 {
            flags.push(FieldAccessFlags::AccEnum);
        }
        flags
    }
}