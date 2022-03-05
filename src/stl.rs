use super::{Context, Value};
use std::cell::RefCell;
use std::error::Error;
use std::io;
use std::rc::Rc;

pub fn inject_all(ctx: &mut Context) {
    ctx.assign(String::from("print"), Value::StdFunction(print));
    ctx.assign(String::from("input"), Value::StdFunction(input));
    ctx.assign(
        String::from("string-split"),
        Value::StdFunction(string_split),
    );
    ctx.assign(String::from("array-set"), Value::StdFunction(array_set));
    ctx.assign(String::from("array-push"), Value::StdFunction(array_push));
    ctx.assign(String::from("array-pop"), Value::StdFunction(array_pop));
    ctx.assign(String::from("array-get"), Value::StdFunction(array_get));
    ctx.assign(
        String::from("array-length"),
        Value::StdFunction(array_length),
    );
    ctx.assign(String::from("to-ascii"), Value::StdFunction(to_ascii));
    ctx.assign(String::from("from-ascii"), Value::StdFunction(from_ascii));
    ctx.assign(String::from("get-args"), Value::StdFunction(get_args));
}

fn print(vals: Vec<Value>) -> Result<Option<Value>, Box<dyn Error>> {
    if vals.len() == 1 {
        match &vals[0] {
            Value::Integer(i) => println!("{}", i),
            Value::Float(f) => println!("{}", f),
            Value::String(s) => println!("{}", s),
            v => println!("{:?}", v), // _ => Err("unprintable value")?,
        };
        Ok(Some(vals[0].clone()))
    } else {
        Err("bad argument count in call to 'print'")?
    }
}

fn input(vals: Vec<Value>) -> Result<Option<Value>, Box<dyn Error>> {
    if vals.len() == 0 {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        line.pop();
        Ok(Some(Value::String(line)))
    } else {
        Err("bad argument count in call to 'input'")?
    }
}

fn string_split(vals: Vec<Value>) -> Result<Option<Value>, Box<dyn Error>> {
    if vals.len() == 2 {
        if let Value::String(s) = &vals[0] {
            if let Value::String(d) = &vals[1] {
                let mut v = s
                    .split(d)
                    .map(|sub| Value::String(sub.to_string()))
                    .collect::<Vec<Value>>();
                v.remove(0);
                v.pop();
                Ok(Some(Value::Array(Rc::new(RefCell::new(v)))))
            } else {
                Err("invalid delimiter string in call to 'string-split'")?
            }
        } else {
            Err("invalid target string in call to 'string-split'")?
        }
    } else {
        Err("bad argument count in call to 'string-split'")?
    }
}

fn array_set(vals: Vec<Value>) -> Result<Option<Value>, Box<dyn Error>> {
    if vals.len() == 3 {
        if let Value::Array(v) = &vals[0] {
            if let Value::Integer(i) = &vals[1] {
                let index: usize = (*i).try_into()?;
                if v.borrow().len() > index {
                    v.borrow_mut()[index] = vals[2].clone();
                    Ok(None)
                } else {
                    Err("index out of array in call to 'array-set'")?
                }
            } else {
                Err("invalid index in call to 'array-set'")?
            }
        } else {
            Err("invalid array in call to 'array-set'")?
        }
    } else {
        Err("bad argument count in call to 'array-set'")?
    }
}

fn array_push(vals: Vec<Value>) -> Result<Option<Value>, Box<dyn Error>> {
    if vals.len() == 2 {
        if let Value::Array(v) = &vals[0] {
            v.borrow_mut().push(vals[1].clone());
            Ok(None)
        } else {
            Err("invalid array in call to 'array-push'")?
        }
    } else {
        Err("bad argument count in call to 'array-push'")?
    }
}

fn array_pop(vals: Vec<Value>) -> Result<Option<Value>, Box<dyn Error>> {
    if vals.len() == 1 {
        if let Value::Array(v) = &vals[0] {
            Ok(Some(
                v.borrow_mut()
                    .pop()
                    .ok_or("empty array in call to 'array-pop'")?,
            ))
        } else {
            Err("invalid array in call to 'array-pop'")?
        }
    } else {
        Err("bad argument count in call to 'array-pop'")?
    }
}

fn array_get(vals: Vec<Value>) -> Result<Option<Value>, Box<dyn Error>> {
    if vals.len() == 2 {
        if let Value::Array(v) = &vals[0] {
            if let Value::Integer(i) = &vals[1] {
                let index: usize = (*i).try_into()?;
                if v.borrow().len() > index {
                    Ok(Some(v.borrow_mut()[index].clone()))
                } else {
                    Err("index out of array in call to 'array-get'")?
                }
            } else {
                Err("invalid index in call to 'array-get'")?
            }
        } else {
            Err("invalid array in call to 'array-get'")?
        }
    } else {
        Err("bad argument count in call to 'array-get'")?
    }
}

fn array_length(vals: Vec<Value>) -> Result<Option<Value>, Box<dyn Error>> {
    if vals.len() == 1 {
        if let Value::Array(v) = &vals[0] {
            Ok(Some(Value::Integer(v.borrow().len() as i64)))
        } else {
            Err("invalid array in call to 'array-length'")?
        }
    } else {
        Err("bad argument count in call to 'array-length'")?
    }
}

fn to_ascii(vals: Vec<Value>) -> Result<Option<Value>, Box<dyn Error>> {
    if vals.len() == 1 {
        if let Value::Integer(i) = &vals[0] {
            if i <= &255 {
                Ok(Some(Value::String(String::from_utf8(vec![*i as u8])?)))
            } else {
                Err("invalid integer in call to 'to-ascii'")?
            }
        } else {
            Err("invalid argument in call to 'to-ascii'")?
        }
    } else {
        Err("bad argument count in call to 'to-ascii'")?
    }
}

fn from_ascii(vals: Vec<Value>) -> Result<Option<Value>, Box<dyn Error>> {
    if vals.len() == 1 {
        if let Value::String(s) = &vals[0] {
            if s.len() == 1 {
                Ok(Some(Value::Integer(s.as_bytes()[0] as i64)))
            } else {
                Err("invalid string in call to 'from-ascii'")?
            }
        } else {
            Err("invalid argument in call to 'from-ascii'")?
        }
    } else {
        Err("bad argument count in call to 'from-ascii'")?
    }
}

fn get_args(vals: Vec<Value>) -> Result<Option<Value>, Box<dyn Error>> {
    if vals.len() == 0 {
        Ok(Some(Value::Array(Rc::new(RefCell::new(
            std::env::args()
                .skip(1)
                .map(|arg| Value::String(arg))
                .collect(),
        )))))
    } else {
        Err("bad argument count in call to 'get-args'")?
    }
}
