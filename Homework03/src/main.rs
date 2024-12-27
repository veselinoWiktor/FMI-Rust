#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    Atom(char),
    Not(Box<Expr>),
    And(Vec<Expr>),
    Or(Vec<Expr>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    UnexpectedExpr,
    UnexpectedUnaryOp,
    UnexpectedBinOp,
    UnexpectedParen,
    UnexpectedEnd,
}

/// Парсър за прост израз, който не съдържа скоби
pub struct SimpleExprParser {
}

impl SimpleExprParser {
    pub fn new() -> SimpleExprParser {
        todo!()
    }

    pub fn push_atom(&mut self, c: char) -> Result<(), ParseError> {
        todo!()
    }

    pub fn push_op(&mut self, op: char) -> Result<(), ParseError> {
        todo!()
    }

    pub fn finish(self) -> Result<Expr, ParseError> {
        todo!()
    }
}

/// Парсър за пълния израз
pub struct ExprParser {

}

impl ExprParser {
    pub fn new() -> ExprParser {
        todo!();
    }

    /// Приема атом.
    ///
    /// `c` ще бъде валиден символ за атом.
    /// В противен случай можете да panic-нете (няма да се тества)
    pub fn push_atom(&mut self, c: char) -> Result<(), ParseError> {
        todo!()
    }

    /// Приема символ за операция.
    ///
    /// `op` ще бъде едно от '&', '|', '!'.
    /// В противен случай можете да panic-нете (няма да се тества)
    pub fn push_op(&mut self, op: char) -> Result<(), ParseError> {
        todo!()
    }

    /// Приема отваряща скоба.
    pub fn open_paren(&mut self) -> Result<(), ParseError> {
        todo!()
    }

    /// Приема затваряща скоба.
    pub fn close_paren(&mut self) -> Result<(), ParseError> {
        todo!()
    }

    /// Завършва парсването и връща построения израз.
    pub fn finish(self) -> Result<Expr, ParseError> {
        todo!()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    True,
    False,
    Expr(Expr),
}

pub fn eval(expr: &Expr, truthy: &[char], falsy: &[char]) -> Value {
    match expr {
        Expr::Atom(atom) => {
            if truthy.contains(atom) {
                Value::True
            } else if falsy.contains(atom) {
                Value::False
            } else {
                Value::Expr(expr.clone())
            }
        }
        Expr::Not(sub_expr) => match eval(sub_expr, truthy, falsy) {
            Value::True => Value::False,
            Value::False => Value::True,
            Value::Expr(inner_expr) => Value::Expr(Expr::Not(Box::new(inner_expr))),
        },
        Expr::And(sub_expr) => {
            let mut simplified = Vec::new();
            for sub_expr in sub_expr {
                match eval(sub_expr, truthy, falsy) {
                    Value::False => return Value::False, // Ако има False, целият And е False
                    Value::True => continue,             // Пропускаме True
                    Value::Expr(e) => simplified.push(e),
                }
            }
            if simplified.is_empty() {
                Value::True // Ако няма останали елементи, връщаме True
            } else if simplified.len() == 1 {
                eval(&simplified.pop().unwrap(), truthy, falsy) // Ако е останал само един елемент, опростяваме до него
            } else {
                Value::Expr(Expr::And(simplified))
            }
        }
        Expr::Or(sub_expr) => {
            let mut simplified = Vec::new();
            for sub_expr in sub_expr {
                match eval(sub_expr, truthy, falsy) {
                    Value::True => return Value::True, // Ако има True, целият Or е True
                    Value::False => continue,          // Пропускаме False
                    Value::Expr(e) => simplified.push(e),
                }
            }
            if simplified.is_empty() {
                Value::False // Ако няма останали елементи, връщаме False
            } else if simplified.len() == 1 {
                eval(&simplified.pop().unwrap(), truthy, falsy) // Ако е останал само един елемент, опростяваме до него
            } else {
                Value::Expr(Expr::Or(simplified))
            }
        }
    }
}

macro_rules! expr {
    (atom($c:expr)) => {
        Expr::Atom($c)
    };
    ( not( $tag:ident ( $($e:tt)* ) ) ) => {
        Expr::Not(Box::new(expr!( $tag($( $e )*) )))
    };
    ( and( $( $tag:ident ( $($e:tt)* ) ),* ) ) => {
        Expr::And(vec![$( expr!($tag($( $e )*)) ),*])
    };
    ( or( $( $tag:ident ( $($e:tt)* ) ),* ) ) => {
        Expr::Or(vec![$( expr!($tag($( $e )*)) ),*])
    };

    // ($op:ident($($e:tt),+)) => {
    //     Expr::$op(vec![$(expr!($e)),+])
    // };
}

fn main() {
    let expr1 = expr!(atom('A'));
    let expr2 = expr!(not(atom('A')));
    let expr3 = expr!(and(atom('A'), atom('B')));
    let expr4 = expr!(or(atom('A'), atom('B'), atom('C')));
    let expr5 = expr!(not(or(atom('A'), atom('B'))));
    let expr6 = expr!(and(not(atom('A')), atom('B')));

    println!("{:?}", expr1);
    println!("{:?}", expr2);
    println!("{:?}", expr3);
    println!("{:?}", expr4);
    println!("{:?}", expr5);
    println!("{:?}", expr6);
}

fn feed_simple(parser: &mut SimpleExprParser, text: &str) -> Result<(), ParseError> {
    for c in text.chars() {
        match c {
            ' ' => {}
            '&' | '|' | '!' => parser.push_op(c)?,
            _ => parser.push_atom(c)?,
        }
    }

    Ok(())
}
fn feed_full(parser: &mut ExprParser, text: &str) -> Result<(), ParseError> {
    for c in text.chars() {
        match c {
            ' ' => {}
            '&' | '|' | '!' => parser.push_op(c)?,
            '(' => parser.open_paren()?,
            ')' => parser.close_paren()?,
            _ => parser.push_atom(c)?,
        }
    }

    Ok(())
}

#[test]
fn test_simple_atom() {
    // "A"
    let mut parser = SimpleExprParser::new();
    parser.push_atom('A').unwrap();
    assert_eq!(parser.finish().unwrap(), expr!(atom('A')),);
}

#[test]
fn test_simple_and_or() {
    // "A & B"
    let mut parser = SimpleExprParser::new();
    parser.push_atom('A').unwrap();
    parser.push_op('&').unwrap();
    parser.push_atom('B').unwrap();
    assert_eq!(parser.finish().unwrap(), expr!(and(atom('A'), atom('B'))));

    // "A | B"
    let mut parser = SimpleExprParser::new();
    parser.push_atom('A').unwrap();
    parser.push_op('|').unwrap();
    parser.push_atom('B').unwrap();
    assert_eq!(parser.finish().unwrap(), expr!(or(atom('A'), atom('B'))));
}

#[test]
fn test_simple_not() {
    // "!B"
    let mut parser = SimpleExprParser::new();
    parser.push_op('!').unwrap();
    parser.push_atom('B').unwrap();
    assert_eq!(parser.finish().unwrap(), expr!(not(atom('B'))));
}

#[test]
fn test_simple_expr_and_not() {
    let mut parser = SimpleExprParser::new();
    feed_simple(&mut parser, "A & !B").unwrap();
    assert_eq!(parser.finish().unwrap(), expr!(and(atom('A'), not(atom('B')))));

    let mut parser = SimpleExprParser::new();
    feed_simple(&mut parser, "!A | B").unwrap();
    assert_eq!(parser.finish().unwrap(), expr!(or(not(atom('A')), atom('B'))));

    let mut parser = SimpleExprParser::new();
    feed_simple(&mut parser, "!A & !B").unwrap();
    assert_eq!(parser.finish().unwrap(), expr!(and(not(atom('A')), not(atom('B')))));
}

#[test]
fn test_simple_multiple_atoms_same_op() {
    let mut parser = SimpleExprParser::new();
    feed_simple(&mut parser, "A & B & C").unwrap();
    assert_eq!(parser.finish().unwrap(), expr!(and(atom('A'), atom('B'), atom('C'))));

    let mut parser = SimpleExprParser::new();
    feed_simple(&mut parser, "X | Y | Z | W").unwrap();
    assert_eq!(
        parser.finish().unwrap(),
        expr!(or(atom('X'), atom('Y'), atom('Z'), atom('W')))
    );
}

#[test]
fn test_simple_multiple_not() {
    let mut parser = SimpleExprParser::new();
    feed_simple(&mut parser, "!!!B").unwrap();
    assert_eq!(parser.finish().unwrap(), expr!(not(not(not(atom('B'))))));

    let mut parser = SimpleExprParser::new();
    feed_simple(&mut parser, "A | !!B").unwrap();
    assert_eq!(parser.finish().unwrap(), expr!(or(atom('A'), not(not(atom('B'))))));

    let mut parser = SimpleExprParser::new();
    feed_simple(&mut parser, "!!A | B").unwrap();
    assert_eq!(parser.finish().unwrap(), expr!(or(not(not(atom('A'))), atom('B'))));

    let mut parser = SimpleExprParser::new();
    feed_simple(&mut parser, "!!A & !B").unwrap();
    assert_eq!(
        parser.finish().unwrap(),
        expr!(and(not(not(atom('A'))), not(atom('B'))))
    );
}

#[test]
fn test_simple_alternating_ops() {
    let mut parser = SimpleExprParser::new();
    feed_simple(&mut parser, "A & B | C & D").unwrap();
    assert_eq!(
        parser.finish().unwrap(),
        expr!(and(or(and(atom('A'), atom('B')), atom('C')), atom('D')))
    );

    let mut parser = SimpleExprParser::new();
    feed_simple(&mut parser, "A | B & C | D").unwrap();
    assert_eq!(
        parser.finish().unwrap(),
        expr!(or(and(or(atom('A'), atom('B')), atom('C')), atom('D')))
    );

    let mut parser = SimpleExprParser::new();
    feed_simple(&mut parser, "A & B & C | D | E").unwrap();
    assert_eq!(
        parser.finish().unwrap(),
        expr!(or(and(atom('A'), atom('B'), atom('C')), atom('D'), atom('E')))
    );
}

#[test]
fn test_simple_errors() {
    let parser = SimpleExprParser::new();
    assert_eq!(parser.finish(), Err(ParseError::UnexpectedEnd));

    let mut parser = SimpleExprParser::new();
    assert_eq!(parser.push_op('&'), Err(ParseError::UnexpectedBinOp));

    let mut parser = SimpleExprParser::new();
    assert_eq!(parser.push_op('|'), Err(ParseError::UnexpectedBinOp));

    let mut parser = SimpleExprParser::new();
    parser.push_atom('A').unwrap();
    assert_eq!(parser.push_atom('B'), Err(ParseError::UnexpectedExpr));

    let mut parser = SimpleExprParser::new();
    parser.push_atom('A').unwrap();
    assert_eq!(parser.push_op('!'), Err(ParseError::UnexpectedUnaryOp));

    let mut parser = SimpleExprParser::new();
    parser.push_op('!').unwrap();
    parser.push_atom('A').unwrap();
    assert_eq!(parser.push_atom('B'), Err(ParseError::UnexpectedExpr));

    let mut parser = SimpleExprParser::new();
    feed_simple(&mut parser, "A &").unwrap();
    assert_eq!(parser.finish(), Err(ParseError::UnexpectedEnd));

    let mut parser = SimpleExprParser::new();
    feed_simple(&mut parser, "A &").unwrap();
    assert_eq!(parser.push_op('&'), Err(ParseError::UnexpectedBinOp));

    let mut parser = SimpleExprParser::new();
    feed_simple(&mut parser, "A &").unwrap();
    assert_eq!(parser.push_op('|'), Err(ParseError::UnexpectedBinOp));

    let mut parser = SimpleExprParser::new();
    feed_simple(&mut parser, "A |").unwrap();
    assert_eq!(parser.finish(), Err(ParseError::UnexpectedEnd));

    let mut parser = SimpleExprParser::new();
    feed_simple(&mut parser, "A |").unwrap();
    assert_eq!(parser.push_op('&'), Err(ParseError::UnexpectedBinOp));

    let mut parser = SimpleExprParser::new();
    feed_simple(&mut parser, "A |").unwrap();
    assert_eq!(parser.push_op('|'), Err(ParseError::UnexpectedBinOp));
}

#[test]
fn test_full_basic_expr_bracketed() {
    let mut parser = ExprParser::new();
    feed_full(&mut parser, "(A)").unwrap();
    assert_eq!(parser.finish().unwrap(), expr!(atom('A')));

    let mut parser = ExprParser::new();
    feed_full(&mut parser, "(A & B)").unwrap();
    assert_eq!(parser.finish().unwrap(), expr!(and(atom('A'), atom('B'))));

    let mut parser = ExprParser::new();
    feed_full(&mut parser, "(A | B)").unwrap();
    assert_eq!(parser.finish().unwrap(), expr!(or(atom('A'), atom('B'))));

    let mut parser = ExprParser::new();
    feed_full(&mut parser, "(!A)").unwrap();
    assert_eq!(parser.finish().unwrap(), expr!(not(atom('A'))));
}

#[test]
fn test_full_one_bracketed() {
    let mut parser = ExprParser::new();
    feed_full(&mut parser, "X | (A & B)").unwrap();
    assert_eq!(
        parser.finish().unwrap(),
        expr!(or(atom('X'), and(atom('A'), atom('B'))))
    );

    let mut parser = ExprParser::new();
    feed_full(&mut parser, "(A | B) & X").unwrap();
    assert_eq!(
        parser.finish().unwrap(),
        expr!(and(or(atom('A'), atom('B')), atom('X')))
    );
}

#[test]
fn test_full_not_paren() {
    let mut parser = ExprParser::new();
    feed_full(&mut parser, "X & !(A | B)").unwrap();
    assert_eq!(
        parser.finish().unwrap(),
        expr!(and(atom('X'), not(or(atom('A'), atom('B')))))
    );

    let mut parser = ExprParser::new();
    feed_full(&mut parser, "!!(A | B) | X").unwrap();
    assert_eq!(
        parser.finish().unwrap(),
        expr!(or(not(not(or(atom('A'), atom('B')))), atom('X')))
    );
}

// #[test]
// fn test_full_same_op_outside_bracket_inside_bracket() {
//     // Не е специфицирано в условието какво се очаква, затова и двата варианта са допустими

//     let mut parser = ExprParser::new();
//     feed_full(&mut parser, "X & (A & B)").unwrap();
//     let expr = parser.finish().unwrap();
//     assert!(
//         expr == expr!(and(atom('X'), atom('A'), atom('B'))) || expr == expr!(and(atom('X'), and(atom('A'), atom('B'))))
//     );

//     let mut parser = ExprParser::new();
//     feed_full(&mut parser, "(A & B) & X").unwrap();
//     let expr = parser.finish().unwrap();
//     assert!(
//         expr == expr!(and(atom('A'), atom('B'), atom('X'))) || expr == expr!(and(and(atom('A'), atom('B')), atom('X')))
//     );
// }

#[test]
fn test_full_surrounded() {
    let mut parser = ExprParser::new();
    feed_full(&mut parser, "X | (A & B) | (C & D) | Y").unwrap();
    assert_eq!(
        parser.finish().unwrap(),
        expr!(or(
            atom('X'),
            and(atom('A'), atom('B')),
            and(atom('C'), atom('D')),
            atom('Y')
        ))
    );
}

#[test]
fn test_full_nested_parenthesis() {
    let mut parser = ExprParser::new();
    feed_full(&mut parser, "!(A & !(B & !(C & D)))").unwrap();
    assert_eq!(
        parser.finish().unwrap(),
        expr!(not(and(atom('A'), not(and(atom('B'), not(and(atom('C'), atom('D'))))))))
    );
}

#[test]
fn test_full_error_1() {
    let mut parser = ExprParser::new();
    assert_eq!(parser.close_paren(), Err(ParseError::UnexpectedParen));

    let mut parser = ExprParser::new();
    parser.open_paren().unwrap();
    assert_eq!(parser.close_paren(), Err(ParseError::UnexpectedParen));

    let mut parser = ExprParser::new();
    parser.open_paren().unwrap();
    parser.push_atom('A').unwrap();
    parser.push_op('&').unwrap();
    assert_eq!(parser.close_paren(), Err(ParseError::UnexpectedParen));

    let mut parser = ExprParser::new();
    parser.open_paren().unwrap();
    parser.push_op('!').unwrap();
    assert_eq!(parser.close_paren(), Err(ParseError::UnexpectedParen));

    let mut parser = ExprParser::new();
    parser.open_paren().unwrap();
    assert_eq!(parser.finish(), Err(ParseError::UnexpectedEnd));

    let mut parser = ExprParser::new();
    parser.open_paren().unwrap();
    parser.push_atom('A').unwrap();
    assert_eq!(parser.finish(), Err(ParseError::UnexpectedEnd));
}

#[test]
fn test_full_error_2() {
    let mut parser = ExprParser::new();
    parser.push_atom('A').unwrap();
    assert!(matches!(
        parser.open_paren(),
        Err(ParseError::UnexpectedExpr | ParseError::UnexpectedParen)
    ));

    let mut parser = ExprParser::new();
    feed_full(&mut parser, "(A & B)").unwrap();
    assert!(matches!(
        parser.open_paren(),
        Err(ParseError::UnexpectedExpr | ParseError::UnexpectedParen)
    ));
}

#[test]
fn test_eval_full() {
    assert_eq!(eval(&expr!(atom('A')), &['A'], &[]), Value::True);
    assert_eq!(eval(&expr!(atom('A')), &[], &['A']), Value::False);

    assert_eq!(eval(&expr!(not(atom('B'))), &['A'], &['B']), Value::True);
    assert_eq!(eval(&expr!(not(atom('B'))), &['B'], &['A']), Value::False);

    assert_eq!(eval(&expr!(and(atom('A'), atom('B'))), &['A', 'B'], &[]), Value::True);
    assert_eq!(eval(&expr!(and(atom('A'), atom('B'))), &['A'], &['B']), Value::False);
    assert_eq!(eval(&expr!(or(atom('A'), atom('B'))), &['A'], &['B']), Value::True);
    assert_eq!(eval(&expr!(or(atom('A'), atom('B'))), &[], &['A', 'B']), Value::False);
}

#[test]
fn test_eval_not_and_or() {
    assert_eq!(
        eval(&expr!(not(and(atom('A'), atom('B')))), &['A', 'B'], &[]),
        Value::False
    );
    assert_eq!(
        eval(&expr!(not(and(atom('A'), atom('B')))), &['A'], &['B']),
        Value::True
    );
    assert_eq!(
        eval(&expr!(not(or(atom('A'), atom('B')))), &['A'], &['B']),
        Value::False
    );
    assert_eq!(
        eval(&expr!(not(or(atom('A'), atom('B')))), &[], &['A', 'B']),
        Value::True
    );
}

#[test]
fn test_eval_partial() {
    assert_eq!(eval(&expr!(atom('A')), &[], &[]), Value::Expr(expr!(atom('A'))));
    assert_eq!(
        eval(&expr!(not(atom('B'))), &[], &[]),
        Value::Expr(expr!(not(atom('B'))))
    );

    assert_eq!(
        eval(&expr!(and(atom('A'), atom('B'), atom('C'))), &['B'], &[]),
        Value::Expr(expr!(and(atom('A'), atom('C'))))
    );
    assert_eq!(
        eval(&expr!(and(atom('A'), atom('B'), atom('C'))), &[], &['B']),
        Value::False
    );

    assert_eq!(
        eval(&expr!(or(atom('A'), atom('B'), atom('C'))), &['B'], &[]),
        Value::True
    );
    assert_eq!(
        eval(&expr!(or(atom('A'), atom('B'), atom('C'))), &[], &['B']),
        Value::Expr(expr!(or(atom('A'), atom('C'))))
    );

    assert_eq!(
        eval(&expr!(and(atom('A'), not(atom('B')), atom('C'))), &[], &['B']),
        Value::Expr(expr!(and(atom('A'), atom('C'))))
    );
    assert_eq!(
        eval(&expr!(and(atom('A'), not(atom('B')), atom('C'))), &['B'], &[]),
        Value::False
    );

    assert_eq!(
        eval(&expr!(or(atom('A'), not(atom('B')), atom('C'))), &[], &['B']),
        Value::True
    );
    assert_eq!(
        eval(&expr!(or(atom('A'), not(atom('B')), atom('C'))), &['B'], &[]),
        Value::Expr(expr!(or(atom('A'), atom('C'))))
    );
}

#[test]
fn test_eval_unwrap_and_or() {
    assert_eq!(
        eval(&expr!(and(atom('A'), atom('B'), atom('C'))), &['A', 'C'], &[]),
        Value::Expr(expr!(atom('B')))
    );
    assert_eq!(
        eval(&expr!(or(atom('A'), atom('B'), atom('C'))), &[], &['A', 'C']),
        Value::Expr(expr!(atom('B')))
    );

    assert_eq!(
        eval(&expr!(and(atom('A'), not(atom('B')), atom('C'))), &['A', 'C'], &[]),
        Value::Expr(expr!(not(atom('B'))))
    );
    assert_eq!(
        eval(&expr!(or(atom('A'), not(atom('B')), atom('C'))), &[], &['A', 'C']),
        Value::Expr(expr!(not(atom('B'))))
    );
}

#[test]
fn test_eval_unwrap_nested() {
    assert_eq!(
        eval(
            &expr!(or(
                atom('X'),
                and(atom('A'), atom('B')),
                not(and(atom('C'), atom('D'))),
                atom('Y')
            )),
            &['A', 'C'],
            &[]
        ),
        Value::Expr(expr!(or(atom('X'), atom('B'), not(atom('D')), atom('Y'))))
    );
}

// #[test]
// fn test_eval_no_unwrap_parenthesis() {
//     assert_eq!(
//         eval(&expr!(not(and(atom('X'), atom('Y')))), &[], &[]),
//         Value::Expr(expr!(not(and(atom('X'), atom('Y')))))
//     );
// }


