use std::error::Error;
use std::fs;

use roxmltree::Document;

mod context;
mod error;
mod instruction;
mod stl;
mod util;
mod value;

use context::Context;
use error::{InvalidProgram, MissingChild, Unnamed};
use instruction::Instruction;
use value::{Function, Value};

pub fn run_file(filename: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    run(contents)
}

pub fn run(program: String) -> Result<(), Box<dyn Error>> {
    let doc = Document::parse(&program)?;
    let root = doc.root();

    let mut ctx = Context::new(None);
    stl::inject_all(&mut ctx);

    let main = root
        .first_element_child()
        .ok_or(InvalidProgram)?
        .children()
        .find(|node| util::tag_name(&node) == "main")
        .ok_or(MissingChild("program", "main"))?;
    let main_ast = Instruction::from_children(main)?;

    let functions = root
        .first_element_child()
        .ok_or(InvalidProgram)?
        .children()
        .filter(|node| node.tag_name().name() == String::from("function"));

    for fun in functions {
        ctx.assign(
            String::from(fun.attribute("name").ok_or(Unnamed("function"))?),
            Value::Function(Function::from(&fun)?),
        );
    }

    let mut main_ctx = Context::new(Some(&ctx));

    for ins in main_ast {
        ins.run(&mut main_ctx, &ctx)?;
    }

    Ok(())
}
