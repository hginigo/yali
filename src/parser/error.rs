use crate::tokenizer::Token;
use std::fmt;
use std::num;

#[derive(Debug)]
pub enum ParserErr {
    TokenNotFound(String),
    ParseInt(num::ParseIntError),
    UnexpectedToken((Token, Vec<String>)),
    UnclosedList,
}

impl fmt::Display for ParserErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn format_vec(v: &Vec<String>) -> String {
            let mut result = format!("`{}'", v[0]);
            for s in &v[1..] {
                result = format!("{}, `{}'", result, s);
            }
            result
        }
        match &self {
            Self::TokenNotFound(msg) => write!(f, "{}", msg),
            Self::ParseInt(e) => write!(f, "{}", e),
            Self::UnexpectedToken((t, v)) => {
                write!(
                    f,
                    "Expected any of {} but found `{}'",
                    format_vec(v),
                    t.value.as_str()
                )
            }
            Self::UnclosedList => write!(
                f,
                "Unexpected EOF parsing cons, ')' may be missing (unclosed list)"
            ),
        }
    }
}

#[macro_export]
macro_rules! token_not_found {
    ( $s:expr ) => {
        ParserErr::TokenNotFound(($s as &str).to_owned())
    };
}

#[macro_export]
macro_rules! unexpected_token {
    ( $t:expr, $( $s:expr ),* ) => {
        ParserErr::UnexpectedToken(($t, vec![$( $s.to_owned() ),*]))
    };
}

#[macro_export]
macro_rules! unclosed_list {
    () => {
        ParserErr::UnclosedList
    };
}

// impl Error for ParserErr {
//     fn source(&self) -> Option<&(dyn Error + 'static)> {
//         unimplemented!();
//     }
// }
