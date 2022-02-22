use roxmltree::{Document, Node};
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::rc::Rc;

mod util {
    use super::{Error, Node};
    pub fn tag_name(node: &Node) -> String {
        node.tag_name().name().to_lowercase()
    }

    pub fn find_node<'a>(
        node: &'a Node<'a, 'a>,
        tag: &str,
    ) -> Result<Node<'a, 'a>, Box<dyn Error>> {
        Ok(node
            .children()
            .find(|n| tag_name(n) == tag)
            .ok_or(format!("node '{}' not found", tag))?)
    }
}

#[derive(Clone, Debug)]
pub enum Instruction {
    Value(String),
    Assign(String, Box<Instruction>),
    Integer(String),
    Float(String),
    String(String),
    Array(Vec<Instruction>),
    Add(Vec<Instruction>),
    Subtract(Vec<Instruction>),
    Multiply(Vec<Instruction>),
    Divide(Vec<Instruction>),
    And(Vec<Instruction>),
    Or(Vec<Instruction>),
    Not(Box<Instruction>),
    Call(Box<Instruction>, Vec<Instruction>),
    CallNamed(String, Vec<Instruction>),
    Return(Box<Instruction>),
    If(Box<Instruction>, Vec<Instruction>),
    IfElse(Box<Instruction>, Vec<Instruction>, Vec<Instruction>),
    For {
        variable: String,
        from: Box<Instruction>,
        to: Box<Instruction>,
        step: Box<Instruction>,
        body: Vec<Instruction>,
    },
    Each(String, Box<Instruction>, Vec<Instruction>),
    While(Box<Instruction>, Vec<Instruction>),
    Print(Box<Instruction>),
    AssignArray(Box<Instruction>, Box<Instruction>, Box<Instruction>),
    InsertArray(Box<Instruction>, Box<Instruction>),
}

impl Instruction {
    pub fn new(node: Node) -> Result<Instruction, Box<dyn Error>> {
        // println!("parsing '{}'", util::tag_name(&node));
        Ok(match util::tag_name(&node).as_str() {
            "value" => Instruction::Value(
                node.attribute("variable")
                    .and_then(|a| Some(String::from(a)))
                    .ok_or("missing 'variable' attribute on 'value' tag")?,
            ),
            "assign" => Instruction::Assign(
                String::from(
                    node.attribute("variable")
                        .ok_or("missing 'variable' attribute on 'assign' tag")?,
                ),
                Box::new(Instruction::new(
                    node.first_element_child()
                        .ok_or("missing child on 'assign' tag")?,
                )?),
            ),
            "integer" => Instruction::Integer(
                node.attribute("value")
                    .ok_or("missing 'value' attribute on 'integer' tag")?
                    .parse()?,
            ),
            "float" => Instruction::Float(
                node.attribute("value")
                    .ok_or("missing 'value' attribute on 'float' tag")?
                    .parse()?,
            ),
            "string" => Instruction::String(String::from(
                node.attribute("value")
                    .ok_or("missing 'value' attribute on 'string' tag")?,
            )),
            "array" => Instruction::Array(Instruction::from_children(node)?),
            "add" => Instruction::Add(Instruction::from_children(node)?),
            "subtract" => Instruction::Subtract(Instruction::from_children(node)?),
            "multiply" => Instruction::Multiply(Instruction::from_children(node)?),
            "divide" => Instruction::Divide(Instruction::from_children(node)?),
            "and" => Instruction::And(Instruction::from_children(node)?),
            "or" => Instruction::Or(Instruction::from_children(node)?),
            "not" => Instruction::Not(Box::new(Instruction::new(
                node.first_element_child()
                    .ok_or("missing value child element in 'not' tag")?,
            )?)),
            "call" => {
                if let Some(function) = node.attribute("function") {
                    Instruction::CallNamed(
                        String::from(function),
                        Instruction::from_children(util::find_node(&node, "arguments")?)?,
                    )
                } else {
                    Instruction::Call(
                        Box::new(Instruction::new(
                            node.first_element_child()
                                .ok_or("missing function child element in 'call' tag")?,
                        )?),
                        Instruction::from_children(util::find_node(&node, "arguments")?)?,
                    )
                }
            }
            "return" => Instruction::Return(Box::new(Instruction::new(
                node.first_element_child()
                    .ok_or("missing value child element in 'return' tag")?,
            )?)),
            "if" => {
                if let Some(else_node) = node.children().find(|n| util::tag_name(n) == "else") {
                    Instruction::IfElse(
                        Box::new(Instruction::new(
                            node.first_element_child()
                                .ok_or("missing condition child element in 'if' tag")?,
                        )?),
                        Instruction::from_children(util::find_node(&node, "then")?)?,
                        Instruction::from_children(else_node)?,
                    )
                } else {
                    Instruction::If(
                        Box::new(Instruction::new(
                            node.first_element_child()
                                .ok_or("missing condition child element in 'if' tag")?,
                        )?),
                        Instruction::from_children(util::find_node(&node, "then")?)?,
                    )
                }
            }
            "for" => Instruction::For {
                variable: String::from(
                    node.attribute("variable")
                        .ok_or("missing 'variable' attribute on 'for' tag")?,
                ),
                from: Box::new(Instruction::new(
                    util::find_node(&node, "from")?
                        .first_element_child()
                        .ok_or("missing 'from' child in 'for' tag")?,
                )?),
                to: Box::new(Instruction::new(
                    util::find_node(&node, "to")?
                        .first_element_child()
                        .ok_or("missing 'to' child in 'for' tag")?,
                )?),
                step: Box::new(Instruction::new(
                    util::find_node(&node, "step")?
                        .first_element_child()
                        .ok_or("missing 'step' child in 'for' tag")?,
                )?),
                body: Instruction::from_children(util::find_node(&node, "do")?)?,
            },
            "each" => Instruction::Each(
                String::from(
                    node.attribute("variable")
                        .ok_or("missing 'variable' attribute on 'each' tag")?,
                ),
                Box::new(Instruction::new(
                    node.first_element_child()
                        .ok_or("missing array child element in 'for' tag")?,
                )?),
                Instruction::from_children(util::find_node(&node, "do")?)?,
            ),
            "while" => Instruction::While(
                Box::new(Instruction::new(
                    node.first_element_child()
                        .ok_or("missing condition child element in 'while' tag")?,
                )?),
                Instruction::from_children(util::find_node(&node, "do")?)?,
            ),
            "print" => Instruction::Print(Box::new(Instruction::new(
                node.first_element_child()
                    .ok_or("missing value child element in 'print' tag")?,
            )?)),
            "assign-array" => Instruction::AssignArray(
                Box::new(Instruction::new(
                    util::find_node(&node, "array")?
                        .first_element_child()
                        .ok_or("missing 'array' in 'assign-array' tag")?,
                )?),
                Box::new(Instruction::new(
                    util::find_node(&node, "index")?
                        .first_element_child()
                        .ok_or("missing 'index' in 'assign-array' tag")?,
                )?),
                Box::new(Instruction::new(
                    util::find_node(&node, "value")?
                        .first_element_child()
                        .ok_or("missing 'value' in 'assign-array' tag")?,
                )?),
            ),
            "insert-array" => Instruction::InsertArray(
                Box::new(Instruction::new(
                    util::find_node(&node, "array")?
                        .first_element_child()
                        .ok_or("missing 'array' in 'insert-array' tag")?,
                )?),
                Box::new(Instruction::new(
                    util::find_node(&node, "value")?
                        .first_element_child()
                        .ok_or("missing 'value' in 'insert-array' tag")?,
                )?),
            ),
            tag => Err(format!("unknown tag '{}'", tag))?,
        })
    }

    pub fn from_children(node: Node) -> Result<Vec<Instruction>, Box<dyn Error>> {
        node.children()
            .filter(Node::is_element)
            .map(Instruction::new)
            .collect()
    }

    fn add(vals: Vec<Value>) -> Result<Value, Box<dyn Error>> {
        if vals.iter().all(|v| matches!(v, Value::Integer(_))) {
            Ok(Value::Integer(
                vals.iter()
                    .map(|v| {
                        if let Value::Integer(i) = v {
                            Ok(i)
                        } else {
                            Err("invalid value")?
                        }
                    })
                    .sum::<Result<i64, Box<dyn Error>>>()?,
            ))
        } else if vals
            .iter()
            .all(|v| matches!(v, Value::Integer(_)) || matches!(v, Value::Float(_)))
        {
            Ok(Value::Float(
                vals.iter()
                    .map(|v| {
                        if let Value::Integer(i) = v {
                            Ok(*i as f64)
                        } else if let Value::Float(f) = v {
                            Ok(*f)
                        } else {
                            Err("invalid value")?
                        }
                    })
                    .sum::<Result<f64, Box<dyn Error>>>()?,
            ))
        } else if vals.iter().all(|v| {
            matches!(v, Value::Integer(_))
                || matches!(v, Value::Float(_))
                || matches!(v, Value::String(_))
        }) {
            Ok(Value::String(
                vals.iter()
                    .map(|v| {
                        Ok(if let Value::String(s) = v {
                            s.to_string()
                        } else if let Value::Integer(i) = v {
                            i.to_string()
                        } else if let Value::Float(f) = v {
                            f.to_string()
                        } else {
                            Err("invalid value")?
                        })
                    })
                    .collect::<Result<Vec<String>, Box<dyn Error>>>()?
                    .join(""),
            ))
        } else {
            Err("invalid value")?
        }
    }

    fn subtract(vals: Vec<Value>) -> Result<Value, Box<dyn Error>> {
        Ok(if vals.iter().all(|v| matches!(v, Value::Integer(_))) {
            let first = match vals.first().ok_or("missing values in 'subtract' tag")? {
                Value::Integer(i) => i,
                _ => Err("invalid value")?,
            };
            Value::Integer(
                *first
                    - vals
                        .iter()
                        .skip(1)
                        .map(|v| {
                            if let Value::Integer(i) = v {
                                Ok(i)
                            } else {
                                Err("invalid value")?
                            }
                        })
                        .sum::<Result<i64, Box<dyn Error>>>()?,
            )
        } else if vals
            .iter()
            .all(|v| matches!(v, Value::Integer(_)) || matches!(v, Value::Float(_)))
        {
            let first = match vals.first().ok_or("not enough values in 'subtract' tag")? {
                Value::Integer(v) => *v as f64,
                Value::Float(v) => *v,
                _ => Err("invalid value")?,
            };
            Value::Float(
                first
                    - vals
                        .iter()
                        .skip(1)
                        .map(|val| {
                            Ok(match val {
                                Value::Integer(v) => *v as f64,
                                Value::Float(v) => *v,
                                _ => Err("invalid value")?,
                            })
                        })
                        .sum::<Result<f64, Box<dyn Error>>>()?,
            )
        } else {
            Err("invalid value")?
        })
    }

    fn multiply(vals: Vec<Value>) -> Result<Value, Box<dyn Error>> {
        if vals.iter().all(|v| matches!(v, Value::Integer(_))) {
            Ok(Value::Integer(
                vals.iter()
                    .map(|v| {
                        if let Value::Integer(i) = v {
                            Ok(i)
                        } else {
                            Err("invalid value")?
                        }
                    })
                    .product::<Result<i64, Box<dyn Error>>>()?,
            ))
        } else if vals
            .iter()
            .all(|v| matches!(v, Value::Integer(_)) || matches!(v, Value::Float(_)))
        {
            Ok(Value::Float(
                vals.iter()
                    .map(|val| {
                        Ok(match val {
                            Value::Integer(v) => *v as f64,
                            Value::Float(v) => *v,
                            _ => Err("invalid value")?,
                        })
                    })
                    .product::<Result<f64, Box<dyn Error>>>()?,
            ))
        } else {
            Err("invalid value")?
        }
    }

    fn divide(vals: Vec<Value>) -> Result<Value, Box<dyn Error>> {
        if vals
            .iter()
            .all(|v| matches!(v, Value::Integer(_)) || matches!(v, Value::Float(_)))
        {
            let first = match vals.first().ok_or("not enough values in 'divide' tag")? {
                Value::Integer(v) => *v as f64,
                Value::Float(v) => *v,
                _ => Err("invalid value")?,
            };
            Ok(Value::Float(
                first
                    * vals
                        .iter()
                        .skip(1)
                        .map(|val| {
                            Ok(match val {
                                Value::Integer(v) => 1.0 / (*v as f64),
                                Value::Float(v) => 1.0 / *v,
                                _ => Err("invalid value")?,
                            })
                        })
                        .product::<Result<f64, Box<dyn Error>>>()?,
            ))
        } else {
            Err("invalid value")?
        }
    }

    fn and(vals: Vec<Value>) -> Value {
        Value::Integer(if vals.iter().all(Value::to_bool) {
            1
        } else {
            0
        })
    }

    fn or(vals: Vec<Value>) -> Value {
        Value::Integer(if vals.iter().any(Value::to_bool) {
            1
        } else {
            0
        })
    }

    fn run_all(
        ins: &Vec<Instruction>,
        ctx: &mut Context,
    ) -> Result<Option<Vec<Value>>, Box<dyn Error>> {
        ins.iter().map(|i| i.run(ctx)).collect()
    }

    pub fn run(&self, ctx: &mut Context) -> Result<Option<Value>, Box<dyn Error>> {
        Ok(if let None = ctx.value(&String::from("__return")) {
            match self {
                Instruction::Value(key) => {
                    Some(match ctx.value(key).ok_or("unknown variable")? {
                        Value::Array(vecrc) => Value::Array(Rc::clone(vecrc)),
                        val => val.clone(),
                    })
                }
                Instruction::Assign(key, ins) => {
                    let v = ins.run(ctx)?.ok_or("invalid child value in 'assign' tag")?;
                    ctx.assign(key.clone(), v);
                    None
                }
                Instruction::Integer(val) => Some(Value::Integer(val.parse()?)),
                Instruction::Float(val) => Some(Value::Float(val.parse()?)),
                Instruction::String(val) => Some(Value::String(val.clone())),
                Instruction::Array(args) => Some(Value::Array(Rc::new(RefCell::new(
                    Instruction::run_all(args, ctx)?
                        .ok_or("invalid child values in 'array' tag")?,
                )))),
                Instruction::Add(args) => {
                    let vals = Instruction::run_all(args, ctx)?
                        .ok_or("invalid child values in 'add' tag")?;
                    Some(Instruction::add(vals)?)
                }
                Instruction::Subtract(args) => {
                    let vals = Instruction::run_all(args, ctx)?
                        .ok_or("invalid child values in 'subtract' tag")?;
                    Some(Instruction::subtract(vals)?)
                }
                Instruction::Multiply(args) => {
                    let vals = Instruction::run_all(args, ctx)?
                        .ok_or("invalid child values in 'multiply' tag")?;
                    Some(Instruction::multiply(vals)?)
                }
                Instruction::Divide(args) => {
                    let vals = Instruction::run_all(args, ctx)?
                        .ok_or("invalid child values in 'divide' tag")?;
                    Some(Instruction::divide(vals)?)
                }
                Instruction::And(args) => {
                    let vals = Instruction::run_all(args, ctx)?
                        .ok_or("invalid child values in 'and' tag")?;
                    Some(Instruction::and(vals))
                }
                Instruction::Or(args) => {
                    let vals = Instruction::run_all(args, ctx)?
                        .ok_or("invalid child values in 'or' tag")?;
                    Some(Instruction::or(vals))
                }
                Instruction::Not(arg) => Some(Value::Integer(
                    if arg
                        .run(ctx)?
                        .ok_or("invalid child value in 'not' tag")?
                        .to_bool()
                    {
                        1
                    } else {
                        0
                    },
                )),
                Instruction::Call(fct_ins, args) => {
                    let vals = Instruction::run_all(args, ctx)?
                        .ok_or("invalid argument values in 'call' tag")?;
                    let fct_val = fct_ins
                        .run(ctx)?
                        .ok_or("invalid child function in 'call' tag")?;
                    if let Value::Function(f) = fct_val {
                        f.run(vals, ctx)?
                    } else {
                        Err("invalid function")?
                    }
                }
                Instruction::CallNamed(fct_name, args) => {
                    let vals: Vec<Value> = Instruction::run_all(args, ctx)?
                        .ok_or("invalid argument values in 'or' tag")?;
                    let fct_val = ctx.value(&fct_name).ok_or("unknown function")?;
                    if let Value::Function(f) = fct_val {
                        let mut local = ctx.clone();
                        f.run(vals, &mut local)?
                    } else {
                        Err("invalid function")?
                    }
                }
                Instruction::Return(ins) => {
                    let v = ins.run(ctx)?.ok_or("invalid child value in 'return' tag")?;
                    ctx.assign(String::from("__return"), v);
                    None
                }
                Instruction::If(cond, then) => {
                    if cond
                        .run(ctx)?
                        .ok_or("invalid condition value in 'if' tag")?
                        .to_bool()
                    {
                        for i in then {
                            i.run(ctx)?;
                        }
                    }
                    None
                }
                Instruction::IfElse(cond, then, els) => {
                    if cond
                        .run(ctx)?
                        .ok_or("invalid condition value in 'if' tag")?
                        .to_bool()
                    {
                        for i in then {
                            i.run(ctx)?;
                        }
                    } else {
                        for i in els {
                            i.run(ctx)?;
                        }
                    }
                    None
                }
                Instruction::For {
                    variable,
                    from,
                    to,
                    step,
                    body,
                } => {
                    if let Value::Integer(f) =
                        from.run(ctx)?.ok_or("invalid 'from' value in 'for' tag")?
                    {
                        if let Value::Integer(t) =
                            to.run(ctx)?.ok_or("invalid 'to' value in 'for' tag")?
                        {
                            if let Value::Integer(s) =
                                step.run(ctx)?.ok_or("invalid 'from' value in 'for' tag")?
                            {
                                for i in (f..t).step_by(s.try_into()?) {
                                    ctx.assign(variable.clone(), Value::Integer(i));
                                    for ins in body {
                                        ins.run(ctx)?;
                                    }
                                }
                            }
                        }
                    }
                    None
                }
                Instruction::Each(variable, array_ins, body) => {
                    if let Value::Array(v) = array_ins
                        .run(ctx)?
                        .ok_or("invalid array value in 'each' tag")?
                    {
                        for i in v.borrow().iter() {
                            ctx.assign(variable.clone(), i.clone());
                            for ins in body {
                                ins.run(ctx)?;
                            }
                        }
                    } else {
                        Err("invalid array")?
                    }
                    None
                }
                Instruction::While(cond, body) => {
                    while cond
                        .run(ctx)?
                        .ok_or("invalid condition value in 'while' tag")?
                        .to_bool()
                    {
                        for ins in body {
                            ins.run(ctx)?;
                        }
                    }
                    None
                }
                Instruction::Print(ins) => {
                    match ins.run(ctx)?.ok_or("invalid child value in 'print' tag")? {
                        Value::Integer(i) => println!("{}", i),
                        Value::Float(f) => println!("{}", f),
                        Value::String(s) => println!("{}", s),
                        _ => Err("Unprintable value")?,
                    };
                    None
                }
                Instruction::AssignArray(array_ins, index_ins, value_ins) => {
                    if let Some(Value::Array(vec)) = array_ins.run(ctx)? {
                        if let Some(Value::Integer(index)) = index_ins.run(ctx)? {
                            vec.borrow_mut().insert(
                                index.try_into()?,
                                value_ins
                                    .run(ctx)?
                                    .ok_or("invalid 'value' value in 'assign-array' tag")?,
                            );
                        }
                    }
                    None
                }
                Instruction::InsertArray(array_ins, value_ins) => {
                    if let Some(Value::Array(vec)) = array_ins.run(ctx)? {
                        vec.borrow_mut().push(
                            value_ins
                                .run(ctx)?
                                .ok_or("invalid 'value' value in 'insert-array' tag")?,
                        );
                    }
                    None
                }
            }
        } else {
            None
        })
    }
}

#[derive(Clone, Debug)]
pub struct Function {
    args: Vec<String>,
    ins: Vec<Instruction>,
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
}

impl Value {
    fn to_bool(&self) -> bool {
        match self {
            Value::Integer(i) => *i != 0,
            Value::Float(f) => *f != 0.0,
            Value::String(s) => s.len() != 0,
            Value::Array(v) => v.borrow().len() != 0,
            _ => true,
        }
    }
}

#[derive(Clone)]
pub struct Context {
    dict: HashMap<String, Value>,
    parent: Option<Box<Context>>,
}

impl Context {
    pub fn new(parent: Option<Box<Context>>) -> Context {
        Context {
            dict: HashMap::new(),
            parent,
        }
    }

    pub fn assign(&mut self, key: String, value: Value) {
        self.dict.insert(key, value);
    }

    pub fn value(&self, key: &String) -> Option<&Value> {
        self.dict
            .get(key)
            .or(self.parent.as_ref().and_then(|p| p.value(key)))
    }

    pub fn take(&mut self, key: &String) -> Option<Value> {
        self.dict.remove(key)
    }
}

pub fn run(filename: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    let doc = Document::parse(&contents)?;
    let root = doc.root();

    let mut ctx = Context::new(None);

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
                args: util::find_node(&fun, "arguments")?
                    .children()
                    .filter(Node::is_element)
                    .map(|n| n.attribute("name").and_then(|s| Some(String::from(s))))
                    .collect::<Option<Vec<String>>>()
                    .ok_or("unnamed argument")?,
                ins: Instruction::from_children(util::find_node(&fun, "body")?)?,
            }),
        );
    }

    for ins in main_ast {
        ins.run(&mut ctx)?;
    }

    Ok(())
}
