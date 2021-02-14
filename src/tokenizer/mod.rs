// use std::string::String;

#[derive(Debug)]
pub struct TokenRange {
    pub start:  usize,
    pub end:    usize,
}

#[derive(Debug)]
pub struct Token {
    pub t: &'static str,
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
            start: pos, 
            end: pos+1
        })},

        '"' => {
            let sl = &s[pos+1..];
            let end = skip_to_char(sl, '"');
            // println!("{} end: {:?}", sl.len(), end);
            Some(TokenRange {
                start: pos,
                end: if end.is_some() { end.unwrap() + 1 }
                    else { sl.len() } + pos + 1,
            })
        },

        _ => {
            Some(TokenRange {
                start: pos,
                end: pos + next_valid_symbol(&s[pos+1..]) + 1,
        })},
    }
}

pub fn tokenize(s: &'static str) -> Vec<Token> {
    let mut parts = &s[..];
    let mut next = next_token(parts);
    let mut res = Vec::new();

    while next.is_some() {
        let tr = next.unwrap();
        let t = Token {
            t: &parts[tr.start..tr.end],
        };

        res.push(t);
        parts = &parts[tr.end..];
        next = next_token(parts);
    }
    res
}

