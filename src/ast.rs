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
pub enum RawTm {
    Lam {var: String, annot: Option<RawType>, body: RawTerm, lb: Pos, rb: Pos},
    App {head: RawTerm, args: Vec<RawTerm>, lb: Pos, rb: Pos},
    Var {var: String, lb: Pos, rb: Pos}
}

#[derive(Debug, Clone)]
pub enum RawTp {
    Lam {var: String, annot: Option<RawKind>, body: RawType, lb: Pos, rb: Pos},
    All {var: String, annot: Option<RawKind>, body: RawType, lb: Pos, rb: Pos},
    App {head: RawType, args: Vec<RawType>, lb: Pos, rb: Pos},
    Arr {dom: RawType, cod: RawType, lb: Pos, rb: Pos},
    Var {var: String, lb: Pos, rb: Pos}
}

#[derive(Debug, Clone)]
pub enum RawKd {
    Star {lb: Pos, rb: Pos},
    Arr {dom: RawKind, cod: RawKind, lb: Pos, rb: Pos}
}

pub type RawTerm = Box<RawTm>;
pub type RawType = Box<RawTp>;
pub type RawKind = Box<RawKd>;

#[derive(Debug, Clone)]
pub enum RawDef {
    Tm {var: String, annot: Option<RawType>, body: RawTerm, lb: Pos, rb: Pos},
    Tp {var: String, annot: Option<RawKind>, body: RawType, lb: Pos, rb: Pos}
}

pub type AST = Vec<RawDef>;

pub struct TmVar {
    var: String,
    tp: Box<Type>,
}

pub struct TpVar {
    var: String,
    kd: Box<Kind>,
}

// Elaborated, information-rich term
pub enum Tm {
    Lam {var: TmVar, body: Term, rettp: Type},
    App {head: Term, args: Vec<Term>, tp: Type},
    Var {var: TmVar}
}

pub enum Tp {
    Arr {dom: Type, cod: Type},
    All {var: TpVar, body: Type},
    Var {var: TpVar}
}

pub enum Kd {
    Star,
    Arr {dom: Kind, cod: Kind},
}

pub type Term = Box<Tm>;
pub type Type = Box<Tp>;
pub type Kind = Box<Kd>;


pub trait Span {
    fn bounds(&self) -> (Pos, Pos);
    fn left(&self) -> Pos {
        self.bounds().0
    }
    fn right(&self) -> Pos {
        self.bounds().1
    }
}

impl Span for RawTm {
    fn bounds(&self) -> (Pos, Pos) {
        match *self {
            RawTm::Lam {var: _, annot: _, body: _, lb, rb} => (lb, rb),
            RawTm::App {head: _, args: _, lb, rb} => (lb, rb),
            RawTm::Var {var: _, lb, rb} => (lb, rb)
        }
    }
}

impl Span for RawTp {
    fn bounds(&self) -> (Pos, Pos) {
        match *self {
            RawTp::Lam {var: _, annot: _, body: _, lb, rb} => (lb, rb),
            RawTp::All {var: _, annot: _, body: _, lb, rb} => (lb, rb),
            RawTp::App {head: _, args: _, lb, rb} => (lb, rb),
            RawTp::Arr {dom: _, cod: _, lb, rb} => (lb, rb),
            RawTp::Var {var: _, lb, rb} => (lb, rb)
        }
    }
}

impl Span for RawKd {
    fn bounds(&self) -> (Pos, Pos) {
        match *self {
            RawKd::Star {lb, rb} => (lb, rb),
            RawKd::Arr {dom: _, cod: _, lb, rb} => (lb, rb)
        }
    }
}

impl Span for RawDef {
    fn bounds(&self) -> (Pos, Pos) {
        match *self {
            RawDef::Tm {var: _, annot: _, body: _, lb, rb} => (lb, rb),
            RawDef::Tp {var: _, annot: _, body: _, lb, rb} => (lb, rb)
        }
    }
}
