pub mod class_format_check {
    use std::error::Error;
    use std::fmt::Display;

    #[derive(Debug)]
    pub enum FormatCause {
        IncorrectMagic(u32),
        ExtraBytes,
    }

    impl Display for FormatCause {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                FormatCause::IncorrectMagic(t) => write!(f, "MagicIncorrect: {:02X?}", t),
                FormatCause::ExtraBytes => write!(f, "ExtraBytes"),
            }
        }
    }

    #[derive(Debug)]
    pub struct FormatError {
        cause: FormatCause,
        msg: String,
    }

    impl FormatError {
        pub fn new(cause: FormatCause, msg: &str) -> FormatError {
            FormatError {
                cause,
                msg: msg.into(),
            }
        }
    }

    impl Error for FormatError {}

    impl Display for FormatError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "VerificationError: {}, {}", self.cause, self.msg)
        }
    }
}

pub mod class_loading {
    use std::error::Error;
    use std::fmt::Display;

    use crate::class_file::ConstantPool;

    #[derive(Debug)]
    pub enum LoadingCause {
        InvalidConstantTag(u8),
        InvalidAttributeNameIndex(ConstantPool),
        InvalidTargetInfoValue(u8),
        InvalidTargetTypeValue(u8),
        InvalidTypePathKind(u8),
    }

    impl Display for LoadingCause {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                LoadingCause::InvalidConstantTag(t) => write!(f, "InvalidConstantTag: {t}"),
                LoadingCause::InvalidAttributeNameIndex(t) => {
                    write!(f, "InvalidAttributeNameIndex: {:?}", t)
                }
                LoadingCause::InvalidTargetInfoValue(t) => {
                    write!(f, "InvalidTargetInfoValue: {t}")
                }
                LoadingCause::InvalidTargetTypeValue(t) => {
                    write!(f, "InvalidTargetTypeValue: {t}")
                }
                LoadingCause::InvalidTypePathKind(t) => {
                    write!(f, "InvalidTypePathKind: {t}")
                }
            }
        }
    }

    #[derive(Debug)]
    pub struct LoadingError {
        cause: LoadingCause,
        msg: String,
    }

    impl LoadingError {
        pub fn new(cause: LoadingCause, msg: &str) -> LoadingError {
            LoadingError {
                cause,
                msg: msg.into(),
            }
        }
    }

    impl Error for LoadingError {}

    impl Display for LoadingError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "LoadingError: {}, {}", self.cause, self.msg)
        }
    }
}
