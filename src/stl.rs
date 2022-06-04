use crate::error::InaccessibleFile;

use super::error::{BadArgumentCount, InvalidArgument};
use super::{Context, Value};
use std::cell::RefCell;
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::{stdin, stdout, Write};
use std::rc::Rc;

pub fn inject_all(ctx: &mut Context) {
    ctx.assign(String::from("print"), Value::StdFunction(print));
    ctx.assign(String::from("print-line"), Value::StdFunction(print_line));
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
    ctx.assign(String::from("write-file"), Value::StdFunction(write_file));
    ctx.assign(String::from("read-file"), Value::StdFunction(read_file));
}

fn print(vals: Vec<Value>) -> Result<Option<Value>, Box<dyn Error>> {
    if vals.len() == 1 {
        match &vals[0] {
            Value::Integer(i) => print!("{}", i),
            Value::Real(f) => print!("{}", f),
            Value::String(s) => print!("{}", s),
            v => print!("{:?}", v), // _ => Err("unprintable value")?,
        };
        let _ = stdout().flush();
        Ok(Some(vals[0].clone()))
    } else {
        Err(BadArgumentCount("print", vals.len(), 1).into())
    }
}

fn print_line(vals: Vec<Value>) -> Result<Option<Value>, Box<dyn Error>> {
    if vals.len() == 1 {
        match &vals[0] {
            Value::Integer(i) => println!("{}", i),
            Value::Real(f) => println!("{}", f),
            Value::String(s) => println!("{}", s),
            v => println!("{:?}", v), // _ => Err("unprintable value")?,
        };
        let _ = stdout().flush();
        Ok(Some(vals[0].clone()))
    } else {
        Err(BadArgumentCount("print-line", vals.len(), 1).into())
    }
}

fn input(vals: Vec<Value>) -> Result<Option<Value>, Box<dyn Error>> {
    if vals.len() == 0 {
        let mut line = String::new();
        stdin().read_line(&mut line)?;
        line.pop();
        Ok(Some(Value::String(line)))
    } else {
        Err(BadArgumentCount("input", vals.len(), 0).into())
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
                Err(InvalidArgument("string-split", "delimiter").into())
            }
        } else {
            Err(InvalidArgument("string-split", "target").into())
        }
    } else {
        Err(BadArgumentCount("string-split", vals.len(), 2).into())
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
                    Err(InvalidArgument("array-set", "index").into())
                }
            } else {
                Err(InvalidArgument("array-set", "index").into())
            }
        } else {
            Err(InvalidArgument("array-set", "array").into())
        }
    } else {
        Err(BadArgumentCount("array-set", vals.len(), 3).into())
    }
}

fn array_push(vals: Vec<Value>) -> Result<Option<Value>, Box<dyn Error>> {
    if vals.len() == 2 {
        if let Value::Array(v) = &vals[0] {
            v.borrow_mut().push(vals[1].clone());
            Ok(None)
        } else {
            Err(InvalidArgument("array-push", "array").into())
        }
    } else {
        Err(BadArgumentCount("array-push", vals.len(), 2).into())
    }
}

fn array_pop(vals: Vec<Value>) -> Result<Option<Value>, Box<dyn Error>> {
    if vals.len() == 1 {
        if let Value::Array(v) = &vals[0] {
            Ok(Some(
                v.borrow_mut()
                    .pop()
                    .ok_or(InvalidArgument("array-pop", "array"))?,
            ))
        } else {
            Err(InvalidArgument("array-pop", "array").into())
        }
    } else {
        Err(BadArgumentCount("array-pop", vals.len(), 1).into())
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
                    Err(InvalidArgument("array-get", "index").into())
                }
            } else {
                Err(InvalidArgument("array-get", "index").into())
            }
        } else {
            Err(InvalidArgument("array-set", "array").into())
        }
    } else {
        Err(BadArgumentCount("array-get", vals.len(), 2).into())
    }
}

fn array_length(vals: Vec<Value>) -> Result<Option<Value>, Box<dyn Error>> {
    if vals.len() == 1 {
        if let Value::Array(v) = &vals[0] {
            Ok(Some(Value::Integer(v.borrow().len() as i64)))
        } else {
            Err(InvalidArgument("array-length", "array").into())
        }
    } else {
        Err(BadArgumentCount("array-length", vals.len(), 1).into())
    }
}

fn to_ascii(vals: Vec<Value>) -> Result<Option<Value>, Box<dyn Error>> {
    if vals.len() == 1 {
        if let Value::Integer(i) = &vals[0] {
            if &0 <= i && i <= &255 {
                Ok(Some(Value::String(String::from_utf8(vec![*i as u8])?)))
            } else {
                Err(InvalidArgument("to-ascii", "integer").into())
            }
        } else {
            Err(InvalidArgument("to-ascii", "integer").into())
        }
    } else {
        Err(BadArgumentCount("to-ascii", vals.len(), 1).into())
    }
}

fn from_ascii(vals: Vec<Value>) -> Result<Option<Value>, Box<dyn Error>> {
    if vals.len() == 1 {
        if let Value::String(s) = &vals[0] {
            if s.len() == 1 {
                Ok(Some(Value::Integer(s.as_bytes()[0] as i64)))
            } else {
                Err(InvalidArgument("from-ascii", "string").into())
            }
        } else {
            Err(InvalidArgument("from-ascii", "string").into())
        }
    } else {
        Err(BadArgumentCount("from-ascii", vals.len(), 1).into())
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
        Err(BadArgumentCount("get-args", vals.len(), 0).into())
    }
}

fn write_file(vals: Vec<Value>) -> Result<Option<Value>, Box<dyn Error>> {
    if vals.len() == 3 {
        if let Value::String(path) = &vals[0] {
            if let Value::String(contents) = &vals[1] {
                if let Ok(mut file) = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .append(Value::to_bool(&vals[2]))
                    .open(path)
                {
                    if let Ok(_) = write!(file, "{}", contents) {
                        Ok(None)
                    } else {
                        Err(InaccessibleFile(path.clone()).into())
                    }
                } else {
                    Err(InaccessibleFile(path.clone()).into())
                }
            } else {
                Err(InvalidArgument("write-file", "string").into())
            }
        } else {
            Err(InvalidArgument("write-file", "string").into())
        }
    } else {
        Err(BadArgumentCount("write-file", vals.len(), 3).into())
    }
}

fn read_file(vals: Vec<Value>) -> Result<Option<Value>, Box<dyn Error>> {
    if vals.len() == 1 {
        if let Value::String(path) = &vals[0] {
            if let Ok(contents) = fs::read_to_string(path) {
                Ok(Some(Value::String(contents)))
            } else {
                Err(InaccessibleFile(path.clone()).into())
            }
        } else {
            Err(InvalidArgument("read-file", "string").into())
        }
    } else {
        Err(BadArgumentCount("read-file", vals.len(), 1).into())
    }
}
