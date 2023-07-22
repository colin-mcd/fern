use crate::lex::{Token, TokTp};
use crate::ast;
use crate::ast::*;
use std::collections::VecDeque;

pub fn parse(toks: &mut Vec<Token>) -> Result<AST, FernError> {
    let toksdeq = &mut VecDeque::from(toks.clone());
    let defs = parse_defs(toksdeq)?;
    parse_drop(toksdeq, TokTp::EOF, Pos {line: 1, col: 0})?;
    Ok(defs)
}

fn parse_drop(toks: &mut VecDeque<Token>, expected: TokTp, lb: Pos) -> Result<(), FernError> {
    let Token{tok, pos} = toks.pop_front().unwrap();
    if tok == expected {
        Ok(())
    } else {
        Err(FernError {msg: format!("expected {expected} here but got {tok}"), lb, rb: pos})
    }
}

fn parse_var(toks: &mut VecDeque<Token>) -> Result<String, FernError> {
    let Token{tok, pos} = toks.pop_front().unwrap();
    match tok {
        TokTp::Var(s) => Ok(s),
        _ => Err(FernError {msg: String::from("expected an identifier here"), lb: pos, rb: pos})
    }
}


/*
DEF ::=
  | def VAR [: TYPE] = TERM
  | type VAR [: KIND] = TYPE

DEFS ::=
  | [empty]
  | DEF DEFS

*/

fn parse_def(toks: &mut VecDeque<Token>) -> Result<RawDef, FernError> {
    let Token{tok, pos: lb} = toks.pop_front().unwrap();
    match tok {
        TokTp::Def => {
            let var = parse_var(toks)?;
            let annot = parse_type_annot(toks)?;
            parse_drop(toks, TokTp::Eq, lb)?;
            let body = parse_term(toks)?;
            let rb = body.clone().right();
            Ok(RawDef::Tm {var, annot, body, lb, rb})
        },
        TokTp::Type => {
            let var = parse_var(toks)?;
            let annot = parse_kind_annot(toks)?;
            parse_drop(toks, TokTp::Eq, lb)?;
            let body = parse_type(toks)?;
            let rb = body.clone().right();
            Ok(RawDef::Tp {var, annot, body, lb, rb})
        },
        _ => Err(FernError {
            msg: format!("unexpected token {tok} when parsing a definition"),
            lb: lb, rb: lb
        })
    }
}

fn parse_defs(toks: &mut VecDeque<Token>) -> Result<AST, FernError> {
    let mut defs = Vec::new();
    while toks.front().unwrap().tok != TokTp::EOF {
        let def = parse_def(toks)?;
        defs.push(def);
    }
    Ok(defs)
}


/*
TERM1 ::=
  | λ VAR [: TYPE]. TERM1
  | TERM2

TERM2 ::=
  | TERM2 TERM3
  | TERM3

TERM3 ::=
  | VAR
  | (TERM1)

//  | let VAR [: TYPE] = TERM in TERM
//  | let VAR [: KIND] = TYPE in TERM
//  | match TERM {CASES}
//  | Λ VAR [: KIND]. TERM
*/

fn parse_term(toks: &mut VecDeque<Token>) -> Result<RawTerm, FernError> {
    parse_term1(toks)
}

fn parse_term1(toks: &mut VecDeque<Token>) -> Result<RawTerm, FernError> {
    // λ VAR [: TYPE]. TERM1
    // TERM2
    let Token{tok, pos} = toks.front().unwrap().clone();
    match tok {
        TokTp::Lam => {
            toks.pop_front();
            let var = parse_var(toks)?;
            let annot = parse_type_annot(toks)?;
            parse_drop(toks, TokTp::Dot, pos)?;
            let body = parse_term1(toks)?;
            Ok(Box::new(RawTm::Lam {var, annot, body: body.clone(), lb: pos, rb: body.right()}))
        }
        _ => parse_term2(toks)
    }
}

fn parse_term2(toks: &mut VecDeque<Token>) -> Result<RawTerm, FernError> {
    // TERM2 TERM3
    // TERM3
    let head = parse_term3(toks)?;
    let mut args: Vec<RawTerm> = Vec::new();
    // Keep trying to parse additional args until you hit an error
    loop {
        // TODO: this seems pretty inefficient, O(n²)
        // Does this become exponential with nested applications?
        let old_toks = (*toks).clone();
        match parse_term3(toks) {
            Err(_) => {
                *toks = old_toks;
                break
            },
            Ok(arg) => args.push(arg)
        }
    }
    if args.is_empty() {
        Ok(head)
    } else {
        let last: RawTerm = args.last().unwrap().clone();
        Ok(Box::new(RawTm::App {head: head.clone(), args, lb: head.left(), rb: last.right()}))
    }
}

fn parse_term3(toks: &mut VecDeque<Token>) -> Result<RawTerm, FernError> {
    // VAR
    // (TERM1)
    let Token{tok, pos} = toks.pop_front().unwrap();
    match tok {
        TokTp::Var(s) => {
            Ok(Box::new(RawTm::Var {var: s, lb: pos, rb: pos}))
        },
        TokTp::ParenL => {
            let tm = parse_term1(toks)?;
            parse_drop(toks, TokTp::ParenR, pos)?;
            Ok(tm)
        }
        _ => Err(FernError {msg: format!("unexpected token {tok} when parsing a term"), lb: pos, rb: pos})
    }
}

/*
TYPE1 ::=
  | λ VAR [: KIND]. TYPE1
  | TYPE2

TYPE2 ::=
  | forall VAR [: KIND]. TYPE2
  | TYPE3

TYPE3 ::=
  | TYPE4 -> TYPE2
  | TYPE4

TYPE4 ::=
  | TYPE4 TYPE5
  | TYPE5

TYPE5 ::=
  | VAR
  | (TYPE1)
*/

fn parse_type_annot(toks: &mut VecDeque<Token>) -> Result<Option<RawType>, FernError> {
    let Token{tok, pos} = toks.front().unwrap().clone();
    match tok {
        TokTp::Colon => {
            toks.pop_front();
            parse_type(toks).map(Some)
        },
        _ => Ok(None)
    }
}

fn parse_kind_annot(toks: &mut VecDeque<Token>) -> Result<Option<RawKind>, FernError> {
    let Token{tok, pos} = toks.front().unwrap().clone();
    match tok {
        TokTp::Colon => {
            toks.pop_front();
            parse_kind(toks).map(Some)
        },
        _ => Ok(None)
    }
}

fn parse_type(toks: &mut VecDeque<Token>) -> Result<RawType, FernError> {
    parse_type1(toks)
}

fn parse_type1(toks: &mut VecDeque<Token>) -> Result<RawType, FernError> {
    // λ VAR [: KIND]. TYPE1
    // TYPE2
    let Token{tok, pos} = toks.front().unwrap().clone();
    match tok {
        TokTp::Lam => {
            toks.pop_front();
            let var = parse_var(toks)?;
            let annot = parse_kind_annot(toks)?;
            parse_drop(toks, TokTp::Dot, pos)?;
            let body = parse_type1(toks)?;
            Ok(Box::new(RawTp::Lam {var, annot, body: body.clone(), lb: pos, rb: body.right()}))
        }
        _ => parse_type2(toks)
    }
}

fn parse_type2(toks: &mut VecDeque<Token>) -> Result<RawType, FernError> {
    // forall VAR [: KIND]. TYPE2
    // TYPE3
    let Token{tok, pos} = toks.front().unwrap().clone();
    match tok {
        TokTp::All => {
            toks.pop_front();
            let var = parse_var(toks)?;
            let annot = parse_kind_annot(toks)?;
            parse_drop(toks, TokTp::Dot, pos)?;
            let body = parse_type2(toks)?;
            Ok(Box::new(RawTp::All {var, annot, body: body.clone(), lb: pos, rb: body.right()}))
        },
        _ => parse_type3(toks)
    }
}

fn parse_type3(toks: &mut VecDeque<Token>) -> Result<RawType, FernError> {
    // TYPE4 -> TYPE2
    // TYPE4
    let ltp = parse_type4(toks)?;
    let Token{tok, pos} = toks.front().unwrap().clone();
    match tok {
        TokTp::Arr => {
            toks.pop_front();
            let rtp = parse_type2(toks)?;
            Ok(Box::new(RawTp::Arr {dom: ltp, cod: rtp.clone(), lb: pos, rb: rtp.right()}))
        },
        _ => Ok(ltp)
    }
}

fn parse_type4(toks: &mut VecDeque<Token>) -> Result<RawType, FernError> {
    // TYPE4 TYPE5
    // TYPE5
    let head = parse_type5(toks)?;
    let mut args: Vec<RawType> = Vec::new();
    // Keep trying to parse additional args until you hit an error
    loop {
        // TODO: this seems pretty inefficient, O(n²)
        // Does this become exponential with nested applications?
        let old_toks = (*toks).clone();
        match parse_type5(toks) {
            Err(_) => {
                *toks = old_toks;
                break
            },
            Ok(arg) => args.push(arg)
        }
    }
    if args.is_empty() {
        Ok(head)
    } else {
        let last: RawType = args.last().unwrap().clone();
        Ok(Box::new(RawTp::App {head: head.clone(), args, lb: head.left(), rb: last.right()}))
    }
}

fn parse_type5(toks: &mut VecDeque<Token>) -> Result<RawType, FernError> {
    // VAR
    // (TYPE1)
    let Token{tok, pos} = toks.pop_front().unwrap();
    match tok {
        TokTp::Var(s) => {
            Ok(Box::new(RawTp::Var {var: s, lb: pos, rb: pos}))
        },
        TokTp::ParenL => {
            let tp = parse_type1(toks)?;
            parse_drop(toks, TokTp::ParenR, pos)?;
            Ok(tp)
        }
        _ => Err(FernError {msg: format!("unexpected token {tok} when parsing a type"), lb: pos, rb: pos})
    }
}

/*
KIND1 ::=
  | KIND2 -> KIND1
  | KIND2

KIND2 ::=
  | *
  | (KIND1)
*/
fn parse_kind(toks: &mut VecDeque<Token>) -> Result<RawKind, FernError> {
    parse_kind1(toks)
}


fn parse_kind1(toks: &mut VecDeque<Token>) -> Result<RawKind, FernError> {
    let k = parse_kind2(toks)?;
    match toks.front().unwrap() {
        Token {tok: TokTp::Arr, pos: _} => {
            toks.pop_front();
            let k2 = parse_kind1(toks)?;
            Ok(Box::new(RawKd::Arr {dom: k.clone(), cod: k2.clone(), lb: k.left(), rb: k2.right()}))
        },
        _ => Ok(k)
    }
}


fn parse_kind2(toks: &mut VecDeque<Token>) -> Result<RawKind, FernError> {
    match toks.pop_front().unwrap() {
        Token {tok: TokTp::Star, pos} => {
            Ok(Box::new(RawKd::Star {lb: pos, rb: pos + Pos {line: 0, col: 1}}))
        },
        Token {tok: TokTp::ParenL, pos} => {
            let k = parse_kind1(toks)?;
            parse_drop(toks, TokTp::ParenR, pos)?;
            Ok(k)
        },
        Token {tok, pos} => Err(FernError {
            msg: format!("unexpected {} when parsing a kind", tok),
            lb: pos, rb: pos
        })
    }
}
