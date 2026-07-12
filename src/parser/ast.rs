

#[derive(Debug)]
pub enum Statement {
    Block(Vec<Statement>),
    VarDeclaration {
        identifier: String,
        initializer: Box<Expression>
    },
}

#[derive(Debug, PartialEq)]
pub enum BinaryOp {
    Equality,
    StrictEquality,
    Inequality, 
    StrictInequality,
    And, 
    Or,
    Add,
    Sub,
    Div,
    Mul,
}

#[derive(Debug)]
pub struct BinaryExpression { 
    left: Box<Expression>,
    right: Box<Expression>,
    op: BinaryOp,
}

#[derive(Debug)]
pub enum Expression {
   Binary(BinaryExpression),
}
