use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum EvaLLVMError {
    #[error("Failed to write bitcode file '{file}'")]
    BitcodeWriteError { file: String },

    #[error("Failed to write LLVM IR file '{file}': {message}")]
    IRWriteError { file: String, message: String },

    #[error("Failed to create function: {func_name}")]
    FunctionError { func_name: String },
}

pub type Result<T> = std::result::Result<T, EvaLLVMError>;
