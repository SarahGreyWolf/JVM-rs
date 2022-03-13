pub mod class_format_check {
    use std::error::Error;
    use std::fmt::{write, Display};

    #[derive(Debug)]
    pub enum FormatCause {
        MagicNotCorrect,
        ExtraBytes,
    }

    impl Display for FormatCause {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                FormatCause::MagicNotCorrect => write!(f, "MagicNotCorrect"),
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
    use std::fmt::{write, Display};

    use crate::class_file::ConstantPool;

    #[derive(Debug)]
    pub enum LoadingCause {
        InvalidConstantTag(u8),
        InvalidAttributeNameIndex(ConstantPool)
    }

    impl Display for LoadingCause {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                LoadingCause::InvalidConstantTag(t) => write!(f, "InvalidConstantTag: {t}"),
                LoadingCause::InvalidAttributeNameIndex(t) => write!(f, "InvalidAttributeNameIndex: {:?}", t),
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
