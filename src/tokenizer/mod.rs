// use std::string::String;

#[derive(Debug)]
pub struct TokenRange {
    start:  usize,
    end:    usize,
}

#[derive(Debug)]
pub struct Token<'a> {
    pub value:  &'a str,
    pub range:  TokenRange,
    pub ttype:  TokenType,
}

#[derive(Debug)]
pub enum TokenType {
    Opc,
    Clc,
    Quo,
    Quasi,
    Unquo,
    Assoc,
    Str,
    Other,
}

fn skip_whitespace(s: &str) -> usize {
    match s.chars().position(|c| !c.is_whitespace()) {
        None => s.len(),
        Some(pos) => pos,
    }
}

fn skip_to_char(s: &str, c: char) -> Option<usize> {
    // println!("{}", s);
    s.chars().position(|x| x == c)
}

fn next_valid_symbol(s: &str) -> usize {
    for (i, c) in s.chars().enumerate() {
        if c.is_whitespace() {
            return i;
        }
        match c {
            '(' | ')' | '"' | '\''|
            ';' | ',' => return i,
            _ => {
                continue
            },
        }
    }
    s.len()
}

fn next_token(s: &str) -> Option<TokenRange> {
    let mut pos = skip_whitespace(&s);
    let sc = s.as_bytes();

    if s[pos..].len() == 0 {
        return None;
    }

    while (sc[pos] as char) == ';' {
        pos += match skip_to_char(&s[pos+1..], '\n') {
            Some(n) => n,
            None => s[pos+1..].len(),
        };
        pos += skip_whitespace(&s[pos+1..]);
    }

    match sc[pos] as char {
        '(' |
        ')' |
        '.' |
        '\''|
        ',' => {
            Some(TokenRange { 
                // ttype: TokenType::Symbol,
                start: pos, 
                end: pos+1,
            })},

        '"' => {
            let sl = &s[pos+1..];
            let end = skip_to_char(sl, '"');
            // println!("{} end: {:?}", sl.len(), end);
            Some(TokenRange {
                // ttype: TokenType::Str,
                start: pos,
                end: if end.is_some() { end.unwrap() + 1 }
                    else { sl.len() } + pos + 1,
            })
        },

        _ => {
            Some(TokenRange {
                // ttype: TokenType::Name,
                start: pos,
                end: pos + next_valid_symbol(&s[pos+1..]) + 1,
        })},
    }
}

pub fn tokenize<'a>(s: &'a str) -> Vec<Token> {
    let mut res = Vec::new();
    let mut offs = 0;

    loop {
        let tok_ran = match next_token(&s[offs..]) {
            // Return the given range with current offset
            Some(tr) => TokenRange {
                start:  tr.start + offs,
                end:    tr.end + offs,
            },
            None => break, // All tokens have been processed, end here
        };

        let val = &s[tok_ran.start..tok_ran.end];
        offs = tok_ran.end;

        let c = match val.chars().next() {
            Some(c) => c,
            None => continue, // Empty token, just ignore
        };

        let ttype = match c {
            '(' => TokenType::Opc,
            ')' => TokenType::Clc,
            '\''=> TokenType::Quo,
            '`' => TokenType::Quasi,
            ',' => TokenType::Unquo,
            '.' => TokenType::Assoc,
            '"' => TokenType::Str,
            _   => TokenType::Other,
        };
        let t = Token {
            value:  &val,
            range:  tok_ran,
            ttype:  ttype,
        };
        res.push(t);
    }

    res.reverse();
    res
}

