use inkwell::builder::BuilderError;
use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum EvaLLVMError {
    #[error("Failed to write bitcode file '{file}'")]
    BitcodeWriteError { file: String },

    #[error("Failed to write LLVM IR file '{file}': {message}")]
    IRWriteError { file: String, message: String },

    #[error("{message}: {func_name}")]
    FunctionError { message: String, func_name: String },

    #[error("Builder error: {0}")]
    BuilderError(#[from] BuilderError),
}

pub type Result<T> = std::result::Result<T, EvaLLVMError>;
