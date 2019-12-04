use std::error::Error;
use std::fmt;

pub type ESResult<T> = Result<T, ESError>;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ESErrorKind {
    CorruptedGraph,
    CorruptedRPNStack,
    CorruptedRule,
    UnknownFact,
    UnknownOp,
    LineError,
    IOError,
    RPNError,
    ExecError,
}

pub struct ESError {
    kind: ESErrorKind,
    what: Option<String>,
}

impl ESError {
    pub fn new(kind: ESErrorKind) -> ESError {
        ESError {
            kind: kind,
            what: None,
        }
    }

    pub fn new_w_what(kind: ESErrorKind, what: String) -> ESError {
        ESError {
            kind: kind,
            what: Some(what),
        }
    }

    pub fn unknown_fact(fact: char) -> Self {
        ESError {
            kind: ESErrorKind::UnknownFact,
            what: Some(format!("Fact '{}' is not recognized", fact)),
        }
    }

    pub fn corrupted_graph() -> Self {
        ESError {
            kind: ESErrorKind::CorruptedGraph,
            what: Some(String::from(
                "The graph seems to have its data corrupted.",
            )),
        }
    }

    pub fn corrupted_rule() -> Self {
        ESError {
            kind: ESErrorKind::CorruptedRule,
            what: Some(String::from("A rule owne an empty rpn stack.")),
        }
    }

    pub fn corrupted_rpn_stack() -> Self {
        ESError {
            kind: ESErrorKind::CorruptedRPNStack,
            what: Some(String::from(
                "A RPN stack is invalid and can't be solved.",
            )),
        }
    }

    pub fn failed_execution(what: String) -> Self {
        ESError {
            kind: ESErrorKind::ExecError,
            what: Some(what),
        }
    }

    pub fn kind(&self) -> ESErrorKind {
        self.kind
    }

    pub fn kind_str(&self) -> String {
        match self.kind {
            ESErrorKind::CorruptedGraph => String::from("Graph Error"),
            ESErrorKind::CorruptedRPNStack => String::from("RPN Error"),
            ESErrorKind::CorruptedRule => String::from("Rule Error"),
            ESErrorKind::UnknownOp => String::from("Unknown OP"),
            ESErrorKind::LineError => String::from("Line Error"),
            ESErrorKind::IOError => String::from("IO Error"),
            ESErrorKind::UnknownFact => String::from("Logic Error"),
            ESErrorKind::RPNError => String::from("RPN Error"),
            ESErrorKind::ExecError => String::from("Execution Error"),
        }
    }

    pub fn what(&self) -> String {
        match &self.what {
            None => String::from("unknown error").clone(),
            Some(s) => s.clone(),
        }
    }
}

impl fmt::Display for ESError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.kind_str(), self.what())
    }
}

impl fmt::Debug for ESError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "<{}:{}> {}: {}",
            file!(),
            line!(),
            self.kind_str(),
            self.what()
        )
    }
}
impl Error for ESError {}

impl From<std::io::Error> for ESError {
    fn from(error: std::io::Error) -> Self {
        ESError {
            kind: ESErrorKind::IOError,
            what: Some(String::from(error.description())),
        }
    }
}
