use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::FunctionType;
use inkwell::values::FunctionValue;
use std::path::Path;

mod error;

pub use error::{EvaLLVMError, Result};

pub struct EvaLLVM<'ctx> {
    /// Container of module and other metadata
    context: &'ctx Context,
    /// Container for functions, globals, constants, etc.
    module: Module<'ctx>,
    /// APIs for building IR instructions
    builder: Builder<'ctx>,
    /// Curent function being built
    function: Option<FunctionValue<'ctx>>,
}

impl<'ctx> EvaLLVM<'ctx> {
    pub fn new(ctx: &'ctx Context) -> Self {
        Self {
            context: ctx,
            module: ctx.create_module("evallvm"),
            builder: ctx.create_builder(),
            function: None,
        }
    }

    /// Creates a function prototype in the module
    fn create_function_proto(
        &self,
        fn_name: &str,
        fn_type: FunctionType<'ctx>,
    ) -> Result<inkwell::values::FunctionValue<'ctx>> {
        let fn_val = self.module.add_function(fn_name, fn_type, None);
        if !fn_val.verify(false) {
            return Err(EvaLLVMError::FunctionError {
                func_name: fn_name.to_string(),
            });
        }
        Ok(fn_val)
    }

    /// Creates a function in the module, or returns the existing one if it already exists
    fn create_function(
        &self,
        fn_name: &str,
        fn_type: FunctionType<'ctx>,
    ) -> Result<inkwell::values::FunctionValue<'ctx>> {
        let fn_val = if self.module.get_function(fn_name).is_none() {
            self.create_function_proto(fn_name, fn_type)?
        } else {
            self.module.get_function(fn_name).unwrap()
        };
        self.create_function_block(fn_val)?;
        Ok(fn_val)
    }

    /// Creates a function block within the specified function as parent
    fn create_basic_block(
        &self,
        parent: FunctionValue<'ctx>,
        block_name: &str,
    ) -> BasicBlock<'ctx> {
        self.context.append_basic_block(parent, block_name)
    }

    /// Creates a basic block with the specified function as parent
    fn create_function_block(&self, parent: FunctionValue<'ctx>) -> Result<()> {
        let block = self.create_basic_block(parent, "entry");
        // Set entry point for the function
        self.builder.position_at_end(block);
        Ok(())
    }

    /// Compiles program
    pub fn compile(&mut self, _program: &str) -> Result<()> {
        let i32_type = self.context.i32_type();
        let fn_type = i32_type.fn_type(&[], false);
        let i32_val = i32_type.const_int(42, false);
        let fn_main = self.create_function("main", fn_type)?;
        self.function = Some(fn_main);

        self.builder.build_return(Some(&i32_val)).unwrap();
        Ok(())
    }

    /// Executes the program by compiling it and saving the module to a file
    pub fn exec(&mut self, program: &str) -> Result<()> {
        self.compile(program)?;
        self.save_module_to_file("out")?;
        Ok(())
    }

    /// Saves the module to a file in both bitcode and LLVM IR formats
    pub fn save_module_to_file(&self, file: &str) -> Result<()> {
        let file_bc = format!("{}.bc", file);
        let file_ll = format!("{}.ll", file);

        if !self
            .module
            .write_bitcode_to_path(&Path::new(file_bc.as_str()))
        {
            return Err(EvaLLVMError::BitcodeWriteError { file: file_bc });
        }

        self.module
            .print_to_file(&file_ll)
            .map_err(|source| EvaLLVMError::IRWriteError {
                file: file_ll.clone(),
                message: source.to_string(),
            })?;

        self.module.print_to_stderr();
        Ok(())
    }
}
