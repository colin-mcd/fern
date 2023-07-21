#[derive(Debug)]
pub struct FernError {
    pub msg: String,
    pub lb: Pos,
    pub rb: Pos
}

impl std::fmt::Display for FernError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error from line {}, column {} to line {}, column {}: {}\n",
               self.lb.line, self.lb.col, self.rb.line, self.rb.col, self.msg)
    }
}

impl std::error::Error for FernError {}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pos {
    pub line: u64,
    pub col: u64
}

// Raw, unchecked term
#[derive(Debug, Clone)]
pub enum RawTerm {
    Lam {var: String, annot: Option<Box<RawType>>, body: Box<RawTerm>, lb: Pos, rb: Pos},
    App {head: Box<RawTerm>, args: Vec<Box<RawTerm>>, lb: Pos, rb: Pos},
    Var {var: String, lb: Pos, rb: Pos}
}

#[derive(Debug, Clone)]
pub enum RawType {
    Lam {var: String, annot: Option<Box<RawKind>>, body: Box<RawType>, lb: Pos, rb: Pos},
    All {var: String, annot: Option<Box<RawKind>>, body: Box<RawType>, lb: Pos, rb: Pos},
    App {head: Box<RawType>, args: Vec<Box<RawType>>, lb: Pos, rb: Pos},
    Arr {dom: Box<RawType>, cod: Box<RawType>, lb: Pos, rb: Pos},
    Var {var: String, lb: Pos, rb: Pos}
}

#[derive(Debug, Clone)]
pub enum RawKind {
    Star {lb: Pos, rb: Pos},
    Arr {dom: Box<RawKind>, cod: Box<RawKind>, lb: Pos, rb: Pos}
}

#[derive(Debug, Clone)]
pub enum RawDef {
    Tm {var: String, tp: Option<Box<RawType>>, tm: Box<RawTerm>, lb: Pos, rb: Pos},
    Tp {var: String, tp: Box<RawType>, lb: Pos, rb: Pos}
}

pub type AST = Vec<RawDef>;

pub struct TmVar {
    var: String,
    tp: *const Type,
}

pub struct TpVar {
    var: String,
    kd: *const Kind,
}

// Elaborated, information-rich term
pub enum Term {
    Lam {var: TmVar, body: *const Term, rettp: *const Type},
    App {head: *const Term, args: Vec<*const Term>, tp: *const Type},
    Var {var: TmVar}
}

pub enum Type {
    Arr {dom: *const Type, cod: *const Type},
    All {var: TpVar, body: *const Type},
    Var {var: TpVar}
}

pub enum Kind {
    Star,
    Arr {dom: *const Kind, cod: *const Kind},
}


pub trait Span {
    fn bounds(&self) -> (Pos, Pos);
    fn left(&self) -> Pos {
        self.bounds().0
    }
    fn right(&self) -> Pos {
        self.bounds().1
    }
}

impl Span for RawTerm {
    fn bounds(&self) -> (Pos, Pos) {
        match *self {
            RawTerm::Lam {var: _, annot: _, body: _, lb, rb} => (lb, rb),
            RawTerm::App {head: _, args: _, lb, rb} => (lb, rb),
            RawTerm::Var {var: _, lb, rb} => (lb, rb)
        }
    }
}

impl Span for RawType {
    fn bounds(&self) -> (Pos, Pos) {
        match *self {
            RawType::Lam {var: _, annot: _, body: _, lb, rb} => (lb, rb),
            RawType::All {var: _, annot: _, body: _, lb, rb} => (lb, rb),
            RawType::App {head: _, args: _, lb, rb} => (lb, rb),
            RawType::Arr {dom: _, cod: _, lb, rb} => (lb, rb),
            RawType::Var {var: _, lb, rb} => (lb, rb)
        }
    }
}

impl Span for RawKind {
    fn bounds(&self) -> (Pos, Pos) {
        match *self {
            RawKind::Star {lb, rb} => (lb, rb),
            RawKind::Arr {dom: _, cod: _, lb, rb} => (lb, rb)
        }
    }
}
