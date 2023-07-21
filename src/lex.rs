//use std::io;
//use std::cmp::Ordering;
use crate::ast;
use crate::ast::{Pos, FernError};
use std::ops::Add;
use std::vec::Vec;
use std::error::Error;
use std::fmt;

//#[derive(Debug, Copy, Clone, PartialEq)]
#[derive(Debug, Clone, PartialEq)]
pub enum TokTp {
    Var(String),
    Lam,
    ParenL,
    ParenR,
    Eq,
    Dot,
    Let,
    In,
    Colon,
    Arr,
    All,
    Star,
    Data,
    Match,
    MatchArr,
    Bar,
    Syntax,
    EOF
}

impl fmt::Display for TokTp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from(match self {
            TokTp::Var(s)   => s.as_str(),
            TokTp::Lam      => "λ",
            TokTp::ParenL   => "(",
            TokTp::ParenR   => ")",
            TokTp::Eq       => "=",
            TokTp::Dot      => ".",
            TokTp::Let      => "let",
            TokTp::In       => "in",
            TokTp::Colon    => ":",
            TokTp::Arr      => "->",
            TokTp::All      => "forall",
            TokTp::Star     => "*",
            TokTp::Data     => "data",
            TokTp::Match    => "match",
            TokTp::MatchArr => "=>",
            TokTp::Bar      => "|",
            TokTp::Syntax   => "syntax",
            TokTp::EOF      => "EOF"
        }))
    }
}

//#[derive(Debug, Copy, Clone, PartialEq)]
#[derive(Debug, Clone)]
pub struct Token {
    pub tok: TokTp,
    pub pos: Pos,
}

impl Add for Pos {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {line: self.line + other.line, col: self.col + other.col}
    }
}

pub fn lex(body: String) -> Result<Vec<Token>, FernError> {
    let mut toks = Vec::new();
    lex_h(body.chars().collect(), 0, Pos{line:1, col:0}, &mut toks)?;
    Ok(toks)
}

fn is_punctuation(c: char) -> Option<TokTp> {
    match c {
        'λ' => Some(TokTp::Lam),
        '\\' => Some(TokTp::Lam),
        '(' => Some(TokTp::ParenL),
        ')' => Some(TokTp::ParenR),
        '.' => Some(TokTp::Dot),
        ':' => Some(TokTp::Colon),
        _ => None
    }
}

fn is_whitespace(c: char, pos: Pos) -> Option<Pos> {
    if c == '\n' {
        Some(Pos{line: pos.line + 1, col: 0})
    } else if c.is_whitespace() {
        Some(pos + Pos{line: 0, col: 1})
    } else {
        None
    }
}

fn lex_h(body: Vec<char>, idx: usize, pos: Pos, toks: &mut Vec<Token>) -> Result<(), FernError> {
    match body.get(idx) {
        None => {
            toks.push(Token {tok: TokTp::EOF, pos: pos});
            Ok(())
        },
        Some(mut c) => {
            if let Some(newpos) = is_whitespace(*c, pos) {
                /*return*/ lex_h(body, idx + 1, newpos, toks)//;
            } else if let Some(tok) = is_punctuation(*c) {
                toks.push(Token {tok, pos});
                /*return*/ lex_h(body, idx + 1, pos + Pos{line: 0, col:1}, toks)//;
            /*} else if *c == '"' {
                let mut esc = False;
                let mut escaped_str = String::new();
                let mut sz: usize = 0;
                while *c != '"' || esc {
                    let mut c2 = *c;
                    if esc {
                        c2 = match *c {
                            '\\' => '\\',
                            't' => '\t',
                            'n' => '\n',
                            'r' => '\r',
                            '"' => '\"',
                            '\'' => '\'',
                            '0' => '\0',
                            _ => return FernError {
                                msg: format!("invalid escape sequence '\\{}'", *c),
                                lb: pos + Pos{line: 0, col: sz},
                                rb: pos + Pos{line: 0, col: sz + 1}
                            };
                        }
                    } else {
                        esc = c == '\\';
                    }
                    esc = (c2 == '\\') && !esc;
                    escaped_str.push(c2);
                    sz += 1;
                }*/
            } else {
                let mut var = String::new();
                let mut sz: usize = 0;
                while is_punctuation(*c).is_none() && is_whitespace(*c, pos).is_none() {
                    var.push(*c);
                    sz += 1;
                    
                    match body.get(idx + sz) {
                        None => break,
                        Some(c2) => c = c2,
                    }
                }
                let tok = match var.as_str() {
                    "syntax" => TokTp::Syntax,
                    "data" => TokTp::Data,
                    "forall" => TokTp::All,
                    "match" => TokTp::Match,
                    "=>" => TokTp::MatchArr,
                    "=" => TokTp::Eq,
                    "|" => TokTp::Bar,
                    "*" => TokTp::Star,
                    "->" => TokTp::Arr,
                    _ => TokTp::Var(var)
                };
                toks.push(Token {tok, pos});
                /*return*/ lex_h(body, idx + sz, pos + Pos {line: 0, col: sz as u64}, toks)//;
            }
        }
    }
}
