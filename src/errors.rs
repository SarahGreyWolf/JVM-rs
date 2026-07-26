pub mod vm {
    use std::{error::Error, fmt::Display};
    #[derive(Debug)]
    pub enum VMSetupCause {
        NoClassDefFoundError,
    }

    impl Display for VMSetupCause {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                VMSetupCause::NoClassDefFoundError => {
                    write!(f, "NoClassDefFoundError")
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct VMSetupError {
        cause: VMSetupCause,
        msg: String,
    }

    impl VMSetupError {
        pub fn new(cause: VMSetupCause, msg: &str) -> VMSetupError {
            VMSetupError {
                cause,
                msg: msg.into(),
            }
        }
    }

    impl Error for VMSetupError {}

    impl Display for VMSetupError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "VMSetupError: {}, {}", self.cause, self.msg)
        }
    }
}

pub mod exceptions {
    use std::{error::Error, fmt::Display};

    use jloader::errors::class_format_check::FormatError;
    #[derive(Debug)]
    pub enum Exception {
        ClassNotFound,
        JLoader(FormatError),
        Other(Box<dyn Error>),
    }

    impl Display for Exception {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Exception::ClassNotFound => write!(f, "ClassNotFoundException"),
                Exception::JLoader(format_error) => {
                    write!(f, "Format Error {}", format_error)
                }
                Exception::Other(err) => write!(f, "{:?}", err),
            }
        }
    }

    #[derive(Debug)]
    pub struct ExceptionError {
        exception: Exception,
        msg: String,
    }

    impl ExceptionError {
        pub fn new(exception: Exception, msg: &str) -> ExceptionError {
            ExceptionError {
                exception,
                msg: msg.into(),
            }
        }
    }

    impl Error for ExceptionError {}

    impl Display for ExceptionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Exception: {}, {}", self.exception, self.msg)
        }
    }
}
