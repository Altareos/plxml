use super::{Context, Instruction};
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Function {
    pub args: Vec<String>,
    pub ins: Vec<Instruction>,
}

impl Function {
    pub fn run(
        &self,
        args: Vec<Value>,
        ctx: &mut Context,
    ) -> Result<Option<Value>, Box<dyn Error>> {
        if args.len() != self.args.len() {
            Err("not enough arguments")?;
        }
        self.args
            .iter()
            .zip(args.into_iter())
            .for_each(|(p, a)| ctx.assign(p.clone(), a));
        for i in self.ins.iter() {
            i.run(ctx)?;
        }
        Ok(ctx.take(&String::from("__return")))
    }
}

#[derive(Clone, Debug)]
pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Array(Rc<RefCell<Vec<Value>>>),
    Function(Function),
    StdFunction(fn(Vec<Value>) -> Result<Option<Value>, Box<dyn Error>>),
}

impl Value {
    pub fn to_bool(&self) -> bool {
        match self {
            Value::Integer(i) => *i != 0,
            Value::Float(f) => *f != 0.0,
            Value::String(s) => s.len() != 0,
            Value::Array(v) => v.borrow().len() != 0,
            _ => true,
        }
    }
}
