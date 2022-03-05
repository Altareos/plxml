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

pub fn run(filename: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    let doc = Document::parse(&contents)?;
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

    for ins in main_ast {
        ins.run(&mut ctx)?;
    }

    Ok(())
}
