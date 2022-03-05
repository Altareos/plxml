use super::error::{
    BadChildCount, IncompatibleValues, InvalidValue, MissingAttribute, MissingChild,
    UnknownVariable,
};
use super::{util, Context, Value};
use roxmltree::Node;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum Instruction {
    Value(String),
    Assign(String, Box<Instruction>),
    Integer(String),
    IntegerCast(Box<Instruction>),
    Float(String),
    FloatCast(Box<Instruction>),
    String(String),
    StringCast(Box<Instruction>),
    Array(Vec<Instruction>),
    Add(Vec<Instruction>),
    Subtract(Vec<Instruction>),
    Multiply(Vec<Instruction>),
    Divide(Vec<Instruction>),
    And(Vec<Instruction>),
    Or(Vec<Instruction>),
    Not(Box<Instruction>),
    Equal(Box<Instruction>, Box<Instruction>),
    Greater(Box<Instruction>, Box<Instruction>),
    Lower(Box<Instruction>, Box<Instruction>),
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
}

impl Instruction {
    pub fn new(node: Node) -> Result<Instruction, Box<dyn Error>> {
        Ok(match util::tag_name(&node).as_str() {
            "value" => Instruction::Value(
                node.attribute("variable")
                    .and_then(|a| Some(String::from(a)))
                    .ok_or(MissingAttribute("value", "variable"))?,
            ),
            "assign" => Instruction::Assign(
                String::from(
                    node.attribute("variable")
                        .ok_or(MissingAttribute("assign", "variable"))?,
                ),
                Box::new(Instruction::new(
                    node.first_element_child()
                        .ok_or(MissingChild("assign", "value"))?,
                )?),
            ),
            "integer" => {
                if let Some(v) = node.attribute("value") {
                    Instruction::Integer(String::from(v))
                } else if let Some(n) = node.first_element_child() {
                    Instruction::IntegerCast(Box::new(Instruction::new(n)?))
                } else {
                    Err(MissingAttribute("integer", "value"))?
                }
            }
            "float" => {
                if let Some(v) = node.attribute("value") {
                    Instruction::Float(String::from(v))
                } else if let Some(n) = node.first_element_child() {
                    Instruction::FloatCast(Box::new(Instruction::new(n)?))
                } else {
                    Err(MissingAttribute("float", "value"))?
                }
            }
            "string" => {
                if let Some(v) = node.attribute("value") {
                    Instruction::String(String::from(v))
                } else if let Some(n) = node.first_element_child() {
                    Instruction::StringCast(Box::new(Instruction::new(n)?))
                } else {
                    Err(MissingAttribute("string", "value"))?
                }
            }
            "array" => Instruction::Array(Instruction::from_children(node)?),
            "add" => Instruction::Add(Instruction::from_children(node)?),
            "subtract" => Instruction::Subtract(Instruction::from_children(node)?),
            "multiply" => Instruction::Multiply(Instruction::from_children(node)?),
            "divide" => Instruction::Divide(Instruction::from_children(node)?),
            "and" => Instruction::And(Instruction::from_children(node)?),
            "or" => Instruction::Or(Instruction::from_children(node)?),
            "not" => Instruction::Not(Box::new(Instruction::new(
                node.first_element_child()
                    .ok_or(MissingAttribute("not", "value"))?,
            )?)),
            "equal" => {
                let children: Vec<Node> = node.children().filter(Node::is_element).collect();
                if children.len() == 2 {
                    Instruction::Equal(
                        Box::new(Instruction::new(children[0])?),
                        Box::new(Instruction::new(children[1])?),
                    )
                } else {
                    Err(BadChildCount("equal", children.len()))?
                }
            }
            "greater" => {
                let children: Vec<Node> = node.children().filter(Node::is_element).collect();
                if children.len() == 2 {
                    Instruction::Greater(
                        Box::new(Instruction::new(children[0])?),
                        Box::new(Instruction::new(children[1])?),
                    )
                } else {
                    Err(BadChildCount("greater", children.len()))?
                }
            }
            "lower" => {
                let children: Vec<Node> = node.children().filter(Node::is_element).collect();
                if children.len() == 2 {
                    Instruction::Lower(
                        Box::new(Instruction::new(children[0])?),
                        Box::new(Instruction::new(children[1])?),
                    )
                } else {
                    Err(BadChildCount("lower", children.len()))?
                }
            }
            "call" => {
                if let Some(function) = node.attribute("function") {
                    Instruction::CallNamed(
                        String::from(function),
                        Instruction::from_children(
                            util::find_node(&node, "arguments")
                                .ok_or(MissingChild("call", "arguments"))?,
                        )?,
                    )
                } else {
                    Instruction::Call(
                        Box::new(Instruction::new(
                            node.first_element_child()
                                .ok_or(MissingChild("call", "function"))?,
                        )?),
                        Instruction::from_children(
                            util::find_node(&node, "arguments")
                                .ok_or(MissingChild("call", "arguments"))?,
                        )?,
                    )
                }
            }
            "return" => Instruction::Return(Box::new(Instruction::new(
                node.first_element_child()
                    .ok_or(MissingChild("return", "value"))?,
            )?)),
            "if" => {
                if let Some(else_node) = node.children().find(|n| util::tag_name(n) == "else") {
                    Instruction::IfElse(
                        Box::new(Instruction::new(
                            node.first_element_child()
                                .ok_or(MissingChild("if", "condition"))?,
                        )?),
                        Instruction::from_children(
                            util::find_node(&node, "then").ok_or(MissingChild("if", "then"))?,
                        )?,
                        Instruction::from_children(else_node)?,
                    )
                } else {
                    Instruction::If(
                        Box::new(Instruction::new(
                            node.first_element_child()
                                .ok_or(MissingChild("if", "condition"))?,
                        )?),
                        Instruction::from_children(
                            util::find_node(&node, "then").ok_or(MissingChild("if", "then"))?,
                        )?,
                    )
                }
            }
            "for" => Instruction::For {
                variable: String::from(
                    node.attribute("variable")
                        .ok_or(MissingAttribute("for", "variable"))?,
                ),
                from: Box::new(Instruction::new(
                    util::find_node(&node, "from")
                        .and_then(|n| n.first_element_child())
                        .ok_or(MissingChild("for", "from"))?,
                )?),
                to: Box::new(Instruction::new(
                    util::find_node(&node, "to")
                        .and_then(|n| n.first_element_child())
                        .ok_or(MissingChild("for", "to"))?,
                )?),
                step: Box::new(Instruction::new(
                    util::find_node(&node, "step")
                        .and_then(|n| n.first_element_child())
                        .ok_or(MissingChild("for", "step"))?,
                )?),
                body: Instruction::from_children(
                    util::find_node(&node, "do").ok_or(MissingChild("for", "do"))?,
                )?,
            },
            "each" => Instruction::Each(
                String::from(
                    node.attribute("variable")
                        .ok_or(MissingAttribute("each", "variable"))?,
                ),
                Box::new(Instruction::new(
                    node.first_element_child()
                        .ok_or(MissingChild("each", "array"))?,
                )?),
                Instruction::from_children(
                    util::find_node(&node, "do").ok_or(MissingChild("each", "from"))?,
                )?,
            ),
            "while" => Instruction::While(
                Box::new(Instruction::new(
                    node.first_element_child()
                        .ok_or(MissingChild("while", "condition"))?,
                )?),
                Instruction::from_children(
                    util::find_node(&node, "do").ok_or(MissingChild("while", "from"))?,
                )?,
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
                            Err(InvalidValue("add"))?
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
                            Err(InvalidValue("add"))?
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
                            Err(InvalidValue("add"))?
                        })
                    })
                    .collect::<Result<Vec<String>, Box<dyn Error>>>()?
                    .join(""),
            ))
        } else {
            Err(InvalidValue("add"))?
        }
    }

    fn subtract(vals: Vec<Value>) -> Result<Value, Box<dyn Error>> {
        Ok(if vals.iter().all(|v| matches!(v, Value::Integer(_))) {
            let first = match vals.first().ok_or(BadChildCount("subtract", 0usize))? {
                Value::Integer(i) => i,
                _ => Err(InvalidValue("subtract"))?,
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
                                Err(InvalidValue("subtract"))?
                            }
                        })
                        .sum::<Result<i64, Box<dyn Error>>>()?,
            )
        } else if vals
            .iter()
            .all(|v| matches!(v, Value::Integer(_)) || matches!(v, Value::Float(_)))
        {
            let first = match vals.first().ok_or(BadChildCount("subtract", 0usize))? {
                Value::Integer(v) => *v as f64,
                Value::Float(v) => *v,
                _ => Err(InvalidValue("subtract"))?,
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
                                _ => Err(InvalidValue("subtract"))?,
                            })
                        })
                        .sum::<Result<f64, Box<dyn Error>>>()?,
            )
        } else {
            Err(InvalidValue("subtract"))?
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
                            Err(InvalidValue("multiply"))?
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
                            _ => Err(InvalidValue("multiply"))?,
                        })
                    })
                    .product::<Result<f64, Box<dyn Error>>>()?,
            ))
        } else {
            Err(InvalidValue("multiply"))?
        }
    }

    fn divide(vals: Vec<Value>) -> Result<Value, Box<dyn Error>> {
        if vals
            .iter()
            .all(|v| matches!(v, Value::Integer(_)) || matches!(v, Value::Float(_)))
        {
            let first = match vals.first().ok_or(BadChildCount("divide", 0))? {
                Value::Integer(v) => *v as f64,
                Value::Float(v) => *v,
                _ => Err(InvalidValue("divide"))?,
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
                                _ => Err(InvalidValue("divide"))?,
                            })
                        })
                        .product::<Result<f64, Box<dyn Error>>>()?,
            ))
        } else {
            Err(InvalidValue("divide"))?
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

    fn compare(v1: Value, v2: Value) -> Result<i64, Box<dyn Error>> {
        use std::cmp::Ordering;
        match v1 {
            Value::Integer(i1) => match v2 {
                Value::Integer(i2) => Ok(i1 - i2),
                Value::Float(f2) => Ok(
                    match (i1 as f64).partial_cmp(&f2).ok_or(IncompatibleValues)? {
                        Ordering::Less => -1,
                        Ordering::Equal => 0,
                        Ordering::Greater => 1,
                    },
                ),
                _ => Err(IncompatibleValues)?,
            },
            Value::Float(f1) => match v2 {
                Value::Integer(i2) => Ok(
                    match f1.partial_cmp(&(i2 as f64)).ok_or(IncompatibleValues)? {
                        Ordering::Less => -1,
                        Ordering::Equal => 0,
                        Ordering::Greater => 1,
                    },
                ),
                Value::Float(f2) => Ok(match f1.partial_cmp(&f2).ok_or(IncompatibleValues)? {
                    Ordering::Less => -1,
                    Ordering::Equal => 0,
                    Ordering::Greater => 1,
                }),
                _ => Err(IncompatibleValues)?,
            },
            Value::String(s1) => {
                if let Value::String(s2) = v2 {
                    Ok(match s1.cmp(&s2) {
                        Ordering::Less => -1,
                        Ordering::Equal => 0,
                        Ordering::Greater => 1,
                    })
                } else {
                    Err(IncompatibleValues)?
                }
            }
            _ => Err(IncompatibleValues)?,
        }
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
                    Some(match ctx.value(key).ok_or(UnknownVariable(key.clone()))? {
                        Value::Array(vecrc) => Value::Array(Rc::clone(vecrc)),
                        val => val.clone(),
                    })
                }
                Instruction::Assign(key, ins) => {
                    let v = ins.run(ctx)?.ok_or(InvalidValue("assign"))?;
                    ctx.assign(key.clone(), v);
                    None
                }
                Instruction::Integer(val) => Some(Value::Integer(val.parse()?)),
                Instruction::IntegerCast(ins) => Some(Value::Integer(
                    match ins.run(ctx)?.ok_or(InvalidValue("integer"))? {
                        Value::Integer(i) => i,
                        Value::Float(f) => f as i64,
                        Value::String(s) => s.parse()?,
                        _ => Err(InvalidValue("integer"))?,
                    },
                )),
                Instruction::Float(val) => Some(Value::Float(val.parse()?)),
                Instruction::FloatCast(ins) => Some(Value::Float(
                    match ins.run(ctx)?.ok_or(InvalidValue("float"))? {
                        Value::Integer(i) => i as f64,
                        Value::Float(f) => f,
                        Value::String(s) => s.parse()?,
                        _ => Err(InvalidValue("float"))?,
                    },
                )),
                Instruction::String(val) => Some(Value::String(val.clone())),
                Instruction::StringCast(ins) => Some(Value::String(
                    match ins.run(ctx)?.ok_or(InvalidValue("string"))? {
                        Value::Integer(i) => i.to_string(),
                        Value::Float(f) => f.to_string(),
                        Value::String(s) => s,
                        _ => Err(InvalidValue("string"))?,
                    },
                )),
                Instruction::Array(args) => Some(Value::Array(Rc::new(RefCell::new(
                    Instruction::run_all(args, ctx)?.ok_or(InvalidValue("array"))?,
                )))),
                Instruction::Add(args) => {
                    let vals = Instruction::run_all(args, ctx)?.ok_or(InvalidValue("add"))?;
                    Some(Instruction::add(vals)?)
                }
                Instruction::Subtract(args) => {
                    let vals = Instruction::run_all(args, ctx)?.ok_or(InvalidValue("subtract"))?;
                    Some(Instruction::subtract(vals)?)
                }
                Instruction::Multiply(args) => {
                    let vals = Instruction::run_all(args, ctx)?.ok_or(InvalidValue("multiply"))?;
                    Some(Instruction::multiply(vals)?)
                }
                Instruction::Divide(args) => {
                    let vals = Instruction::run_all(args, ctx)?.ok_or(InvalidValue("divide"))?;
                    Some(Instruction::divide(vals)?)
                }
                Instruction::And(args) => {
                    let vals = Instruction::run_all(args, ctx)?.ok_or(InvalidValue("and"))?;
                    Some(Instruction::and(vals))
                }
                Instruction::Or(args) => {
                    let vals = Instruction::run_all(args, ctx)?.ok_or(InvalidValue("or"))?;
                    Some(Instruction::or(vals))
                }
                Instruction::Not(arg) => Some(Value::Integer(
                    if arg.run(ctx)?.ok_or(InvalidValue("not"))?.to_bool() {
                        0
                    } else {
                        1
                    },
                )),
                Instruction::Equal(v1, v2) => Some(Value::Integer(
                    if Instruction::compare(
                        v1.run(ctx)?.ok_or(InvalidValue("equal"))?,
                        v2.run(ctx)?.ok_or(InvalidValue("equal"))?,
                    )? == 0
                    {
                        1
                    } else {
                        0
                    },
                )),
                Instruction::Greater(v1, v2) => Some(Value::Integer(
                    if Instruction::compare(
                        v1.run(ctx)?.ok_or(InvalidValue("greater"))?,
                        v2.run(ctx)?.ok_or(InvalidValue("greater"))?,
                    )? > 0
                    {
                        1
                    } else {
                        0
                    },
                )),
                Instruction::Lower(v1, v2) => Some(Value::Integer(
                    if Instruction::compare(
                        v1.run(ctx)?.ok_or(InvalidValue("lower"))?,
                        v2.run(ctx)?.ok_or(InvalidValue("lower"))?,
                    )? < 0
                    {
                        1
                    } else {
                        0
                    },
                )),
                Instruction::Call(fct_ins, args) => {
                    let vals = Instruction::run_all(args, ctx)?.ok_or(InvalidValue("call"))?;
                    let fct_val = fct_ins.run(ctx)?.ok_or(InvalidValue("call"))?;
                    if let Value::Function(f) = fct_val {
                        f.run(vals, ctx)?
                    } else if let Value::StdFunction(f) = fct_val {
                        f(vals)?
                    } else {
                        Err(InvalidValue("call"))?
                    }
                }
                Instruction::CallNamed(fct_name, args) => {
                    let vals: Vec<Value> =
                        Instruction::run_all(args, ctx)?.ok_or(InvalidValue("call"))?;
                    let fct_val = ctx
                        .value(&fct_name)
                        .ok_or(UnknownVariable(fct_name.clone()))?;
                    if let Value::Function(f) = fct_val {
                        let mut local = ctx.clone();
                        f.run(vals, &mut local)?
                    } else if let Value::StdFunction(f) = fct_val {
                        f(vals)?
                    } else {
                        Err(InvalidValue("call"))?
                    }
                }
                Instruction::Return(ins) => {
                    let v = ins.run(ctx)?.ok_or(InvalidValue("return"))?;
                    ctx.assign(String::from("__return"), v);
                    None
                }
                Instruction::If(cond, then) => {
                    if cond.run(ctx)?.ok_or(InvalidValue("if"))?.to_bool() {
                        for i in then {
                            i.run(ctx)?;
                        }
                    }
                    None
                }
                Instruction::IfElse(cond, then, els) => {
                    if cond.run(ctx)?.ok_or(InvalidValue("if"))?.to_bool() {
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
                    if let Value::Integer(f) = from.run(ctx)?.ok_or(InvalidValue("for"))? {
                        if let Value::Integer(t) = to.run(ctx)?.ok_or(InvalidValue("for"))? {
                            if let Value::Integer(s) = step.run(ctx)?.ok_or(InvalidValue("for"))? {
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
                    if let Value::Array(v) = array_ins.run(ctx)?.ok_or(InvalidValue("each"))? {
                        for i in v.borrow().iter() {
                            ctx.assign(variable.clone(), i.clone());
                            for ins in body {
                                ins.run(ctx)?;
                            }
                        }
                    } else {
                        Err(InvalidValue("each"))?
                    }
                    None
                }
                Instruction::While(cond, body) => {
                    while cond.run(ctx)?.ok_or(InvalidValue("while"))?.to_bool() {
                        for ins in body {
                            ins.run(ctx)?;
                        }
                    }
                    None
                }
            }
        } else {
            None
        })
    }
}
