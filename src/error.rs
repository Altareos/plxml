use std::error::Error;
use std::fmt;

#[derive(Clone, Debug)]
pub struct MissingChild(pub &'static str, pub &'static str);

impl fmt::Display for MissingChild {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "missing '{}' child in '{}' node", self.1, self.0)
    }
}

impl Error for MissingChild {}

#[derive(Clone, Debug)]
pub struct MissingAttribute(pub &'static str, pub &'static str);

impl fmt::Display for MissingAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "missing '{}' child in '{}' node", self.1, self.0)
    }
}

impl Error for MissingAttribute {}

#[derive(Clone, Debug)]
pub struct BadArgumentCount(pub &'static str, pub usize);

impl fmt::Display for BadArgumentCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "bad argument count ({}) in call to '{}'", self.1, self.0)
    }
}

impl Error for BadArgumentCount {}

#[derive(Clone, Debug)]
pub struct BadChildCount(pub &'static str, pub usize);

impl fmt::Display for BadChildCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "bad child count ({}) in '{}' tag", self.1, self.0)
    }
}

impl Error for BadChildCount {}

#[derive(Clone, Debug)]
pub struct InvalidArgument(pub &'static str, pub &'static str);

impl fmt::Display for InvalidArgument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "invalid value for argument '{}' in call to '{}'",
            self.1, self.0
        )
    }
}

impl Error for InvalidArgument {}

#[derive(Clone, Debug)]
pub struct InvalidValue(pub &'static str);

impl fmt::Display for InvalidValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid value in '{}' tag", self.0)
    }
}

impl Error for InvalidValue {}

#[derive(Clone, Debug)]
pub struct InvalidProgram;

impl fmt::Display for InvalidProgram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid program structure")
    }
}

impl Error for InvalidProgram {}

#[derive(Clone, Debug)]
pub struct IncompatibleValues;

impl fmt::Display for IncompatibleValues {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "incompatible comparison values")
    }
}

impl Error for IncompatibleValues {}

#[derive(Clone, Debug)]
pub struct Unnamed(pub &'static str);

impl fmt::Display for Unnamed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unnamed '{}'", self.0)
    }
}

impl Error for Unnamed {}

#[derive(Clone, Debug)]
pub struct UnknownVariable(pub String);

impl fmt::Display for UnknownVariable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown variable '{}'", self.0)
    }
}

impl Error for UnknownVariable {}
