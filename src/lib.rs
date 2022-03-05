use std::error::Error;
use std::fs;

use roxmltree::{Document, Node};

mod context;
mod instruction;
mod stl;
mod util;
mod value;

use context::Context;
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
        .ok_or("invalid program structure")?
        .children()
        .find(|node| util::tag_name(&node) == "main")
        .ok_or("No 'main' block")?;
    let main_ast = Instruction::from_children(main)?;

    let functions = root
        .first_element_child()
        .ok_or("invalid program structure")?
        .children()
        .filter(|node| node.tag_name().name() == String::from("function"));

    for fun in functions {
        ctx.assign(
            String::from(fun.attribute("name").ok_or("unnamed function")?),
            Value::Function(Function {
                args: util::find_node(&fun, "arguments")
                    .ok_or("missing 'arguments' block in 'call' tag")?
                    .children()
                    .filter(Node::is_element)
                    .map(|n| n.attribute("name").and_then(|s| Some(String::from(s))))
                    .collect::<Option<Vec<String>>>()
                    .ok_or("unnamed argument")?,
                ins: Instruction::from_children(
                    util::find_node(&fun, "body").ok_or("missing 'body' block in 'call' tag")?,
                )?,
            }),
        );
    }

    for ins in main_ast {
        ins.run(&mut ctx)?;
    }

    Ok(())
}
