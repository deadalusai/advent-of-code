
pub type Signal = u16;
pub type Label = String;

#[derive(Debug, Clone)]
pub enum Source {
    Wire(Label),
    Const(Signal)
}

#[derive(Debug, Clone)]
pub enum Gate1 {
    NOT
}

#[derive(Debug, Clone)]
pub enum Gate2 {
    AND, OR, LSHIFT, RSHIFT
}

#[derive(Debug, Clone)]
pub enum Expr {
    Input(Source),
    Gate1(Gate1, Source),
    Gate2(Gate2, Source, Source)
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub expr: Expr,
    pub target: Label
}