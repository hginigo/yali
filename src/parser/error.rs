use crate::tokenizer::Token;
use std::num;
use std::fmt;

#[derive(Debug)]
pub enum ParserErr {
    TokenNotFound(String),
    ParseInt(num::ParseIntError),
    UnexpectedToken((String, Token)),
}

impl fmt::Display for ParserErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::TokenNotFound(msg) =>
                write!(f, "{}", msg),
            Self::ParseInt(e) =>
                write!(f,"{}", e),
            Self::UnexpectedToken((s, t)) =>
                write!(f, "Expected `{}` but found `{}`", s, t.value.as_str())
        }
    }
}


#[macro_export]
macro_rules! token_not_found {
    ( $s:expr ) => { 
        ParserErr::TokenNotFound(($s as &str).to_owned())
    }
}

#[macro_export]
macro_rules! unexpected_token {
    ( $s:expr, $t:expr ) => { 
        ParserErr::UnexpectedToken((($s as &str).to_owned(), $t))
    }
}

// impl Error for ParserErr {
//     fn source(&self) -> Option<&(dyn Error + 'static)> {
//         unimplemented!();
//     }
// }


