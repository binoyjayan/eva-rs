mod evallvm;

use inkwell::context::Context;
use miette::Result;

use evallvm::EvaLLVM;

fn main() -> Result<()> {
    let context = Context::create();
    let mut eva = EvaLLVM::new(&context).init()?;
    eva.exec("42")?;
    Ok(())
}
